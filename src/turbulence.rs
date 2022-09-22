use serde::{Serialize, Deserialize};
use schemars::{JsonSchema};

#[cfg_attr(feature = "jsonse", derive(JsonSchema))]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "TurbulencePointJSON")]
pub struct TurbulencePoint {
    pub diameter: f64,
    pub position: f64,
    pub start_time: f64,
    pub end_time: f64,
}

impl Default for TurbulencePoint {
    fn default() -> TurbulencePoint {
        TurbulencePoint {
            diameter: 0.0,
            position: 0.0,
            start_time: 0.0,
            end_time: f64::NAN,
        }
    }
}