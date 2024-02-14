use crate::services::message::RMQMessage;
use async_trait::async_trait;

#[async_trait]
pub trait BridgeService: Send + Sync {
    async fn exec(&self, msg: &RMQMessage) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {}

impl BridgeServiceImpl {
    pub fn new() -> Self {
        BridgeServiceImpl {}
    }
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {
    async fn exec(&self, _msg: &RMQMessage) -> Result<(), ()> {
        Ok(())
    }
}
