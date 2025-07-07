use std::{sync::Arc, time::Duration};

use tokio_cron_scheduler::{Job, JobScheduler};

use crate::services::partner_service::{PartnerService, PartnerServiceImpl};

pub async fn start_cronjob(partner_svc: Arc<PartnerServiceImpl>) {
    let sched = JobScheduler::new().await.unwrap();
    sched
        .add(
            Job::new_async("*/30 * * * * *", move |_uuid, _l| {
                let partner_service = partner_svc.clone();

                Box::pin(async move {
                    println!("This runs every 30s!");
                    partner_service.fetch_partners_info().await.unwrap();
                })
            })
            .unwrap(),
        )
        .await
        .unwrap();

    sched.start().await.unwrap();

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
