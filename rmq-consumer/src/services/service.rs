use crate::services::message::RMQMessage;

pub trait BridgeService {
    async fn exec(&self, msg: &RMQMessage) -> Result<(), ()>;
}

pub struct BridgeServiceImpl {}

impl BridgeServiceImpl { pub fn new() -> Self { BridgeServiceImpl {} } }

impl BridgeService for BridgeServiceImpl { async fn exec(&self, _msg: &RMQMessage) -> Result <(), ()> { Ok(()) } }