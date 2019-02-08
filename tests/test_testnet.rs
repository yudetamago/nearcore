use std::net::SocketAddr;
use std::panic;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use configs::chain_spec::read_or_default_chain_spec;
use configs::ClientConfig;
use configs::NetworkConfig;
use configs::RPCConfig;
use node_http::types::SignedBeaconBlockResponse;
use node_http::types::SignedShardBlockResponse;
use primitives::hash::hash_struct;
use primitives::network::PeerInfo;
use primitives::signer::write_key_file;
use primitives::test_utils::get_key_pair_from_seed;

fn test_node_ready(
    base_path: PathBuf,
    node_info: PeerInfo,
    rpc_port: u16,
    boot_nodes: Vec<PeerInfo>,
) {
    if base_path.exists() {
        std::fs::remove_dir_all(base_path.clone()).unwrap();
    }

    let client_cfg = ClientConfig {
        base_path,
        account_id: node_info.account_id.unwrap(),
        public_key: None,
        chain_spec: read_or_default_chain_spec(&Some(PathBuf::from(
            "./node/configs/res/testnet_chain.json",
        ))),
        log_level: log::LevelFilter::Off,
    };

    let network_cfg = NetworkConfig {
        listen_addr: node_info.addr,
        peer_id: node_info.id,
        boot_nodes,
        reconnect_delay: Duration::from_millis(50),
        gossip_interval: Duration::from_millis(50),
        gossip_sample_size: 10,
    };

    let rpc_cfg = RPCConfig { rpc_port };
    thread::spawn(|| {
        testnet::start_from_configs(client_cfg, network_cfg, rpc_cfg);
    });
    thread::sleep(Duration::from_secs(1));
}

fn check_result(output: Output) -> Result<String, String> {
    let mut result = String::from_utf8_lossy(output.stdout.as_slice());
    if !output.status.success() {
        if result.is_empty() {
            result = String::from_utf8_lossy(output.stderr.as_slice());
        }
        return Err(result.to_owned().to_string());
    }
    Ok(result.to_owned().to_string())
}

const TMP_DIR: &str = "./tmp/testnet";
const KEY_STORE_PATH: &str = "./tmp/testnet/key_store";

fn get_public_key(participant: &str) -> String {
    let key_store_path = Path::new(KEY_STORE_PATH);
    let (public_key, secret_key) = get_key_pair_from_seed(&participant);
    write_key_file(key_store_path, public_key, secret_key)
}

fn setup_network() {
    // Setup network by launching two nodes.
    // alice_node (boot node) listening at 127.0.0.1:3000 (3030 RPC Port)
    // bob_node listening at 127.0.0.1:3001 (3031 RPC Port)

    // Start boot node.
    let mut base_path = PathBuf::from(TMP_DIR);
    base_path.push("node_alice");
    let alice_info = PeerInfo {
        account_id: Some(String::from("alice.near")),
        id: hash_struct(&1),
        addr: SocketAddr::from_str("127.0.0.1:3000").unwrap(),
    };
    test_node_ready(base_path, alice_info.clone(), 3030, vec![]);

    // Start secondary node that boots from the alice node.
    let mut base_path = PathBuf::from(TMP_DIR);
    base_path.push("node_bob");
    let bob_info = PeerInfo {
        account_id: Some(String::from("bob.near")),
        id: hash_struct(&2),
        addr: SocketAddr::from_str("127.0.0.1:3001").unwrap(),
    };
    test_node_ready(base_path, bob_info.clone(), 3031, vec![alice_info]);
}

fn create_account(name: &str) {
    // Create an account on alice node.
    Command::new("./scripts/rpc.py")
        .arg("create_account")
        .arg(name)
        .arg("1")
        .arg("-d")
        .arg(KEY_STORE_PATH)
        .arg("-k")
        .arg(get_public_key("alice.near"))
        .arg("-u")
        .arg("http://127.0.0.1:3030/")
        .output()
        .expect("create_account command failed to process");

    // Wait until this account is present on the bob.near node.
    let view_account = || -> bool {
        let res = Command::new("./scripts/rpc.py")
            .arg("view_account")
            .arg("-a")
            .arg(name)
            .arg("-u")
            .arg("http://127.0.0.1:3031/")
            .output()
            .expect("view_account command failed to process");
        check_result(res).is_ok()
    };
    wait(view_account, 500, 60000);
}

fn start_testnet() {
    setup_network();
    create_account("jason");
}

