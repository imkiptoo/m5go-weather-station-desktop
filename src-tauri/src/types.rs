use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Default timestamp function for when timestamp field is missing
fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub location: String,
    pub gps_lat: f64,
    pub gps_lon: f64,
    pub condition: String,
    pub current_icon: String,
    pub wind_speed: f64,
    pub wind_direction: String,
    pub current_temp: f64,
    pub humidity: i32,
    pub pressure: i32,
    pub forecast: Vec<ForecastDay>,
    pub history: Vec<HistoryDay>,
    #[serde(default = "default_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastDay {
    pub day: String,
    pub date: String,
    pub temp: f64,
    pub humidity: i32,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDay {
    pub day: String,
    pub date: String,
    pub temp: f64,
    pub humidity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertLevel {
    Info,
    Warning,
    Emergency,
}

impl Default for AlertLevel {
    fn default() -> Self {
        AlertLevel::Info
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertData {
    pub message: String,
    #[serde(default)]
    pub level: AlertLevel,
    #[serde(default = "default_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    pub broker_host: String,
    pub broker_port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            broker_host: "192.168.137.1".to_string(),
            broker_port: 1883,
            username: None,
            password: None,
            client_id: format!("weather-desktop-{}", chrono::Utc::now().timestamp()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub mqtt: bool,
    pub api: bool,
    pub last_update: DateTime<Utc>,
}