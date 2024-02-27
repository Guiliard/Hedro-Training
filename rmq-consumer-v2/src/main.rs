mod infra;
mod services;

use infra::aws_messenger::AWSMessenger;
use log::info;
use services::service::BridgeServiceImpl;
use crate::infra::rmq_messenger::RabbitMQMessenger;


#[tokio::main]
async fn main() 
{
    dotenvy::dotenv().expect("Failure to read .env....");
    env_logger::init();

    info!("starting aplication.....");

    let aws_messenger = AWSMessenger::new();

    let service = BridgeServiceImpl::new(Box::new(aws_messenger));

    let _rmq_messenger = RabbitMQMessenger::new(Box::new(service))
        .connect()
        .await
        .expect("Failed to connect to RabbitMQ Broker....");
}