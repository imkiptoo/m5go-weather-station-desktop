use crate::types::*;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use tracing::{info, error, warn};
use chrono::{Utc, DateTime, Local, Datelike};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

const OPENWEATHERMAP_API_KEY: &str = "959aa734172f631b6ceb521badee9dbf";
const CACHE_FILE_NAME: &str = "weather_cache.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct WeatherCache {
    pub data: WeatherData,
    pub last_updated: DateTime<Local>,
    pub coordinates: (f64, f64), // (lat, lon)
}

pub struct WeatherApiClient {
    client: Client,
    cache_path: PathBuf,
}

impl WeatherApiClient {
    pub fn new() -> Self {
        let cache_path = Self::get_cache_path();
        Self {
            client: Client::new(),
            cache_path,
        }
    }

    fn get_cache_path() -> PathBuf {
        // Use the same directory as the config file
        let mut path = dirs::data_dir()
            .or_else(|| dirs::home_dir())
            .unwrap_or_else(|| PathBuf::from("."));
        
        path.push("weather-station-desktop");
        
        // Create directory if it doesn't exist
        if let Err(e) = fs::create_dir_all(&path) {
            warn!("Failed to create cache directory: {}", e);
        }
        
        path.push(CACHE_FILE_NAME);
        path
    }

    pub async fn fetch_weather(&self, lat: f64, lon: f64, api_key: &str) -> Result<WeatherData> {
        info!("🌤️  CALLING OPENWEATHERMAP API!");
        info!("API Key: {}", api_key);
        info!("Fetching weather data for coordinates: {}, {}", lat, lon);

        // Using OpenWeatherMap One Call API 3.0
        let url = format!(
            "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}&units=metric&exclude=minutely,hourly,alerts",
            lat, lon, api_key
        );

        info!("Making API request to: {}", url);

        let response = self.client.get(&url).send().await?;
        
        info!("API response status: {}", response.status());
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            error!("API request failed with status {}: {}", status, error_text);
            return Err(anyhow!("API request failed: {} - {}", status, error_text));
        }

        let data: Value = response.json().await?;
        info!("✅ SUCCESSFULLY RECEIVED API RESPONSE");
        
        // LOG THE COMPLETE API RESPONSE
        let pretty_json = serde_json::to_string_pretty(&data)?;
        info!("📋 COMPLETE API RESPONSE:\n{}", pretty_json);
        
        // Save raw API response to a file for debugging
        let mut debug_path = self.cache_path.clone();
        debug_path.set_file_name("api_response_debug.json");
        if let Err(e) = fs::write(&debug_path, &pretty_json) {
            warn!("Failed to save debug API response: {}", e);
        } else {
            info!("💾 Saved raw API response to: {:?}", debug_path);
        }

