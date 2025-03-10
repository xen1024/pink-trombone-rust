use serde::{Serialize, Deserialize};
use schemars::{JsonSchema};

#[cfg_attr(feature = "jsonse", derive(JsonSchema))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "TransientJSON")]
pub struct Transient {
    pub position: usize,
    pub start_time: f32,
    pub life_time: f32,
    pub strength: f64,
    pub exponent: f64,
}