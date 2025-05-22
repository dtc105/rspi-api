use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum FloatOrString {
    Float(f32),
    String(String),
}

#[derive(Deserialize, Serialize)]
pub struct SystemUptime {
    #[serde(rename = "upTime")]
    pub up_time: FloatOrString,
    #[serde(rename = "idle_time")]
    pub idle_time: FloatOrString,
}

#[derive(Deserialize, Serialize)]
pub struct SystemInfo {
    #[serde(rename = "cpuTemp")]
    pub cpu_temp: FloatOrString,
    #[serde(rename = "systemUptime")]
    pub system_uptime: SystemUptime,
}
