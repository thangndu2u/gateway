use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub version: String,
    pub throughput: f64,
    pub jitter: u128,
    pub packet_loss: f64,
    pub connected_at: u64,
    pub updated_at: u64,
    pub total_test_performed: u32,
    pub peer_ip_v4: String,
}

#[derive(Serialize, Deserialize)]
pub struct PeerInfoRespond {
    pub version: String,
    pub peer_ip_v4: String,
    pub throughput: String,
    pub jitter: String,
    pub packet_loss: String,
    pub connected_at: u64,
    pub updated_at: u64,
    pub total_test_performed: u32,
}
