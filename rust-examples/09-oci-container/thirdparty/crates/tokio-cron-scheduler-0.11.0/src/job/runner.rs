use crate::context::Context;
#[cfg(not(feature = "has_bytes"))]
use crate::job::job_data::JobState;
#[cfg(feature = "has_bytes")]
use crate::job::job_data_prost::JobState;
use crate::job::to_code::JobCode;
use crate::job_scheduler::JobsSchedulerLocked;
use crate::JobSchedulerError;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::RwLock;
use tracing::error;
use uuid::Uuid;

#[derive(Default)]
pub struct JobRunner {}

impl JobRunner {
    async fn listen_for_activations(
        job_code: Arc<RwLock<Box<dyn JobCode + Send + Sync>>>,
        mut rx: Receiver<Uuid>,
        tx_notify: Sender<(Uuid, JobState)>,
        job_scheduler: JobsSchedulerLocked,
    ) {
        loop {
            let val = rx.recv().await;
            if let Err(e) = val {
                error!("Error receiving {:?}", e);
                if matches!(e, RecvError::Closed) {
                    break;
                }
                continue;
            }
            let uuid = val.unwrap();
            {
                let tx = tx_notify.clone();
                tokio::spawn(async move {
                    if let Err(e) = tx.send((uuid, JobState::Started)) {
                        error!("Error sending error listening for activation {:?}", e);
                    }
                });
            }
            let mut w = job_code.write().await;
            let code = w.get(uuid).await;
            match code {
                Ok(Some(job)) => {
                    let mut job = job.write().await;
                    let v = (job)(uuid, job_scheduler.clone());
                    let tx = tx_notify.clone();
                    tokio::spawn(async move {
                        v.await;
                        if let Err(e) = tx.send((uuid, JobState::Done)) {
                            error!("Error sending spawned task {:?}", e);
                        }
                    });
                }
                _ => {
                    error!("Error getting {:?} from job code", uuid);
                    continue;
                }
            }
        }
    }

    pub fn init(
        &mut self,
        context: &Context,
        job_scheduler: JobsSchedulerLocked,
    ) -> Pin<Box<dyn Future<Output = Result<(), JobSchedulerError>> + Send>> {
        let job_code = context.job_code.clone();
        let notify_tx = context.notify_tx.clone();
        let job_activation_rx = context.job_activation_tx.subscribe();

        Box::pin(async move {
            tokio::spawn(JobRunner::listen_for_activations(
                job_code,
                job_activation_rx,
                notify_tx,
                job_scheduler,
            ));
            Ok(())
        })
    }
}
