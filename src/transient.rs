use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transient {
    pub position: usize,
    pub start_time: f32,
    pub life_time: f32,
    pub strength: f64,
    pub exponent: f64,
}