fn get_latest_beacon_block() -> SignedBeaconBlockResponse {
    let output = Command::new("./scripts/rpc.py")
        .arg("view_latest_beacon_block")
        .output()
        .expect("view_latest_shard_block command failed to process");
    let result = check_result(output).unwrap();
    serde_json::from_str(&result).unwrap()
}

fn get_latest_shard_block() -> SignedShardBlockResponse {
    let output = Command::new("./scripts/rpc.py")
        .arg("view_latest_shard_block")
        .output()
        .expect("view_latest_shard_block command failed to process");
    let result = check_result(output).unwrap();
    serde_json::from_str(&result).unwrap()
}

fn send_money(sender: &str, receiver: &str, amount: i32) {
    let output = Command::new("./scripts/rpc.py")
        .arg("send_money")
        .arg("-d")
        .arg(KEY_STORE_PATH)
        .arg("-k")
        .arg(get_public_key(sender))
        .arg("--sender")
        .arg(&sender)
        .arg("--receiver")
        .arg(&receiver)
        .arg("--amount")
        .arg(amount.to_string())
        .output()
        .expect("send_money command failed to process");

    let _ = check_result(output).unwrap();
}

#[test]
fn test_two_nodes() {
    start_testnet();
}

#[test]

#[test]
fn test_producing_one_block() {
    setup_network();

    let beacon_block_0 = get_latest_beacon_block();
    let shard_block_0 = get_latest_shard_block();

    let bb0_hash = serde_json::to_string(&beacon_block_0).unwrap();
    let sb0_hash = serde_json::to_string(&shard_block_0).unwrap();

    create_account("jason");

    let beacon_block_1 = get_latest_beacon_block();
    let shard_block_1 = get_latest_shard_block();

    let bb1_hash = serde_json::to_string(&beacon_block_1).unwrap();
    let sb1_hash = serde_json::to_string(&shard_block_1).unwrap();

    assert_ne!(bb0_hash, bb1_hash);
    assert_ne!(sb0_hash, sb1_hash);
}

#[test]
#[should_panic]
fn test_send_money_block_produced(){
    setup_network();

    let beacon_block_0 = get_latest_beacon_block();
    let shard_block_0 = get_latest_shard_block();

    let bb0_hash = serde_json::to_string(&beacon_block_0).unwrap();
    let sb0_hash = serde_json::to_string(&shard_block_0).unwrap();

    send_money("alice.near", "bob.near", 1);

    let beacon_block_1 = get_latest_beacon_block();
    let shard_block_1 = get_latest_shard_block();

    let bb1_hash = serde_json::to_string(&beacon_block_1).unwrap();
    let sb1_hash = serde_json::to_string(&shard_block_1).unwrap();

    assert_ne!(bb0_hash, bb1_hash);
    assert_ne!(sb0_hash, sb1_hash);
}

fn test_balance_after_send_money() {
    setup_network();

    send_money("alice.near", "bob.near", 1);
}

#[test]
#[should_panic]
fn test_producing_two_blocks() {
    setup_network();

    let beacon_block_0 = get_latest_beacon_block();
    let shard_block_0 = get_latest_shard_block();

    let bb0_hash = serde_json::to_string(&beacon_block_0).unwrap();
    let sb0_hash = serde_json::to_string(&shard_block_0).unwrap();

    create_account("jason");

    let beacon_block_1 = get_latest_beacon_block();
    let shard_block_1 = get_latest_shard_block();

    let bb1_hash = serde_json::to_string(&beacon_block_1).unwrap();
    let sb1_hash = serde_json::to_string(&shard_block_1).unwrap();

    assert_ne!(bb0_hash, bb1_hash, "First beacon block was not produced");
    assert_ne!(sb0_hash, sb1_hash, "First shard block was not produced");

    create_account("eve");

    let beacon_block_2 = get_latest_beacon_block();
    let shard_block_2 = get_latest_shard_block();

    let bb2_hash = serde_json::to_string(&beacon_block_2).unwrap();
    let sb2_hash = serde_json::to_string(&shard_block_2).unwrap();

    assert_ne!(bb1_hash, bb2_hash, "Second beacon block was not produced");
    assert_ne!(sb1_hash, sb2_hash, "Second shard block was not produced");
}

fn wait<F>(f: F, check_interval_ms: u64, max_wait_ms: u64)
    where
        F: Fn() -> bool,
{
    let mut ms_slept = 0;
    while !f() {
        thread::sleep(Duration::from_millis(check_interval_ms));
        ms_slept += check_interval_ms;
        if ms_slept > max_wait_ms {
            panic!("Timed out waiting for the condition");
        }
    }
}
