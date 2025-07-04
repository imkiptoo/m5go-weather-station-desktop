import { invoke } from '@tauri-apps/api/core';

export interface WeatherData {
  location: string;
  gps_lat: number;
  gps_lon: number;
  condition: string;
  current_icon: string;
  wind_speed: number;
  wind_direction: string;
  current_temp: number;
  humidity: number;
  pressure: number;
  forecast: ForecastDay[];
  history: HistoryDay[];
  timestamp: string;
}

export interface ForecastDay {
  day: string;
  date: string;
  temp: number;
  humidity: number;
  icon: string;
}

export interface HistoryDay {
  day: string;
  date: string;
  temp: number;
  humidity: number;
}

export interface SensorData {
  temperature: number;
  humidity: number;
  pressure: number;
  timestamp: string;
}

export type AlertLevel = 'info' | 'warning' | 'emergency';

export interface AlertData {
  message: string;
  level: AlertLevel;
  timestamp: string;
}

export interface AppConfig {
  mqtt: MqttSettings;
  weather_api: WeatherApiSettings;
  app: AppSettings;
}

export interface MqttSettings {
  broker_host: string;
  broker_port: number;
  username?: string;
  password?: string;
  client_id: string;
  auto_connect: boolean;
}

export interface WeatherApiSettings {
  api_key: string;
  latitude: number;
  longitude: number;
  auto_fetch_interval_minutes: number;
}

export interface AppSettings {
  auto_refresh_data: boolean;
  desktop_notifications: boolean;
  dark_mode: boolean;
  data_refresh_interval_seconds: number;
}

// MQTT Commands
export async function connectMqtt(brokerHost: string, brokerPort: number): Promise<string> {
  return await invoke('connect_mqtt', { brokerHost, brokerPort });
}

export async function disconnectMqtt(): Promise<string> {
  return await invoke('disconnect_mqtt');
}

export async function getMqttStatus(): Promise<boolean> {
  return await invoke('get_mqtt_status');
}

export async function publishWeatherData(data: WeatherData): Promise<string> {
  return await invoke('publish_weather_data', { data });
}

export async function getLatestWeatherData(): Promise<WeatherData | null> {
  return await invoke('get_latest_weather_data');
}

export async function getSensorData(): Promise<SensorData | null> {
  return await invoke('get_sensor_data');
}

export async function sendAlert(message: string, level: AlertLevel = 'info'): Promise<string> {
  return await invoke('send_alert', { message, level });
}

// Weather API Commands
export async function fetchWeatherApi(lat: number, lon: number, apiKey: string): Promise<WeatherData> {
  return await invoke('fetch_weather_api', { lat, lon, apiKey });
}

export async function fetchWeatherWithDefaultKey(lat: number, lon: number): Promise<WeatherData> {
  return await invoke('fetch_weather_with_default_key', { lat, lon });
}

// Configuration Commands
export async function getConfig(): Promise<AppConfig> {
  return await invoke('get_config');
}

export async function saveConfig(config: AppConfig): Promise<string> {
  return await invoke('save_config', { config });
}

export async function saveMqttSettings(mqttSettings: MqttSettings): Promise<string> {
  return await invoke('save_mqtt_settings', { mqttSettings });
}

export async function saveWeatherApiSettings(weatherApiSettings: WeatherApiSettings): Promise<string> {
  return await invoke('save_weather_api_settings', { weatherApiSettings });
}

export async function saveAppSettings(appSettings: AppSettings): Promise<string> {
  return await invoke('save_app_settings', { appSettings });
}

export async function testEmitSensorData(): Promise<string> {
  return await invoke('test_emit_sensor_data');
}

// Automated Weather Publishing Commands
export async function startAutomatedWeatherPublishing(lat: number, lon: number): Promise<string> {
  return await invoke('start_automated_weather_publishing', { lat, lon });
}

export async function stopAutomatedWeatherPublishing(): Promise<string> {
  return await invoke('stop_automated_weather_publishing');
}

export async function isAutoPublishing(): Promise<boolean> {
  return await invoke('is_auto_publishing');
}

export async function refreshWeatherCache(lat: number, lon: number): Promise<string> {
  return await invoke('refresh_weather_cache', { lat, lon });
}