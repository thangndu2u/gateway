use crate::APP_CONFIG;
use crate::model::{
    partner::PartnerConfigCondition,
    peer::{PeerInfo, PeerInfoRespond},
};
use anyhow::Result;
use async_trait::async_trait;
use futures::future::join_all;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task;

#[async_trait]
pub trait PartnerService: Send + Sync {
    async fn fetch_partners_info(self: Arc<Self>) -> Result<()>;
    async fn filter_partners_info(&self, config: PartnerConfigCondition) -> Vec<PeerInfoRespond>;
}

pub struct PartnerServiceImpl {
    providers: RwLock<Vec<PeerInfo>>,
}

#[async_trait]
impl PartnerService for PartnerServiceImpl {
    async fn fetch_partners_info(self: Arc<Self>) -> Result<()> {
        let masternode_ips = &APP_CONFIG.masternode_ips;
        let client = reqwest::Client::new();
        let this = self.clone();

        let tasks: Vec<_> = masternode_ips
            .iter()
            .map(|ip| {
                let client = client.clone();
                let ip = ip.clone();
                task::spawn(async move {
                    let url = format!("{}:9093/peers", ip);
                    match client
                        .get(&url)
                        //API timeout
                        .timeout(Duration::from_secs(5))
                        .send()
                        .await
                    {
                        Ok(res) => {
                            if res.status().is_success() {
                                match res.json::<PeerInfo>().await {
                                    Ok(peer_data) => Some(peer_data),
                                    Err(_) => None,
                                }
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                })
            })
            .collect();

        // Join all task, unwrap result
        let results = join_all(tasks).await;

        let peers_info: Vec<PeerInfo> = results
            .into_iter()
            .filter_map(|r| r.ok().flatten())
            .collect();

        this.set_current_providers(peers_info).await;
        Ok(())
    }

    async fn filter_partners_info(&self, config: PartnerConfigCondition) -> Vec<PeerInfoRespond> {
        let providers = self.providers.read().await;
        providers
            .iter()
            .filter(|item| {
                let mut pass = true;
                if let Some(min) = config.throughput_from {
                    pass &= item.throughput >= min;
                }
                if let Some(max) = config.throughput_to {
                    pass &= item.throughput <= max;
                }
                if let Some(min) = config.jitter_from {
                    pass &= item.jitter >= min;
                }
                if let Some(max) = config.jitter_to {
                    pass &= item.jitter <= max;
                }
                if let Some(min) = config.packet_loss_from {
                    pass &= item.packet_loss >= min;
                }
                if let Some(max) = config.packet_loss_to {
                    pass &= item.packet_loss <= max;
                }

                pass
            })
            .map(|speed| PeerInfoRespond {
                peer_ip_v4: speed.peer_ip_v4.clone(),
                version: speed.version.clone(),
                throughput: format!("{:.2} Mbps", speed.throughput),
                jitter: format!("{} ms", speed.jitter),
                packet_loss: format!("{:.2} %", speed.packet_loss),
                connected_at: speed.connected_at,
                updated_at: speed.updated_at,
                total_test_performed: speed.total_test_performed,
            })
            .collect()
    }
}

impl PartnerServiceImpl {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(Vec::new()),
        }
    }

    pub async fn set_current_providers(&self, providers: Vec<PeerInfo>) {
        let mut providers_ref = self.providers.write().await;
        *providers_ref = providers;
    }
}
