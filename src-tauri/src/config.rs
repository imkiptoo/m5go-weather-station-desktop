use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mqtt: MqttSettings,
    pub weather_api: WeatherApiSettings,
    pub app: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttSettings {
    pub broker_host: String,
    pub broker_port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: String,
    pub auto_connect: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherApiSettings {
    pub api_key: String,
    pub latitude: f64,
    pub longitude: f64,
    pub auto_fetch_interval_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_refresh_data: bool,
    pub desktop_notifications: bool,
    pub dark_mode: bool,
    pub data_refresh_interval_seconds: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mqtt: MqttSettings::default(),
            weather_api: WeatherApiSettings::default(),
            app: AppSettings::default(),
        }
    }
}

impl Default for MqttSettings {
    fn default() -> Self {
        Self {
            broker_host: "192.168.137.1".to_string(),
            broker_port: 1883,
            username: None,
            password: None,
            client_id: format!("weather-desktop-{}", chrono::Utc::now().timestamp()),
            auto_connect: true,
        }
    }
}

impl Default for WeatherApiSettings {
    fn default() -> Self {
        Self {
            api_key: "API_KEY_HERE".to_string(),
            latitude: 48.7758,
            longitude: 9.1829,
            auto_fetch_interval_minutes: 30,
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_refresh_data: true,
            desktop_notifications: false,
            dark_mode: false,
            data_refresh_interval_seconds: 30,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path).unwrap_or_else(|e| {
            warn!("Failed to load config: {}, using defaults", e);
            AppConfig::default()
        });

        Ok(Self {
            config_path,
            config,
        })
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?
            .join("weather-station-desktop");

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
            info!("Created config directory: {:?}", config_dir);
        }

        Ok(config_dir.join("config.toml"))
    }

    fn load_config(path: &PathBuf) -> Result<AppConfig> {
        if !path.exists() {
            info!("Config file not found at {:?}, using defaults", path);
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        info!("Loaded config from {:?}", path);
        Ok(config)
    }

    pub fn save_config(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        info!("Saved config to {:?}", self.config_path);
        Ok(())
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn update_config(&mut self, config: AppConfig) -> Result<()> {
        self.config = config;
        self.save_config()
    }

    pub fn update_mqtt_settings(&mut self, mqtt: MqttSettings) -> Result<()> {
        self.config.mqtt = mqtt;
        self.save_config()
    }

    pub fn update_weather_api_settings(&mut self, weather_api: WeatherApiSettings) -> Result<()> {
        self.config.weather_api = weather_api;
        self.save_config()
    }

    pub fn update_app_settings(&mut self, app: AppSettings) -> Result<()> {
        self.config.app = app;
        self.save_config()
    }

    // Convenience getters
    pub fn mqtt_settings(&self) -> &MqttSettings {
        &self.config.mqtt
    }

    pub fn should_auto_connect_mqtt(&self) -> bool {
        self.config.mqtt.auto_connect
    }
}

// Create initial config file if it doesn't exist
pub fn ensure_config_file_exists() -> Result<PathBuf> {
    let config_path = ConfigManager::get_config_path()?;
    
    if !config_path.exists() {
        let default_config = AppConfig::default();
        let content = toml::to_string_pretty(&default_config)?;
        fs::write(&config_path, content)?;
        info!("Created default config file at {:?}", config_path);
    }

    Ok(config_path)
}