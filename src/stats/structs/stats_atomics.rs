use std::sync::atomic::{AtomicBool, AtomicI64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsAtomics {
    pub started: AtomicI64,
    pub timestamp_run_save: AtomicI64,
    pub timestamp_run_timeout: AtomicI64,
    pub timestamp_run_console: AtomicI64,
    pub timestamp_run_keys_timeout: AtomicI64,
    pub torrents: AtomicI64,
    pub torrents_updates: AtomicI64,
    pub users: AtomicI64,
    pub users_updates: AtomicI64,
    pub seeds: AtomicI64,
    pub peers: AtomicI64,
    pub completed: AtomicI64,
    pub whitelist_enabled: AtomicBool,
    pub whitelist: AtomicI64,
    pub whitelist_updates: AtomicI64,
    pub blacklist_enabled: AtomicBool,
    pub blacklist: AtomicI64,
    pub blacklist_updates: AtomicI64,
    pub keys_enabled: AtomicBool,
    pub keys: AtomicI64,
    pub keys_updates: AtomicI64,
    pub tcp4_not_found: AtomicI64,
    pub tcp4_failure: AtomicI64,
    pub tcp4_connections_handled: AtomicI64,
    pub tcp4_api_handled: AtomicI64,
    pub tcp4_announces_handled: AtomicI64,
    pub tcp4_scrapes_handled: AtomicI64,
    pub tcp6_not_found: AtomicI64,
    pub tcp6_failure: AtomicI64,
    pub tcp6_connections_handled: AtomicI64,
    pub tcp6_api_handled: AtomicI64,
    pub tcp6_announces_handled: AtomicI64,
    pub tcp6_scrapes_handled: AtomicI64,
    pub udp4_bad_request: AtomicI64,
    pub udp4_invalid_request: AtomicI64,
    pub udp4_connections_handled: AtomicI64,
    pub udp4_announces_handled: AtomicI64,
    pub udp4_scrapes_handled: AtomicI64,
    pub udp6_bad_request: AtomicI64,
    pub udp6_invalid_request: AtomicI64,
    pub udp6_connections_handled: AtomicI64,
    pub udp6_announces_handled: AtomicI64,
    pub udp6_scrapes_handled: AtomicI64,
}