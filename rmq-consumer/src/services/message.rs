use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RMQMessage {
    pub device: String,
    #[serde(rename(deserialize = "type"))]
    pub typ: String,
    pub value: String,
}
