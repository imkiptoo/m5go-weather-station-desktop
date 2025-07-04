import { writable } from 'svelte/store';
import type { WeatherData, SensorData, AppConfig, MqttSettings, WeatherApiSettings, AppSettings } from './tauri';
import { getConfig, saveConfig, saveMqttSettings, saveWeatherApiSettings, saveAppSettings } from './tauri';

// Connection status
export const mqttConnected = writable<boolean>(false);
export const apiConnected = writable<boolean>(false);

// Weather data
export const currentWeatherData = writable<WeatherData | null>(null);
export const currentSensorData = writable<SensorData | null>(null);

// UI state
export const activeTab = writable<string>('dashboard');
export const isLoading = writable<boolean>(false);
export const errorMessage = writable<string>('');
export const successMessage = writable<string>('');

// Configuration stores
export const appConfig = writable<AppConfig | null>(null);
export const mqttConfig = writable<MqttSettings | null>(null);
export const apiConfig = writable<WeatherApiSettings | null>(null);
export const appSettings = writable<AppSettings | null>(null);

// Load configuration from backend
export async function loadConfig() {
  try {
    const config = await getConfig();
    appConfig.set(config);
    mqttConfig.set(config.mqtt);
    apiConfig.set(config.weather_api);
    appSettings.set(config.app);
    console.log('Configuration loaded successfully');
  } catch (error) {
    console.error('Failed to load configuration:', error);
    showMessage('Failed to load configuration', true);
  }
}

// Save functions for each config section
export async function saveMqttConfig(config: MqttSettings) {
  try {
    await saveMqttSettings(config);
    mqttConfig.set(config);
    showMessage('MQTT settings saved', false);
  } catch (error) {
    console.error('Failed to save MQTT settings:', error);
    showMessage('Failed to save MQTT settings', true);
  }
}

export async function saveApiConfig(config: WeatherApiSettings) {
  try {
    await saveWeatherApiSettings(config);
    apiConfig.set(config);
    showMessage('Weather API settings saved', false);
  } catch (error) {
    console.error('Failed to save Weather API settings:', error);
    showMessage('Failed to save Weather API settings', true);
  }
}

export async function saveAppConfig(config: AppSettings) {
  try {
    await saveAppSettings(config);
    appSettings.set(config);
    showMessage('App settings saved', false);
  } catch (error) {
    console.error('Failed to save app settings:', error);
    showMessage('Failed to save app settings', true);
  }
}

// Auto-clear messages after 5 seconds
export function showMessage(message: string, isError: boolean = false) {
  if (isError) {
    errorMessage.set(message);
    setTimeout(() => errorMessage.set(''), 5000);
  } else {
    successMessage.set(message);
    setTimeout(() => successMessage.set(''), 5000);
  }
}