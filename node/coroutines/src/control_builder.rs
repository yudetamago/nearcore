//! Constructs control for TxFlow using the current Client state.
use client::Client;
use nightshade::nightshade_task::{Control, ConsensusPariticipants};
use primitives::signature::get_key_pair;
use primitives::aggregate_signature::BlsPublicKey;
use primitives::aggregate_signature::BlsSecretKey;
use primitives::chain::ChainPayload;

// TODO remove
fn get_bls_key_pair() -> (BlsPublicKey, BlsSecretKey) {
    let secret_key = BlsSecretKey::generate();
    let public_key = secret_key.get_public_key();
    (public_key, secret_key)
}

pub fn get_control(client: &Client, block_index: u64) -> Control<ChainPayload> {
    let (owner_uid, uid_to_authority_map) =
        client.get_uid_to_authority_map(block_index);
    let num_authorities = uid_to_authority_map.len();
    let (public_keys, secret_keys): (Vec<_>, Vec<_>) = (0..num_authorities).map(|_| get_key_pair()).unzip();
    let (bls_public_keys, bls_secret_keys): (Vec<_>, Vec<_>) = (0..num_authorities).map(|_| get_bls_key_pair()).unzip();
    let owner_id = 0;
    let block = ChainPayload{
        transactions: vec![],
        receipts: vec![]
    };
    let witnesses = (0 as u64 .. num_authorities as u64).collect();
    // TODO get keys from the client
    match owner_uid {
        None => Control::Stop,
        Some(owner_uid) => {
            Control::Reset((ConsensusPariticipants::new(
                owner_uid,
                witnesses,
                public_keys.clone(),
                secret_keys[owner_id].clone(),
                bls_public_keys.clone(),
                bls_secret_keys[owner_id].clone(),
            ), block))
        }
    }
}
