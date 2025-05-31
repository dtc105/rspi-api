use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum FloatOrString {
    Float(f32),
    String(String),
}

#[derive(Debug, Serialize)]
pub struct SystemUptime {
    #[serde(rename = "upTime")]
    pub up_time: FloatOrString,
    #[serde(rename = "idleTime")]
    pub idle_time: FloatOrString,
}

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    #[serde(rename = "cpuTemp")]
    pub cpu_temp: FloatOrString,
    #[serde(rename = "systemUptime")]
    pub system_uptime: SystemUptime,
}
