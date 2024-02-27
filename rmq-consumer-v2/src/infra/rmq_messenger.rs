use std::env;

use futures_util::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Consumer,
};
use log::{error, info};

use crate::services::{messages::RMQMessage, service::BridgeService};

struct RabbitMQConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    exchange: String,
    queue: String,
}

pub struct RabbitMQMessenger {
    service: Box<dyn BridgeService>
}

impl RabbitMQMessenger {
    pub fn new(service: Box<dyn BridgeService>) -> Self {
        Self { service }
    }
}

impl RabbitMQMessenger {
    fn envs(&self) -> Result<RabbitMQConfig, ()> {
        let Ok(host) = env::var("RABBITMQ_HOST") else {
            error!("Failed to read RABBIT_HOST env....");
            return Err(())
        };

        let Ok(port) = env::var("RABBITMQ_PORT") else {
            error!("Failed to read RABBITMQ_PORT env....");
            return Err(())
        };

        let Ok(user) = env::var("RABBITMQ_USER") else {
            error!("Failed to read RABBITMQ_USER env....");
            return Err(())
        };

        let Ok(password) = env::var("RABBITMQ_PASSWORD") else {
            error!("Failed to read RABBITMQ_PASSWORD env....");
            return Err(())
        };

        let Ok(exchange) = env::var("RABBITMQ_EXCHANGE") else {
            error!("Failed to read RABBITMQ_EXCHANGE env....");
            return Err(())
        };

        let Ok(queue) = env::var("RABBITMQ_QUEUE") else {
            error!("Failed to read RABBITMQ_QUEUE env....");
            return Err(())
        };

        Ok(RabbitMQConfig {
            host,
            port,
            user,
            password,
            exchange,
            queue,
        })
    }

    pub async fn connect(&mut self) -> Result<(), ()> {
        let envs = self.envs()?;

        info!("Starting RabbitMq conection!!");

        let addr = format!(
            "amqp://{}:{}@{}:{}",
            envs.user, envs.password, envs.host, envs.port
        );

        let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
            error!("RabbitMq connection failre....");
            return Err(())
        };

        info!("RabbitMq conected!");
        info!("Starting RabbitMq chanel!!");

        let Ok(channel) = conn.create_channel().await else {
            error!("RabbitMq channel failure....");
            return Err(())
        };

        info!("RabbitMq channel created!");

        let Ok(_exchange) = channel
            .exchange_declare(
                &envs.exchange,
                lapin::ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Rabbitmq exchange failure....");
            return Err(())
        };

        info!("Rabbitmq exchange created! ");

        let Ok(_queue) = channel
            .queue_declare(
                &envs.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Rabbitmq queue failure....");
            return Err(())
        };

        info!("Rabbitmq queue created!");

        let Ok(_queue_bind) = channel
            .queue_bind(
                &envs.queue,
                &envs.exchange,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Rabbitmq queue bind failure....");
            return Err(())
        };

        info!("Rabbitmq queue bind created!");

        let mut consumer = self.consume(channel).await?;

        while let Some(event) = consumer.next().await {
            let Ok(delivery) = event else {
                error!("Failed to deliver message....");
                return Err(())
            };
            self.handler(delivery).await;
        }

        Ok(())
    }

    async fn consume(&self, channel: Channel) -> Result<Consumer, ()> {
        let envs = self.envs()?;
        let Ok(consumer) = channel
            .basic_consume(
                &envs.queue,
                "consumer-test",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Failed to create consumer....");
            return Err(())
        };

        info!("Consumer created!");
        Ok(consumer)
    }

    async fn handler(&self, delivery: Delivery) {
        let Ok(_ack) = delivery.ack(BasicAckOptions::default()).await else {
            error!("Failed to ack message.");
            return
        };

        let Ok(deserialized_msg): Result <RMQMessage, _> = serde_json::from_slice(&delivery.data) else {
            error!("Failed to deserialize....");
            return
        };
        info!("Message: {:?}", deserialized_msg);

        match self.service.exec(&deserialized_msg).await {
            
            Ok(_) => {
                info!("Message processed successfully!")
            }

            Err(_) => error!("Failed to process message....")
        }
    }
}
