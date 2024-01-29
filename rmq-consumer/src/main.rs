use futures_util::StreamExt;
use lapin::{options::BasicConsumeOptions, types::FieldTable, Connection, ConnectionProperties};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), ()> {
    
    env_logger::init();

    info!("Starting application!");

    info!("Starting rabbitmq connection!");

    let addr = format!("amqp://{}:{}@{}:{}", "guest", "guest", "localhost", "5672");

    let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
        error!("Rabbitmq connection failure....");
        return Err(());
    };

    info!("Rabbitmq connected!");

    info!("Starting rabbitmq channel!");

    let Ok(channel) = conn.create_channel().await else {
        error!("rabbitmq channel failure....");
        return Err(());
    };

    let mut consumer = channel
        .basic_consume(
            "batatinha",
            "batatinha-consumer",
            BasicConsumeOptions {
                no_ack: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Failure to create consumer....");

    while let Some(event) = consumer.next().await {
        let Ok(delivery) = event else {
            error!("Error to receive rmq msg....");
            continue;
        };

        info!("message received!");

        let data = delivery.data;
        info!("{:?}", data);

        info!("message processed successfully!");
    }

    Ok(())
}