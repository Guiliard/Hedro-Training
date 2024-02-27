use async_trait::async_trait;
use aws_sdk_timestreamwrite::{types::Record, Client};
use log::{error, info};
use std::{env, error::Error};

use crate::services::service::Messaging;

struct AWSConfigs {
    database: String,
    table: String,
}

pub struct AWSMessenger {}

impl AWSMessenger {
    pub fn new() -> Self {
        Self {}
    }
}

impl AWSMessenger {
    fn envs(&self) -> Result<AWSConfigs, ()> {
        let Ok(database) = env::var("AWS_DATABASE_NAME") else {
            error!("Failed to read AWS_DATABASE_NAME env....");
            return Err(());
        };

        let Ok(table) = env::var("AWS_TABLE_NAME") else {
            error!("Failed to read AWS_TABLE_NAME env....");
            return Err(());
        };

        Ok(AWSConfigs { database, table })
    }

    pub async fn connect(&self) -> Result<Client, Box<dyn Error + Send + Sync>> {
        let config = aws_config::load_from_env().await;

        let client = match Client::new(&config).with_endpoint_discovery_enabled().await {
            Ok((c, _)) => Ok(c),
            Err(err) => {
                error!("Failure to connect....");
                Err(err)
            }
        }?;

        Ok(client)
    }
}

#[async_trait]
impl Messaging for AWSMessenger {
    async fn publish(&self, record: Record) -> Result<(), ()> {
        let envs = self.envs()?;

        let Ok(client) = self.connect().await else {
            error!("Failed to connect to client.");
            return Err(());
        };

        match client
            .write_records()
            .set_database_name(Some(envs.database.into()))
            .set_table_name(Some(envs.table.into()))
            .set_records(Some(vec![record]))
            .send()
            .await
        {
            Ok(_) => {
                info!("Inserted values in database!");
            }
            Err(err) => {
                error!("Failure to insert the values in database....");
                error!("{:?}", err);
            }
        }

        Ok(())
    }
}
