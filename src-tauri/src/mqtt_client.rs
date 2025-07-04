use crate::types::*;
use crate::weather_api::WeatherApiClient;
use anyhow::{Result, anyhow};
use rumqttc::{AsyncClient, MqttOptions, Event, Packet, QoS};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration, interval};
use tracing::{info, error, warn, debug};
// Removed unused imports: Local and ChronoDuration
use tauri::{AppHandle, Emitter};

pub struct MqttManager {
    client: Option<AsyncClient>,
    config: MqttConfig,
    connected: bool,
    latest_weather_data: Arc<Mutex<Option<WeatherData>>>,
    latest_sensor_data: Arc<Mutex<Option<SensorData>>>,
    event_loop_handle: Option<tokio::task::JoinHandle<()>>,
    weather_publish_handle: Option<tokio::task::JoinHandle<()>>,
    app_handle: Option<AppHandle>,
    weather_api_client: Arc<WeatherApiClient>,
}

impl MqttManager {
    pub fn new() -> Self {
        Self {
            client: None,
            config: MqttConfig::default(),
            connected: false,
            latest_weather_data: Arc::new(Mutex::new(None)),
            latest_sensor_data: Arc::new(Mutex::new(None)),
            event_loop_handle: None,
            weather_publish_handle: None,
            app_handle: None,
            weather_api_client: Arc::new(WeatherApiClient::new()),
        }
    }

