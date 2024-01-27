use crate::services::service::Messaging;
use async_trait::async_trait;
use lapin::{ BasicProperties, Channel, Connection, ConnectionProperties };
use log::{ error, info };
use std::env; 

struct RabbbitMQConfigs {

    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct RabbitMQMessaging {

    conn: Option <Connection>,
    channel: Option <Channel>,
}

impl RabbitMQMessaging {

    pub fn new() -> Self {
        RabbitMQMessaging { conn: None, channel: None, }
    }
}

#[async_trait]
impl Messaging for RabbitMQMessaging {

    async fn publish (&self, destination: String, data: &[u8]) -> Result <(), ()>{
        if self.channel.is_none() {
            error!("Connection wasn´t establish yet....");
            return Err(());
        }

        match self.channel
            .clone()
            .unwrap()
            .basic_publish(
                &destination,
                "",
                lapin::options::BasicPublishOptions {
                    mandatory: false,
                    immediate: false,
                },
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

impl RabbitMQMessaging {

    fn envs(&self) -> Result <RabbbitMQConfigs, ()> {

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

        Ok(RabbbitMQConfigs {
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
        self.conn = Some(conn);
        self.channel = Some(channel);

        Ok(())
    }
}