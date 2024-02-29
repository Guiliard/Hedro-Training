use crate::services::messages::RMQMessage;
use async_trait::async_trait;
use aws_sdk_timestreamwrite::{
    self,
    types::{Dimension, DimensionValueType, MeasureValue, MeasureValueType, Record, TimeUnit},
};
use log::{error, info};
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

#[async_trait]
pub trait Messaging { async fn publish(&self, record: Record) -> Result<(), ()>; }

#[async_trait]
pub trait BridgeService { async fn exec(&self, data: &RMQMessage) -> Result<(), ()>; }

pub struct BridgeServiceImpl { messaging: Box<dyn Messaging + Sync + Send>, }

impl BridgeServiceImpl {
    pub fn new(messaging: Box<dyn Messaging + Sync + Send>) -> Self {
        BridgeServiceImpl { messaging }
    }
}

#[async_trait]
impl BridgeService for BridgeServiceImpl {
    async fn exec(&self, data: &RMQMessage) -> Result<(), ()> {
        info!("Message Received!!");

        let time_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Money is more precious than time?");

        let measure = MeasureValue::builder()
            .set_name(Some(data.typ.to_string()))
            .set_type(Some(MeasureValueType::Double))
            .set_value(Some(data.value.to_string()))
            .build()
            .expect("Failed to create MeasureValue....");

        let dimension = Dimension::builder()
            .set_name(Some("Device".into()))
            .set_dimension_value_type(Some(DimensionValueType::Varchar))
            .set_value(Some(data.device.to_string()))
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

        match self.messaging.publish(record).await {
            Ok(_) => {
                info!("Message published!!");
                Ok(())
            }
            Err(_) => {
                error!("Failured to publish message!!");
                Err(())
            }
        }
    }
}
