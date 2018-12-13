use beacon_chain_handler::producer::{ShardChainPayload, ChainConsensusBlockBody};
use primitives::types::{SignedTransaction, SignedMessageData, MessageDataBody};
use primitives::signature::DEFAULT_SIGNATURE;
use futures::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use futures::{Future, future, Stream, Sink, done};
use std::collections::HashSet;
use tokio;
use std::sync::Mutex;


//pub fn create_passthrough_beacon_block_consensus_task(
//    transactions_rx: Mutex<Receiver<SignedTransaction>>,
//    consensus_tx: &Sender<ChainConsensusBlockBody>,
//) -> Box<Future<Item=(), Error=()> + Send> {
//
//}
