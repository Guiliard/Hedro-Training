mod infra;
mod services;

use std::sync::Arc;

use crate::infra::rmq_messaging::RMQConnection;
use crate::services::service::BridgeServiceImpl;
use log::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failure to read .env....");
    env_logger::init();

    info!("Starting consumer application....");

    let service = BridgeServiceImpl::new();

    let mut consumer = RMQConnection::new(Box::new(service));

    consumer
        .connect()
        .await
        .expect("RabbitMQ connection failure....");

    info!("RabbitMQ connected!");
}
