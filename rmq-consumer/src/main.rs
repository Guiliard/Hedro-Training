use log::info;
use crate::services::service::BridgeServiceImpl;
use crate::infra::rmq_messaging::RMQConnection;


mod infra;
mod services;

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().expect("Failure to read .env....");
    env_logger::init();

    info!("Starting consumer application....");

    let service = BridgeServiceImpl::new(Box::new());

    let mut client = RMQConnection::new(Box::new(service));

    client
        .connect()
        .await
        .expect("RabbitMQ connection failure....");

    info!("RabbitMQ connected!");
}
