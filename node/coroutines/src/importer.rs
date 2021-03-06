//! BeaconBlockImporter consumes blocks that we received from other peers and adds them to the
//! chain.
use std::sync::Arc;

use futures::sync::mpsc::Receiver;
use futures::{future, Stream};

use client::Client;
use primitives::beacon::SignedBeaconBlock;
use primitives::chain::SignedShardBlock;

pub fn spawn_block_importer(
    client: Arc<Client>,
    incoming_block_tx: Receiver<(SignedBeaconBlock, SignedShardBlock)>,
) {
    let task = incoming_block_tx.for_each(move |(beacon_block, shard_block)| {
        client.try_import_blocks(beacon_block, shard_block);
        future::ok(())
    });
    tokio::spawn(task);
}
