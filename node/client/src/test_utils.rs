use std::collections::HashMap;

use crate::Client;
use beacon::beacon_chain::BeaconClient;
use configs::ChainSpec;
use primitives::beacon::SignedBeaconBlock;
use primitives::signer::InMemorySigner;
use shard::ShardClient;
use std::sync::RwLock;
use storage::test_utils::create_beacon_shard_storages;

/// Implements dummy client for testing. The differences with the real client:
/// * It does not do the correct signing;
/// * It has in-memory storage.
pub fn get_client_from_cfg(chain_spec: &ChainSpec, signer: InMemorySigner) -> Client {
    let (beacon_storage, shard_storage) = create_beacon_shard_storages();
    let shard_client = ShardClient::new(chain_spec, shard_storage);
    let genesis = SignedBeaconBlock::genesis(shard_client.genesis_hash());
    let beacon_chain = BeaconClient::new(genesis, chain_spec, beacon_storage);
    Client {
        account_id: signer.account_id.clone(),
        signer,
        shard_client,
        beacon_chain,
        pending_beacon_blocks: RwLock::new(HashMap::new()),
        pending_shard_blocks: RwLock::new(HashMap::new()),
    }
}
