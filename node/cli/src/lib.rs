extern crate beacon;
extern crate beacon_chain_handler;
extern crate chain;
extern crate clap;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate network;
extern crate node_rpc;
extern crate node_runtime;
extern crate parking_lot;
extern crate primitives;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate shard;
extern crate storage;
extern crate tokio;

use std::fs;
use std::path::Path;
use std::sync::Arc;
use primitives::signature::DEFAULT_SIGNATURE;

use clap::{App, Arg};
use env_logger::Builder;
use futures::{Future, future, Stream, Sink, done};
use futures::sync::mpsc::channel;
use std::sync::mpsc::channel as sync_channel;
use futures::sync::mpsc::Receiver;
use futures::sync::mpsc::Sender;
use parking_lot::{Mutex, RwLock};
use std::sync::mpsc::Receiver as SyncReceiver;

use beacon::types::{SignedBeaconBlock, BeaconBlockChain, SignedBeaconBlockHeader};
use chain::SignedBlock;
use network::protocol::{Protocol, ProtocolConfig};
use network::service::{create_network_task, NetworkConfiguration, new_network_service};
use node_rpc::api::RpcImpl;
use node_runtime::{Runtime, state_viewer::StateDbViewer};
use primitives::signer::InMemorySigner;
use primitives::types::{SignedTransaction, SignedMessageData, MessageDataBody};
use beacon_chain_handler::producer::{ShardChainPayload, ChainConsensusBlockBody};
use shard::{SignedShardBlock, ShardBlockChain};
use storage::{StateDb, Storage};
use std::collections::HashSet;

pub mod chain_spec;
pub mod test_utils;


fn get_storage(base_path: &Path) -> Arc<Storage> {
    let mut storage_path = base_path.to_owned();
    storage_path.push("storage/db");
    match fs::canonicalize(storage_path.clone()) {
        Ok(path) => info!("Opening storage database at {:?}", path),
        _ => info!("Could not resolve {:?} path", storage_path),
    };
    Arc::new(storage::open_database(&storage_path.to_string_lossy()))
}

