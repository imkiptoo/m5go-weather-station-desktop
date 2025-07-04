// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mqtt_client;
mod weather_api;
mod types;
mod config;

use mqtt_client::MqttManager;
use weather_api::WeatherApiClient;
use types::*;
use config::{ConfigManager, AppConfig, MqttSettings, WeatherApiSettings, AppSettings};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State, Emitter, Manager};
use tracing::{info, error};

// Application state
#[derive(Clone)]
pub struct AppState {
    mqtt_manager: Arc<Mutex<MqttManager>>,
    weather_api: Arc<WeatherApiClient>,
    config_manager: Arc<Mutex<ConfigManager>>,
    app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
}

#[tauri::command]
async fn connect_mqtt(
    broker_host: String,
    broker_port: u16,
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Connecting to MQTT broker: {}:{}", broker_host, broker_port);
    
    // Set app handle in MQTT manager
    let app_handle_guard = state.app_handle.lock().await;
    if let Some(ref handle) = *app_handle_guard {
        let mut mqtt_manager = state.mqtt_manager.lock().await;
        mqtt_manager.set_app_handle(handle.clone());
    }
    drop(app_handle_guard);
    
    let mut mqtt_manager = state.mqtt_manager.lock().await;
    match mqtt_manager.connect(&broker_host, broker_port).await {
        Ok(_) => {
            info!("Successfully connected to MQTT broker");
            Ok("Connected successfully".to_string())
        }
        Err(e) => {
            error!("Failed to connect to MQTT broker: {}", e);
            Err(format!("Connection failed: {}", e))
        }
    }
}

