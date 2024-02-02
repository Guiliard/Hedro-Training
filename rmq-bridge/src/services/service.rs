use crate::services::messages::MQTTMessage;
use async_trait::async_trait;
use log::{ debug, error, info };

#[async_trait]
pub trait Messaging {
    async fn publish(&self, destination: String, data: &[u8]) -> Result<(), ()>;
}

#[async_trait]
pub trait BridgeService {
    async fn exec(&self, msg: &MQTTMessage) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {
    messaging: Box <dyn Messaging + Sync + Send>,
}

impl BridgeServiceImpl {
    pub fn new(messaging: Box <dyn Messaging + Sync + Send>) -> Self { BridgeServiceImpl { messaging } }
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {

    async fn exec(&self, msg: &MQTTMessage) -> Result<(), ()> {

        debug!("Message Received!!");

        let Ok(serialized) = serde_json::to_vec(msg) else {
            error!("Failed to serialize message....");
            return Err(())
        };

        match self.messaging.publish("Hedro_Test".into(), &serialized).await {
            Ok(_) => {
                info!("Message published!!");
                Ok(())
            }
            Err(_) => {
                error!("Failured to publish message!!");
                Err(())
            }
        }
    }
}