        self.parse_weather_response(data, lat, lon).await
    }

    pub async fn fetch_weather_with_default_key(&self, lat: f64, lon: f64) -> Result<WeatherData> {
        info!("🔍 Checking cache for weather data...");
        
        // Check cache first
        if let Some(cached_data) = self.get_cached_weather(lat, lon).await? {
            info!("📄 USING CACHED WEATHER DATA - NO API CALL");
            return Ok(cached_data);
        }

        // If cache is expired or missing, fetch from API
        info!("💾 Cache expired or missing, FETCHING FROM API");
        let weather_data = self.fetch_weather(lat, lon, OPENWEATHERMAP_API_KEY).await?;
        
        // Cache the new data
        info!("💾 Caching fresh weather data...");
        if let Err(e) = self.cache_weather_data(&weather_data, lat, lon).await {
            warn!("Failed to cache weather data: {}", e);
        } else {
            info!("✅ Weather data cached successfully");
        }

        Ok(weather_data)
    }

    async fn get_cached_weather(&self, lat: f64, lon: f64) -> Result<Option<WeatherData>> {
        info!("🔍 CHECKING CACHE at: {:?}", self.cache_path);
        
        if !self.cache_path.exists() {
            info!("❌ Cache file does not exist");
            return Ok(None);
        }

        match fs::read_to_string(&self.cache_path) {
            Ok(content) => {
                info!("📄 Cache file exists, size: {} bytes", content.len());
                match serde_json::from_str::<WeatherCache>(&content) {
                    Ok(cache) => {
                        info!("📅 Cache last updated: {}", cache.last_updated);
                        info!("📍 Cache coordinates: {:?}, requested: ({}, {})", cache.coordinates, lat, lon);
                        
                        // Check if coordinates match (within small tolerance)
                        let coord_match = (cache.coordinates.0 - lat).abs() < 0.001 
                            && (cache.coordinates.1 - lon).abs() < 0.001;
                        
                        if !coord_match {
                            info!("❌ Cache coordinates don't match, ignoring cache");
                            return Ok(None);
                        }

                        // Check if cache is from today
                        let now = Local::now();
                        let cache_date = cache.last_updated.date_naive();
                        let today = now.date_naive();
                        
                        info!("📅 Cache date: {}, Today: {}", cache_date, today);
                        
                        if cache_date == today {
                            info!("✅ Found VALID cache from today - USING CACHED DATA");
                            info!("📊 Cached weather data timestamp: {}", cache.data.timestamp);
                            Ok(Some(cache.data))
                        } else {
                            info!("⏰ Cache is from different day ({} vs {}), will refresh", cache_date, today);
                            Ok(None)
                        }
                    }
                    Err(e) => {
                        warn!("❌ Failed to parse cache file: {}", e);
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                warn!("❌ Failed to read cache file: {}", e);
                Ok(None)
            }
        }
    }

    async fn cache_weather_data(&self, data: &WeatherData, lat: f64, lon: f64) -> Result<()> {
        let cache = WeatherCache {
            data: data.clone(),
            last_updated: Local::now(),
            coordinates: (lat, lon),
        };

        let cache_json = serde_json::to_string_pretty(&cache)?;
        fs::write(&self.cache_path, &cache_json)?;
        
        info!("💾 WEATHER DATA CACHED successfully to: {:?}", self.cache_path);
        info!("📊 Cached data timestamp: {}", data.timestamp);
        info!("📍 Cached coordinates: ({}, {})", lat, lon);
        info!("📄 Cache file size: {} bytes", cache_json.len());
        Ok(())
    }

    pub async fn read_cached_weather_only(&self, lat: f64, lon: f64) -> Result<Option<WeatherData>> {
        // Only read from cache, never call API
        self.get_cached_weather(lat, lon).await
    }

    pub async fn ensure_daily_cache(&self, lat: f64, lon: f64) -> Result<()> {
        info!("🔄 ENSURING DAILY CACHE IS AVAILABLE");
        
        // Check if we need to update cache for today
        match self.get_cached_weather(lat, lon).await? {
            Some(_) => {
                info!("✅ Cache is valid for today, NO API CALL NEEDED");
                Ok(())
            }
            None => {
                info!("⚠️  Cache is missing or expired, UPDATING FROM API");
                let weather_data = self.fetch_weather(lat, lon, OPENWEATHERMAP_API_KEY).await?;
                self.cache_weather_data(&weather_data, lat, lon).await?;
                info!("✅ Daily cache updated successfully");
                Ok(())
            }
        }
    }


    fn generate_historical_data(&self, current_temp: f64, current_humidity: i32) -> Vec<HistoryDay> {
        use chrono::Datelike;
        
        info!("⚠️  Generating historical data with fixed temperature pattern");
        
        let mut history = Vec::new();
        let today = Utc::now().date_naive();
        
        // Fixed temperature pattern: 32, 31, 34, 28, 32, 27, ...
        let temp_pattern = [32.0, 31.0, 34.0, 28.0, 32.0, 27.0, 30.0];
        
        // Fixed humidity pattern: 85, 72, 68, 91, 76, 82, ...
        let humidity_pattern = [85, 72, 68, 91, 76, 82, 79];

        // Generate history for past 6 days (today-6 to today-1) + today
        for days_back in (0..7).rev() { // 6, 5, 4, 3, 2, 1, 0 (today)
            let historical_date = today - chrono::Duration::days(days_back);
            
            // Use fixed temperature pattern for historical days, current temp for today
            let historical_temp = if days_back == 0 {
                current_temp // Use actual current temperature for today
            } else {
                let pattern_index = (6 - days_back) as usize % temp_pattern.len();
                temp_pattern[pattern_index]
            };
            
            // Use fixed humidity pattern for historical days, current humidity for today
            let historical_humidity = if days_back == 0 {
                current_humidity // Use actual current humidity for today
            } else {
                let pattern_index = (6 - days_back) as usize % humidity_pattern.len();
                humidity_pattern[pattern_index]
            };
            
            // Format date as DD/MM
            let date_str = historical_date.format("%d/%m").to_string();
            
            // Calculate day name
            let day_name = if days_back == 0 {
                "TODAY".to_string()
            } else {
                match historical_date.weekday() {
                    chrono::Weekday::Mon => "MON",
                    chrono::Weekday::Tue => "TUE", 
                    chrono::Weekday::Wed => "WED",
                    chrono::Weekday::Thu => "THU",
                    chrono::Weekday::Fri => "FRI",
                    chrono::Weekday::Sat => "SAT",
                    chrono::Weekday::Sun => "SUN",
                }.to_string()
            };
            
            history.push(HistoryDay {
                day: day_name,
                date: date_str,
                temp: historical_temp,
                humidity: historical_humidity,
            });
            
            info!("Generated history day {}: {} {} - temp: {:.1}°C, humidity: {}%", 
                  6 - days_back, history.last().unwrap().day, history.last().unwrap().date, 
                  historical_temp, historical_humidity);
        }

        info!("Generated {} days of historical data with temp pattern {:?} and humidity pattern {:?}", history.len(), temp_pattern, humidity_pattern);
        history
    }

    async fn parse_weather_response(&self, data: Value, lat: f64, lon: f64) -> Result<WeatherData> {
        info!("🔧 PARSING WEATHER RESPONSE");
        
        let current = data.get("current")
            .ok_or_else(|| anyhow!("Missing current weather data"))?;
        info!("✅ Found current weather data");

        let daily = data.get("daily")
            .and_then(|d| d.as_array())
            .ok_or_else(|| anyhow!("Missing daily forecast data"))?;
        info!("✅ Found daily forecast data with {} entries", daily.len());

        let empty_vec = vec![];
        let _hourly = data.get("hourly")
            .and_then(|h| h.as_array())
            .unwrap_or(&empty_vec);

        // Parse current weather
        let current_temp = current.get("temp")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.0);

        let humidity = current.get("humidity")
            .and_then(|h| h.as_i64())
            .unwrap_or(0) as i32;

        let pressure = current.get("pressure")
            .and_then(|p| p.as_i64())
            .unwrap_or(0) as i32;

        let wind_speed = current.get("wind_speed")
            .and_then(|w| w.as_f64())
            .unwrap_or(0.0);

        let wind_deg = current.get("wind_deg")
            .and_then(|w| w.as_f64())
            .unwrap_or(0.0);

        let condition = current.get("weather")
            .and_then(|w| w.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|weather| weather.get("description"))
            .and_then(|desc| desc.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let current_icon = current.get("weather")
            .and_then(|w| w.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|weather| weather.get("icon"))
            .and_then(|icon| icon.as_str())
            .unwrap_or("unknown")
            .to_string();

        // Parse forecast
        let forecast = self.parse_forecast(daily)?;

        // Generate historical data (timemachine API requires paid subscription)
        let history = self.generate_historical_data(current_temp, humidity);

        let weather_data = WeatherData {
            location: format!("LAT: {:.4}, LON: {:.4}", lat, lon),
            gps_lat: lat,
            gps_lon: lon,
            condition: condition.clone(),
            current_icon: current_icon.clone(),
            wind_speed,
            wind_direction: self.wind_deg_to_direction(wind_deg),
            current_temp,
            humidity,
            pressure,
            forecast: forecast.clone(),
            history: history.clone(),
            timestamp: Utc::now(),
        };

        info!("✅ SUCCESSFULLY PARSED WEATHER DATA");
        info!("📊 Current: {}°C, {}, {}", current_temp, condition, current_icon);
        info!("📈 History entries: {}", history.len());
        info!("📅 Forecast entries: {}", forecast.len());
        info!("🕐 Data timestamp: {}", weather_data.timestamp);
        
        Ok(weather_data)
    }

    fn parse_forecast(&self, daily: &[Value]) -> Result<Vec<ForecastDay>> {
        let mut forecast = Vec::new();
        let today = Utc::now().date_naive();

        for (i, day_data) in daily.iter().take(6).enumerate() {
            // Map according to your requirements:
            // temp -> [].temp.max (use max temperature for the day)
            let temp = day_data.get("temp")
                .and_then(|t| t.get("max"))
                .and_then(|max_temp| max_temp.as_f64())
                .unwrap_or(0.0);

            // humidity -> humidity (direct mapping)
            let humidity = day_data.get("humidity")
                .and_then(|h| h.as_i64())
                .unwrap_or(0) as i32;

            // icon -> weather[0].icon (first weather object's icon)
            let icon = day_data.get("weather")
                .and_then(|w| w.as_array())
                .and_then(|arr| arr.first())
                .and_then(|weather| weather.get("icon"))
                .and_then(|icon| icon.as_str())
                .unwrap_or("unknown")
                .to_string();

            // dt -> convert for day name and date
            let dt = day_data.get("dt")
                .and_then(|dt| dt.as_i64())
                .unwrap_or(0);

            let (day_name, date) = if dt > 0 {
                if let Some(datetime) = chrono::DateTime::from_timestamp(dt, 0) {
                    let forecast_date = datetime.date_naive();
                    
                    // Date format: DD/MM (as requested: 31/12)
                    let date_str = datetime.format("%d/%m").to_string();
                    
                    // Day name: "TODAY" for today, else weekday abbreviation
                    let day_name = if forecast_date == today {
                        "TODAY".to_string()
                    } else {
                        // Calculate the actual day name based on weekday
                        match forecast_date.weekday() {
                            chrono::Weekday::Mon => "MON",
                            chrono::Weekday::Tue => "TUE", 
                            chrono::Weekday::Wed => "WED",
                            chrono::Weekday::Thu => "THU",
                            chrono::Weekday::Fri => "FRI",
                            chrono::Weekday::Sat => "SAT",
                            chrono::Weekday::Sun => "SUN",
                        }.to_string()
                    };
                    
                    (day_name, date_str)
                } else {
                    (format!("DAY{}", i + 1), "".to_string())
                }
            } else {
                (format!("DAY{}", i + 1), "".to_string())
            };

            info!("Parsed forecast day {}: {} {} - temp: {} (max), humidity: {}, icon: {}", 
                  i, day_name, date, temp, humidity, icon);

            forecast.push(ForecastDay {
                day: day_name,
                date,
                temp,
                humidity,
                icon,
            });
        }

        Ok(forecast)
    }


    fn wind_deg_to_direction(&self, deg: f64) -> String {
        let directions = [
            "N", "NNE", "NE", "ENE",
            "E", "ESE", "SE", "SSE",
            "S", "SSW", "SW", "WSW",
            "W", "WNW", "NW", "NNW"
        ];
        
        let index = ((deg + 11.25) / 22.5) as usize % 16;
        directions[index].to_string()
    }

}