use crate::services::message::RMQMessage;
use crate::services::service::BridgeService;
use crate::infra::aws_timestream::AWSConnection;
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

pub struct RMQConnection { service: Box <dyn BridgeService>,}

impl RMQConnection {

    pub fn new(service: Box <dyn BridgeService> ) -> Self { return Self { service }; }

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
            error!("RabbitMq connection failure....");
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

            match consumer_product 
            {
                Ok(consumer_product) => {

                    let consumer_tag = consumer_product.delivery_tag;

                    self.handler(consumer_product).await;

                    if let Err(err) = channel
                        .basic_ack(consumer_tag, BasicAckOptions::default())
                        .await
                        { error!("Failed to acknowledge message: {:?}....", err); } 

                    else { info!("Message acknowledgment successful!"); }
                }
                Err(err) => {

                    error!("Error in consumer: {:?}....", err);
                    continue;
                }
            };
        }

        Ok((conn, channel))
    }

    async fn handler(&self, data: Delivery) {

        let deserialized_msg: Result<RMQMessage, _> = serde_json::from_slice(&data.data);

        match deserialized_msg 
        {
            Ok(deserialized_msg) => {

                info!("Deserialized message successfully: {:?}!", deserialized_msg);

                match self.service.exec(&deserialized_msg).await 
                {
                    Ok(_) => { info!("Message processed successfully!"); }

                    Err(_) => { error!("Failure to process message...."); }
                }

                let mut aws_msg = AWSConnection::new(deserialized_msg);

                aws_msg.connect().await.expect("AWS connection failure....");
            }

            Err(err) => { error!("Failed to deserialize message: {:?}....", err); }
        }
        
    }

}