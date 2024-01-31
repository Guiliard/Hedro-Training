use crate::services::message::RMQMessage;
use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::{
    options:: *,
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use std::env;
use log::{ error, info };


struct RMQConfigs {
    host: String,
    port: String,
    user: String,
    password: String,
    queue_name: String,
    exchange_name: String,
    consumer_name: String,
}

pub struct RMQConnection {}

impl RMQConnection {

    pub fn new() -> Self { return RMQConnection{}; }

    fn envs(&self) -> Result <RMQConfigs, ()> {

        let Ok(host) = env::var("RABBITMQ_HOST") else {
            error!("Failed to read RABBIT_HOST env....");
            return Err(());
        };

        let Ok(port) = env::var("RABBITMQ_PORT") else {
            error!("Failed to read RABBITMQ_PORT env....");
            return Err(());
        };

        let Ok(user) = env::var("RABBITMQ_USER") else {
            error!("Failed to read RABBITMQ_USER env....");
            return Err(());
        };

        let Ok(password) = env::var("RABBITMQ_PASSWORD") else {
            error!("Failed to read RABBITMQ_PASSWORD env....");
            return Err(());
        };

        let Ok(exchange_name) = env::var("RABBITMQ_EXCHANGE") else {
            error!("Failed to read RABBITMQ_EXCHANGE env....");
            return Err(());
        };

        let Ok(queue_name) = env::var("RABBITMQ_QUEUE") else {
            error!("Failed to read RABBITMQ_QUEUE env....");
            return Err(());
        };

        let Ok(consumer_name) = env::var("RABBITMQ_QUEUE_CONSUMER") else {
            error!("Failed to read RABBITMQ_QUEUE env....");
            return Err(());
        };

        Ok(RMQConfigs {
            host,
            port, 
            user,
            password,
            exchange_name,
            queue_name,
            consumer_name,
        })
    }

    pub async fn connect(&mut self) -> Result < (Connection,Channel), () > {

        let envs = self.envs()?;

        info!("Starting RabbitMq conection!!");

        let addr = format!(
            "amqp://{}:{}@{}:{}",
            envs.user, envs.password, envs.host, envs.port
        );

        let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
            error!("RabbitMq connection failre....");
            return Err(());
        };

        info!("RabbitMq conected!");
        info!("Starting RabbitMq chanel!!");

        let Ok(channel) = conn.create_channel().await else {
            error!("RabbitMq channel failure....");
            return Err(());
        };

        info!("RabbitMq channel created!");

        let Ok(_exchange) = channel
        .exchange_declare(
            &envs.exchange_name,
            lapin::ExchangeKind::Fanout,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        else {
            error!("Rabbitmq exchange failure....");
            return Err(());
        };

        info!("Rabbitmq exchange created! ");

        let Ok(_queue) = channel
            .queue_declare(
                &envs.queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Rabbitmq queue failure....");
            return Err(());
        };

        info!("Rabbitmq queue created!");

        let Ok(_queue_bind) = channel
            .queue_bind(
                &envs.queue_name,
                &envs.exchange_name,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("Rabbitmq queue bind failure....");
            return Err(());
        };

        let mut consumer = channel
            .basic_consume(
                &envs.queue_name,
                &envs.consumer_name,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failure to creat the consumer....");

        while let Some(consumer_product) = consumer.next().await {

            let consumer_product = match consumer_product {

                Ok(consumer_product) => consumer_product,
                Err(err) => { error!("Error in consumer: {:?}....", err); continue; }
            };
            
            let consumer_tag = consumer_product.delivery_tag;

            self.handler(Ok(consumer_product)).await;

            if let Err(err) = channel
                .basic_ack(consumer_tag, BasicAckOptions::default())
                .await
                {
                    error!("Failed to acknowledge message: {:?}....", err);
                } 
                else { info!("Message acknowledgment successful!"); }
        }

        Ok((conn, channel))
    }

    async fn handler(&self, consumer_product: Result< Delivery, lapin::Error >) {
        
        let data = match consumer_product {
            Ok(data) => data,
            Err(_) => { error!("Error receiving message from RabbitMQ...."); return; }
        };

        let msg = data.data;

        let deserialized_msg: Result<RMQMessage, _> = serde_json::from_slice(&*msg);

        match deserialized_msg {
            Ok(deserialized_msg) => { info!("Received message successfully: {:?}!", deserialized_msg); }
            Err(err) => { error!("Failed to deserialize message: {:?}....", err); }
        }
    }
}