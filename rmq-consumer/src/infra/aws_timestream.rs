use crate::services::message::RMQMessage;
use aws_sdk_timestreamwrite::{
    self,
    types::{Dimension, DimensionValueType, MeasureValue, MeasureValueType, Record, TimeUnit},
};
use log::{ error, info };
use std::{
    env,
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

struct AWSConfigs {
    database: String,
    table: String,
}

pub struct AWSConnection {
    message: RMQMessage,
}

impl AWSConnection { 
    
    pub fn new (message: RMQMessage) -> Self { AWSConnection { message } }

    fn envs(&self) -> Result <AWSConfigs, ()> {

        let Ok(database) = env::var("AWS_DATABASE_NAME") else {
            error!("Failed to read AWS_DATABASE_NAME env....");
            return Err(());
        };

        let Ok(table) = env::var("AWS_TABLE_NAME") else {
            error!("Failed to read AWS_TABLE_NAME env....");
            return Err(());
        };

        Ok(AWSConfigs { 
            database, 
            table,
        })
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        
        let config = aws_config::load_from_env().await;

        let env = match self.envs() {
            Ok(env) => env,
            Err(_) => return Err("Failure to read .env....".into()),
        };

        let client = match aws_sdk_timestreamwrite::Client::new(&config)
        .with_endpoint_discovery_enabled()
        .await
        {
            Ok((c, _)) => Ok(c),
            Err(err) => {
                error!("failure to connect");
                Err(err)
            }
        }?;

        let time_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Money is more precious than time?");

        let measure = MeasureValue::builder()
            .set_name(Some(self.message.typ.to_string()))
            .set_type(Some(MeasureValueType::Double))
            .set_value(Some(self.message.value.to_string()))
            .build()
            .expect("Failed to create MeasureValue....");

        let dimension = Dimension::builder()
            .set_name(Some("Device".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .set_value(Some(self.message.device.to_string()))
            .build()
            .expect("Failed to create Dimension....");

        let record = Record::builder()
            .set_time(Some(time_epoch.as_millis().to_string()))
            .set_time_unit(Some(TimeUnit::Milliseconds))
            .set_measure_name(Some("mesure-name".into()))
            .set_measure_values(Some(vec![measure]))
            .set_measure_value_type(Some(MeasureValueType::Multi))
            .set_dimensions(Some(vec![dimension]))
            .build();

        match client
            .write_records()
            .set_database_name(Some(env.database.into()))
            .set_table_name(Some(env.table.into()))
            .set_records(Some(vec![record]))
            .send()
            .await
            {
                Ok(_) => {
                    info!("Inserted values in database!")
                }
                Err(err) => {
                    error!("Failure to insert the values in database....");
                    error!("{:?}", err);
                }
            }

        Ok(())
    }
}
