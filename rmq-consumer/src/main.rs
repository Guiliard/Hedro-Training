use log::info;
use crate::infra::rmq_messaging::RMQConnection;

mod infra;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failure to read .env....");
    env_logger::init();

    info!("Starting consumer application....");

    let (_rmq_conn, _rmq_channel) = RMQConnection::new()
        .connect()
        .await
        .expect("RabbitMQ connection failure....");

    info!("RabbitMQ connected!");
}