pub fn start_service(
    base_path: &Path,
    chain_spec_path: Option<&Path>,
//    consensus_task_fn: &Fn(
//        Mutex<SyncReceiver<SignedTransaction>>,
//        &Sender<ChainConsensusBlockBody>) -> Box<Future<Item=(), Error=()> + Send>,
) {
    let mut builder = Builder::new();
    builder.filter(Some("runtime"), log::LevelFilter::Debug);
    builder.filter(None, log::LevelFilter::Info);
    builder.init();

    // Create shared-state objects.
    let storage = get_storage(base_path);
    let chain_spec = chain_spec::read_or_default_chain_spec(&chain_spec_path);

    let state_db = Arc::new(StateDb::new(storage.clone()));
    let runtime = Arc::new(RwLock::new(Runtime::new(state_db.clone())));
    let genesis_root = runtime.write().apply_genesis_state(
        &chain_spec.accounts,
        &chain_spec.genesis_wasm,
        &chain_spec.initial_authorities,
    );

    let shard_genesis = SignedShardBlock::genesis(genesis_root);
    let genesis = SignedBeaconBlock::genesis(shard_genesis.block_hash());
    let shard_chain = Arc::new(ShardBlockChain::new(shard_genesis, storage.clone()));
    let beacon_chain = Arc::new(BeaconBlockChain::new(genesis, storage.clone()));

    // Create RPC Server.
    let state_db_viewer = StateDbViewer::new(shard_chain.clone(), state_db.clone());
    // TODO: TxFlow should be listening on these transactions.
    let (transactions_tx, transactions_rx) = sync_channel();
    let (async_transactions_tx, _transactions_rx) = channel(1024);
    let (receipts_tx, _receipts_rx) = channel(1024);
    let rpc_impl = RpcImpl::new(state_db_viewer, transactions_tx.clone());
    let rpc_handler = node_rpc::api::get_handler(rpc_impl);
    let server = node_rpc::server::get_server(rpc_handler);

    // Create a task that consumes the consensuses and produces the beacon chain blocks.
    let signer = Arc::new(InMemorySigner::default());
    let (
        beacon_block_consensus_body_tx,
        beacon_block_consensus_body_rx,
    ) = channel(1024);
    let block_producer_task = beacon_chain_handler::producer::create_block_producer_task(
        beacon_chain.clone(),
        shard_chain.clone(),
        runtime.clone(),
        signer.clone(),
        state_db.clone(),
        beacon_block_consensus_body_rx,
    );

    // Create task that can import beacon chain blocks from other peers.
    let (beacon_block_tx, beacon_block_rx) = channel(1024);
    let block_importer_task = beacon_chain_handler::importer::create_block_importer_task(
        beacon_chain.clone(),
        shard_chain.clone(),
        runtime.clone(),
        state_db.clone(),
        beacon_block_rx
    );

    // Create protocol and the network_task.
    // Note, that network and RPC are using the same channels to send transactions and receipts for
    // processing.
    let (net_messages_tx, net_messages_rx) = channel(1024);
    let protocol_config = ProtocolConfig::default();
    let protocol = Protocol::<_, SignedBeaconBlockHeader>::new(
        protocol_config,
        beacon_chain.clone(),
        beacon_block_tx.clone(),
        async_transactions_tx.clone(),
        receipts_tx.clone(),
        net_messages_tx.clone()
    );
    let network_service = Arc::new(Mutex::new(new_network_service(
        &protocol_config,
        NetworkConfiguration::default(),
    )));
    let (network_task, messages_handler_task) = create_network_task(
        network_service,
        protocol,
        net_messages_rx,
    );

    let transactions_rx = Mutex::new(transactions_rx);
    let consensus_task = done::<(), ()>(Ok(())).map(move |_| {
        loop {
            if let Ok(t) = transactions_rx.lock().recv() {
                let message: SignedMessageData<ShardChainPayload> = SignedMessageData {
                    owner_sig: DEFAULT_SIGNATURE,  // TODO: Sign it.
                    hash: 0,  // Compute real hash
                    body: MessageDataBody {
                        owner_uid: 0,
                        parents: HashSet::new(),
                        epoch: 0,
                        payload: (vec![t], vec![]),
                        endorsements: vec![],
                    },
                };
                let c = ChainConsensusBlockBody {
                    messages: vec![message],
                };
                tokio::spawn(beacon_block_consensus_body_tx.clone().send(c).map(|_| ()).map_err(|e| {
                    error!("Failure sending pass-through consensus {:?}", e);
                }));
            } else {
                break;
            }
        }
    });

    tokio::run(future::lazy(|| {
        tokio::spawn(future::lazy(|| {
            server.wait();
            Ok(())
        }));
        tokio::spawn(block_producer_task);
        tokio::spawn(block_importer_task);
        tokio::spawn(messages_handler_task);
        tokio::spawn(network_task);
        tokio::spawn(consensus_task);
        Ok(())
    }));
}

pub fn run() {
    let matches = App::new("near")
        .arg(
            Arg::with_name("base_path")
                .short("b")
                .long("base-path")
                .value_name("PATH")
                .help("Sets a base path for persisted files")
                .takes_value(true),
        ).arg(
            Arg::with_name("chain_spec_file")
                .short("c")
                .long("chain-spec-file")
                .value_name("CHAIN_SPEC_FILE")
                .help("Sets a file location to read a custom chain spec")
                .takes_value(true),
        ).get_matches();

    let base_path =
        matches.value_of("base_path").map(|x| Path::new(x)).unwrap_or_else(|| Path::new("."));

    let chain_spec_path = matches.value_of("chain_spec_file").map(|x| Path::new(x));

    start_service(
        base_path,
        chain_spec_path,
        //&test_utils::create_passthrough_beacon_block_consensus_task,
    );
}
