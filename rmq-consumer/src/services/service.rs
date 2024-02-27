use crate::services::message::RMQMessage;
use async_trait::async_trait;
use aws_sdk_timestreamwrite::types::Record;
use log::{debug, error, info};

#[async_trait]
pub trait Messaging {
    async fn publish(&self, record: Record) -> Result<(), ()>;
}

#[async_trait]
pub trait BridgeService {
    async fn exec(&self, record: Record) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {
    messaging: Box<dyn Messaging + Sync + Send>,
}

impl BridgeServiceImpl {
    pub fn new(messaging: Box<dyn Messaging + Sync + Send>) -> Self {
        BridgeServiceImpl { messaging }
    }
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {
    async fn exec(&self, record: Record) -> Result<(), ()> {
        debug!("Message Received!!");

        match self.messaging.publish(record).await {
            Ok(_) => {
                info!("Message published!!");
                Ok(())
            }
            Err(_) => {
                error!("Failured to publish message....");
                Err(())
            }
        }
    }
}
