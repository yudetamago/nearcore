use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

use crate::peer_manager::PeerManager;
use crate::peer::PeerState;
use std::ops::Deref;

const POISONED_LOCK_ERR: &str = "The lock was poisoned.";

impl PeerManager {
    pub fn count_ready_channels(&self) -> usize {
        self.all_peer_states
            .read()
            .expect(POISONED_LOCK_ERR)
            .values()
            .filter_map(|state| match state.read().expect(POISONED_LOCK_ERR).deref() {
                PeerState::Ready { .. } => Some(1 as usize),
                _ => None,
            })
            .sum()
    }
}

pub fn wait_all_peers_connected(
    check_interval_ms: u64,
    max_wait_ms: u64,
    peer_managers: &Arc<RwLock<Vec<PeerManager>>>,
    expected_num_managers: usize,
) {
    wait(
        || {
            let guard = peer_managers.read().expect(POISONED_LOCK_ERR);
            guard.len() == expected_num_managers
                && guard.iter().all(|pm| pm.count_ready_channels() == expected_num_managers - 1)
        },
        check_interval_ms,
        max_wait_ms,
    );
}

pub fn wait<F>(f: F, check_interval_ms: u64, max_wait_ms: u64)
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