#[tauri::command]
async fn disconnect_mqtt(state: State<'_, AppState>) -> Result<String, String> {
    info!("Disconnecting from MQTT broker");
    
    let mut mqtt_manager = state.mqtt_manager.lock().await;
    match mqtt_manager.disconnect().await {
        Ok(_) => {
            info!("Successfully disconnected from MQTT broker");
            Ok("Disconnected successfully".to_string())
        }
        Err(e) => {
            error!("Failed to disconnect from MQTT broker: {}", e);
            Err(format!("Disconnect failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_mqtt_status(state: State<'_, AppState>) -> Result<bool, String> {
    let mqtt_manager = state.mqtt_manager.lock().await;
    Ok(mqtt_manager.is_connected())
}

#[tauri::command]
async fn publish_weather_data(
    data: WeatherData,
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Publishing weather data to MQTT");
    
    let mqtt_manager = state.mqtt_manager.lock().await;
    match mqtt_manager.publish_weather_data(&data).await {
        Ok(_) => {
            info!("Weather data published successfully");
            Ok("Data published successfully".to_string())
        }
        Err(e) => {
            error!("Failed to publish weather data: {}", e);
            Err(format!("Publish failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_latest_weather_data(state: State<'_, AppState>) -> Result<Option<WeatherData>, String> {
    let mqtt_manager = state.mqtt_manager.lock().await;
    Ok(mqtt_manager.get_latest_weather_data().await)
}

#[tauri::command]
async fn get_sensor_data(state: State<'_, AppState>) -> Result<Option<SensorData>, String> {
    let mqtt_manager = state.mqtt_manager.lock().await;
    Ok(mqtt_manager.get_latest_sensor_data().await)
}

#[tauri::command]
async fn fetch_weather_api(
    lat: f64,
    lon: f64,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<WeatherData, String> {
    info!("Fetching weather data from API for lat: {}, lon: {}", lat, lon);
    
    match state.weather_api.fetch_weather(lat, lon, &api_key).await {
        Ok(weather_data) => {
            info!("Weather data fetched successfully");
            Ok(weather_data)
        }
        Err(e) => {
            error!("Failed to fetch weather data: {}", e);
            Err(format!("API fetch failed: {}", e))
        }
    }
}

#[tauri::command]
async fn send_alert(
    message: String,
    level: AlertLevel,
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Sending alert: {} (level: {:?})", message, level);
    
    let mqtt_manager = state.mqtt_manager.lock().await;
    let alert = AlertData {
        message,
        level,
        timestamp: chrono::Utc::now(),
    };
    
    match mqtt_manager.send_alert(&alert).await {
        Ok(_) => {
            info!("Alert sent successfully");
            Ok("Alert sent successfully".to_string())
        }
        Err(e) => {
            error!("Failed to send alert: {}", e);
            Err(format!("Alert failed: {}", e))
        }
    }
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config_manager = state.config_manager.lock().await;
    Ok(config_manager.get_config().clone())
}

#[tauri::command]
async fn save_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut config_manager = state.config_manager.lock().await;
    match config_manager.update_config(config) {
        Ok(_) => {
            info!("Configuration saved successfully");
            Ok("Configuration saved successfully".to_string())
        }
        Err(e) => {
            error!("Failed to save configuration: {}", e);
            Err(format!("Failed to save configuration: {}", e))
        }
    }
}

#[tauri::command]
async fn save_mqtt_settings(
    mqtt_settings: MqttSettings,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut config_manager = state.config_manager.lock().await;
    match config_manager.update_mqtt_settings(mqtt_settings) {
        Ok(_) => {
            info!("MQTT settings saved successfully");
            Ok("MQTT settings saved successfully".to_string())
        }
        Err(e) => {
            error!("Failed to save MQTT settings: {}", e);
            Err(format!("Failed to save MQTT settings: {}", e))
        }
    }
}

#[tauri::command]
async fn save_weather_api_settings(
    weather_api_settings: WeatherApiSettings,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut config_manager = state.config_manager.lock().await;
    match config_manager.update_weather_api_settings(weather_api_settings) {
        Ok(_) => {
            info!("Weather API settings saved successfully");
            Ok("Weather API settings saved successfully".to_string())
        }
        Err(e) => {
            error!("Failed to save Weather API settings: {}", e);
            Err(format!("Failed to save Weather API settings: {}", e))
        }
    }
}

#[tauri::command]
async fn save_app_settings(
    app_settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut config_manager = state.config_manager.lock().await;
    match config_manager.update_app_settings(app_settings) {
        Ok(_) => {
            info!("App settings saved successfully");
            Ok("App settings saved successfully".to_string())
        }
        Err(e) => {
            error!("Failed to save app settings: {}", e);
            Err(format!("Failed to save app settings: {}", e))
        }
    }
}

#[tauri::command]
async fn test_emit_sensor_data(
    app: tauri::AppHandle,
) -> Result<String, String> {
    let test_sensor_data = SensorData {
        temperature: 25.5,
        humidity: 60.0,
        pressure: 1013.2,
        timestamp: "2025-07-03T12:00:00".to_string(),
    };
    
    info!("Testing sensor data event emission");
    match app.emit_to("main", "sensor-data-updated", &test_sensor_data) {
        Ok(_) => {
            info!("Test sensor data event emitted successfully");
            Ok("Test event emitted".to_string())
        }
        Err(e) => {
            error!("Failed to emit test sensor data event: {}", e);
            Err(format!("Failed to emit test event: {}", e))
        }
    }
}

#[tauri::command]
async fn start_automated_weather_publishing(
    lat: f64,
    lon: f64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Starting automated weather publishing for coordinates: {}, {}", lat, lon);
    
    let mut mqtt_manager = state.mqtt_manager.lock().await;
    match mqtt_manager.start_automated_weather_publishing(lat, lon).await {
        Ok(_) => {
            info!("Automated weather publishing started successfully");
            Ok("Automated weather publishing started".to_string())
        }
        Err(e) => {
            error!("Failed to start automated weather publishing: {}", e);
            Err(format!("Failed to start automated publishing: {}", e))
        }
    }
}

#[tauri::command]
async fn stop_automated_weather_publishing(
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Stopping automated weather publishing");
    
    let mut mqtt_manager = state.mqtt_manager.lock().await;
    match mqtt_manager.stop_automated_weather_publishing().await {
        Ok(_) => {
            info!("Automated weather publishing stopped successfully");
            Ok("Automated weather publishing stopped".to_string())
        }
        Err(e) => {
            error!("Failed to stop automated weather publishing: {}", e);
            Err(format!("Failed to stop automated publishing: {}", e))
        }
    }
}

#[tauri::command]
async fn is_auto_publishing(state: State<'_, AppState>) -> Result<bool, String> {
    let mqtt_manager = state.mqtt_manager.lock().await;
    Ok(mqtt_manager.is_auto_publishing())
}

#[tauri::command]
async fn fetch_weather_with_default_key(
    lat: f64,
    lon: f64,
    state: State<'_, AppState>,
) -> Result<WeatherData, String> {
    info!("Fetching weather data with default API key for coordinates: {}, {}", lat, lon);
    
    match state.weather_api.fetch_weather_with_default_key(lat, lon).await {
        Ok(weather_data) => {
            info!("Weather data fetched successfully with default key");
            Ok(weather_data)
        }
        Err(e) => {
            error!("Failed to fetch weather data with default key: {}", e);
            Err(format!("API fetch failed: {}", e))
        }
    }
}

#[tauri::command]
async fn refresh_weather_cache(
    lat: f64,
    lon: f64,
    state: State<'_, AppState>,
) -> Result<String, String> {
    info!("Manually refreshing weather cache for coordinates: {}, {}", lat, lon);
    
    match state.weather_api.ensure_daily_cache(lat, lon).await {
        Ok(_) => {
            info!("Weather cache refreshed successfully");
            Ok("Weather cache refreshed successfully".to_string())
        }
        Err(e) => {
            error!("Failed to refresh weather cache: {}", e);
            Err(format!("Cache refresh failed: {}", e))
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting Weather Station Desktop Application");
    
    // Ensure config file exists
    if let Err(e) = config::ensure_config_file_exists() {
        error!("Failed to ensure config file exists: {}", e);
    }
    
    // Initialize configuration manager
    let config_manager = match ConfigManager::new() {
        Ok(manager) => Arc::new(Mutex::new(manager)),
        Err(e) => {
            error!("Failed to initialize config manager: {}", e);
            Arc::new(Mutex::new(ConfigManager::new().unwrap_or_else(|_| {
                panic!("Could not create config manager")
            })))
        }
    };
    
    // Initialize application state
    let mqtt_manager = Arc::new(Mutex::new(MqttManager::new()));
    let weather_api = Arc::new(WeatherApiClient::new());
    
    let app_state = AppState {
        mqtt_manager: Arc::clone(&mqtt_manager),
        weather_api,
        config_manager: Arc::clone(&config_manager),
        app_handle: Arc::new(Mutex::new(None)),
    };
    
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            connect_mqtt,
            disconnect_mqtt,
            get_mqtt_status,
            publish_weather_data,
            get_latest_weather_data,
            get_sensor_data,
            fetch_weather_api,
            fetch_weather_with_default_key,
            refresh_weather_cache,
            send_alert,
            get_config,
            save_config,
            save_mqtt_settings,
            save_weather_api_settings,
            save_app_settings,
            test_emit_sensor_data,
            start_automated_weather_publishing,
            stop_automated_weather_publishing,
            is_auto_publishing
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let state: State<AppState> = app.state();
            let app_handle_arc = state.app_handle.clone();
            let config_manager_clone = state.config_manager.clone();
            let mqtt_manager_clone = state.mqtt_manager.clone();
            
            // Store app handle in the app state and handle auto-connect
            tokio::spawn(async move {
                // Store app handle first
                {
                    let mut handle_guard = app_handle_arc.lock().await;
                    *handle_guard = Some(app_handle.clone());
                }
                
                // Small delay to ensure everything is initialized
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                
                // Auto-connect to MQTT if enabled
                let config_guard = config_manager_clone.lock().await;
                if config_guard.should_auto_connect_mqtt() {
                    let mqtt_settings = config_guard.mqtt_settings().clone();
                    info!("Auto-connecting to MQTT broker: {}:{}", mqtt_settings.broker_host, mqtt_settings.broker_port);
                    
                    drop(config_guard); // Release lock before MQTT operation
                    
                    // Set app handle in MQTT manager first
                    {
                        let mut mqtt_manager = mqtt_manager_clone.lock().await;
                        mqtt_manager.set_app_handle(app_handle);
                    }
                    
                    let mut mqtt_guard = mqtt_manager_clone.lock().await;
                    match mqtt_guard.connect(&mqtt_settings.broker_host, mqtt_settings.broker_port).await {
                        Ok(_) => info!("Auto-connected to MQTT successfully"),
                        Err(e) => error!("Auto-connect to MQTT failed: {}", e),
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}