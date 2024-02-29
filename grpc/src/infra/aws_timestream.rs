use aws_sdk_timestreamwrite::{types::Record, Client};
use log::{error, info};

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

    pub async fn query (&self, query_name: String) -> Result <QueryOutput, SdkError <QueryError> > {

        match self.connect()
            .query()
            .query_string(query_name)
            .send()
            .await
            {
                Ok((query, _)) => Ok(query),
                Err(err) => {
                    error!("Failure to connect with aws query....");
                    return Err(err.into());
                }
            }
    }
}