    pub fn set_app_handle(&mut self, app_handle: AppHandle) {
        self.app_handle = Some(app_handle);
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<()> {
        info!("Connecting to MQTT broker at {}:{}", host, port);

        // Disconnect any existing connection
        if self.event_loop_handle.is_some() {
            self.disconnect().await?;
        }

        // Update config
        self.config.broker_host = host.to_string();
        self.config.broker_port = port;

        // Create MQTT options
        let mut mqttoptions = MqttOptions::new(&self.config.client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(60));
        mqttoptions.set_clean_session(true);

        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            mqttoptions.set_credentials(username, password);
        }

        // Create client and event loop
        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        
        // Test initial connection
        match timeout(Duration::from_secs(10), async {
            // Subscribe to topics
            client.subscribe("weather/data", QoS::AtMostOnce).await?;
            client.subscribe("weather/sensor_data", QoS::AtMostOnce).await?;
            client.subscribe("weather/alert_trigger", QoS::AtMostOnce).await?;
            
            // Wait for connection confirmation
            loop {
                match eventloop.poll().await {
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                        info!("MQTT connection established");
                        break;
                    }
                    Ok(Event::Incoming(Packet::SubAck(_))) => {
                        debug!("Subscription confirmed");
                    }
                    Ok(_) => continue,
                    Err(e) => return Err(anyhow!("MQTT connection error: {}", e)),
                }
            }
            Ok::<(), anyhow::Error>(())
        }).await {
            Ok(_) => {
                self.client = Some(client.clone());
                self.connected = true;
                
                // Start persistent event loop in background
                let weather_data = Arc::clone(&self.latest_weather_data);
                let sensor_data = Arc::clone(&self.latest_sensor_data);
                let app_handle = self.app_handle.clone();
                
                let handle = tokio::spawn(async move {
                    info!("Starting MQTT event loop");
                    loop {
                        match eventloop.poll().await {
                            Ok(Event::Incoming(Packet::Publish(publish))) => {
                                Self::handle_message_static(&publish.topic, &publish.payload, &weather_data, &sensor_data, &app_handle).await;
                            }
                            Ok(_) => continue,
                            Err(e) => {
                                error!("MQTT event loop error: {}", e);
                                break;
                            }
                        }
                    }
                    info!("MQTT event loop ended");
                });
                
                self.event_loop_handle = Some(handle);
                info!("MQTT client connected successfully");
                Ok(())
            }
            Err(_) => {
                error!("MQTT connection timeout");
                Err(anyhow!("Connection timeout after 10 seconds"))
            }
        }
    }

    async fn handle_message_static(
        topic: &str, 
        payload: &[u8], 
        weather_data: &Arc<Mutex<Option<WeatherData>>>, 
        sensor_data: &Arc<Mutex<Option<SensorData>>>,
        app_handle: &Option<AppHandle>
    ) {
        debug!("Received message on topic: {}", topic);
        
        match topic {
            "weather/data" => {
                match serde_json::from_slice::<WeatherData>(payload) {
                    Ok(weather) => {
                        info!("Received weather data update");
                        let mut data = weather_data.lock().await;
                        *data = Some(weather);
                    }
                    Err(e) => {
                        error!("Failed to parse weather data: {}", e);
                    }
                }
            }
            "weather/sensor_data" => {
                match serde_json::from_slice::<SensorData>(payload) {
                    Ok(sensor) => {
                        println!("M5Go Sensor Data: Temperature: {}°C, Humidity: {}%, Pressure: {} hPa, Timestamp: {}", 
                                sensor.temperature, sensor.humidity, sensor.pressure, sensor.timestamp);
                        info!("Received sensor data update");
                        
                        // Update stored data
                        let mut data = sensor_data.lock().await;
                        *data = Some(sensor.clone());
                        
                        // Emit event to frontend
                        if let Some(handle) = app_handle {
                            info!("Emitting sensor-data-updated event to frontend");
                            // Try to emit to all windows
                            match handle.emit_to("main", "sensor-data-updated", &sensor) {
                                Ok(_) => info!("Successfully emitted sensor data event to main window"),
                                Err(e) => {
                                    error!("Failed to emit sensor data event to main window: {}", e);
                                    // Fallback to global emit
                                    if let Err(e2) = handle.emit("sensor-data-updated", &sensor) {
                                        error!("Failed to emit sensor data event globally: {}", e2);
                                    } else {
                                        info!("Successfully emitted sensor data event globally");
                                    }
                                }
                            }
                        } else {
                            warn!("No app handle available to emit sensor data event");
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse sensor data: {}", e);
                    }
                }
            }
            "weather/alert_trigger" => {
                match serde_json::from_slice::<AlertData>(payload) {
                    Ok(alert_data) => {
                        info!("Received alert: {}", alert_data.message);
                        // Handle alert (could emit to frontend)
                    }
                    Err(e) => {
                        error!("Failed to parse alert data: {}", e);
                    }
                }
            }
            _ => {
                warn!("Received message on unknown topic: {}", topic);
            }
        }
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting from MQTT broker");
        
        // Abort the event loop task
        if let Some(handle) = self.event_loop_handle.take() {
            handle.abort();
        }
        
        // Abort the weather publishing task
        if let Some(handle) = self.weather_publish_handle.take() {
            handle.abort();
            info!("Automated weather publishing stopped due to disconnect");
        }
        
        // Disconnect the client
        if let Some(client) = &self.client {
            client.disconnect().await?;
            self.client = None;
        }
        
        self.connected = false;
        info!("MQTT client disconnected");
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn publish_weather_data(&self, data: &WeatherData) -> Result<()> {
        if let Some(client) = &self.client {
            let payload = serde_json::to_vec(data)?;
            
            // Print payload before sending
            // match serde_json::to_string_pretty(data) {
            //     Ok(json_str) => {
            //         println!("Publishing weather data payload:\n{}", json_str);
            //     }
            //     Err(e) => {
            //         warn!("Failed to serialize weather data for printing: {}", e);
            //         println!("Publishing weather data payload: {:?}", data);
            //     }
            // }
            
            client.publish("weather/data", QoS::AtMostOnce, false, payload).await?;
            info!("Published weather data to MQTT");
            Ok(())
        } else {
            Err(anyhow!("MQTT client not connected"))
        }
    }

    pub async fn send_alert(&self, alert: &AlertData) -> Result<()> {
        if let Some(client) = &self.client {
            let payload = serde_json::to_vec(alert)?;
            client.publish("weather/alert_trigger", QoS::AtMostOnce, false, payload).await?;
            info!("Published alert to MQTT: {}", alert.message);
            // Print payload before sending
            match serde_json::to_string_pretty(alert) {
                Ok(json_str) => {
                    println!("Publishing weather data payload:\n{}", json_str);
                }
                Err(e) => {
                    warn!("Failed to serialize weather data for printing: {}", e);
                    println!("Publishing weather data payload: {:?}", alert);
                }
            }
            Ok(())
        } else {
            Err(anyhow!("MQTT client not connected"))
        }
    }


    pub async fn get_latest_weather_data(&self) -> Option<WeatherData> {
        let data = self.latest_weather_data.lock().await;
        data.clone()
    }

    pub async fn get_latest_sensor_data(&self) -> Option<SensorData> {
        let data = self.latest_sensor_data.lock().await;
        data.clone()
    }

    pub async fn start_automated_weather_publishing(&mut self, lat: f64, lon: f64) -> Result<()> {
        if self.weather_publish_handle.is_some() {
            info!("Automated weather publishing is already running");
            return Ok(());
        }

        if !self.connected {
            return Err(anyhow!("MQTT client not connected"));
        }

        let client = self.client.as_ref().ok_or_else(|| anyhow!("MQTT client not available"))?.clone();
        let weather_api_client = Arc::clone(&self.weather_api_client);
        
        info!("Starting automated weather publishing every 5 seconds for coordinates: {}, {}", lat, lon);
        
        let weather_data_arc = Arc::clone(&self.latest_weather_data);
        let app_handle = self.app_handle.clone();
        
        let handle = tokio::spawn(async move {
            // Ensure we have cached data for today
            info!("Ensuring daily weather cache is available...");
            if let Err(e) = weather_api_client.ensure_daily_cache(lat, lon).await {
                error!("Failed to ensure daily cache: {}", e);
                return;
            }

            let mut interval = interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                // Read from cache file only - never call API
                match weather_api_client.read_cached_weather_only(lat, lon).await {
                    Ok(Some(weather_data)) => {
                        // Store the weather data in memory
                        {
                            let mut stored_data = weather_data_arc.lock().await;
                            *stored_data = Some(weather_data.clone());
                        }
                        
                        // Print payload before sending
                        match serde_json::to_string_pretty(&weather_data) {
                            Ok(json_str) => {
                                println!("Publishing cached weather data from file:\n{}", json_str);
                            }
                            Err(e) => {
                                warn!("Failed to serialize weather data for printing: {}", e);
                                println!("Publishing cached weather data from file: {:?}", weather_data);
                            }
                        }
                        
                        // Publish to MQTT
                        match serde_json::to_vec(&weather_data) {
                            Ok(payload) => {
                                match client.publish("weather/data", QoS::AtMostOnce, false, payload).await {
                                    Ok(_) => {
                                        info!("Published weather data from cache file to MQTT");
                                        
                                        // Emit event to frontend if app handle is available
                                        if let Some(handle) = &app_handle {
                                            if let Err(e) = handle.emit("weather-data-updated", &weather_data) {
                                                warn!("Failed to emit weather data event: {}", e);
                                            } else {
                                                debug!("Emitted weather data event to frontend");
                                            }
                                        }
                                    },
                                    Err(e) => error!("Failed to publish weather data: {}", e),
                                }
                            }
                            Err(e) => error!("Failed to serialize weather data: {}", e),
                        }
                    }
                    Ok(None) => {
                        warn!("No cached weather data available - cache may have expired");
                        
                        // Try to refresh cache once
                        info!("Attempting to refresh weather cache...");
                        if let Err(e) = weather_api_client.ensure_daily_cache(lat, lon).await {
                            error!("Failed to refresh cache: {}", e);
                        } else {
                            info!("Cache refreshed successfully");
                        }
                    }
                    Err(e) => {
                        error!("Failed to read cached weather data: {}", e);
                    }
                }
            }
        });
        
        self.weather_publish_handle = Some(handle);
        info!("Automated weather publishing started");
        Ok(())
    }

    pub async fn stop_automated_weather_publishing(&mut self) -> Result<()> {
        if let Some(handle) = self.weather_publish_handle.take() {
            handle.abort();
            info!("Automated weather publishing stopped");
            Ok(())
        } else {
            info!("Automated weather publishing was not running");
            Ok(())
        }
    }

    pub fn is_auto_publishing(&self) -> bool {
        self.weather_publish_handle.is_some()
    }
}