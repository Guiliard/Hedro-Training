use crate::services::service::Messaging;
use async_trait::async_trait;
use lapin::{ BasicProperties, Channel, Connection, ConnectionProperties };
use log::{ error, info };
use std::env; 

struct RabbitMQConfigs {

    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct RabbitMQConnection {}

pub struct RabbitMQMessaging {

    conn: Connection,
    channel: Channel,
}

impl RabbitMQMessaging {

    pub fn new(conn: Connection, channel: Channel) -> Self {
        RabbitMQMessaging { conn, channel }
    }
}

#[async_trait]
impl Messaging for RabbitMQMessaging {

    async fn publish (&self, destination: String, data: &[u8]) -> Result <(), ()> {

        match self.channel
            .basic_publish(
                &destination,
                "",
                lapin::options::BasicPublishOptions::default(),
                data,
                BasicProperties::default(),
            )
            .await 
            {
                Ok(_) => {
                    info!("Published to rmq!");
                    Ok(())
                }
                Err(_) => {
                    error!("Failed to publish msg to rmq....");
                    Err(())
                }
            }
    }
}

impl RabbitMQConnection {

    pub fn new() -> Self { return  RabbitMQConnection {};}

    fn envs(&self) -> Result <RabbitMQConfigs, ()> {

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

        Ok(RabbitMQConfigs {
            host,
            port, 
            user,
            password,
        })
    }

    pub async fn connect(&mut self) -> Result < (),() > {

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

        Ok(())
    }
}