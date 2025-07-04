<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { activeTab, currentWeatherData, currentSensorData, loadConfig } from '$lib/stores';
  import WeatherCard from '$lib/WeatherCard.svelte';
  import ConnectionStatus from '$lib/ConnectionStatus.svelte';
  import Dashboard from './Dashboard.svelte';
  import WeatherAPI from './WeatherAPI.svelte';
  import MQTTControl from './MQTTControl.svelte';
  import Alerts from './Alerts.svelte';
  import Settings from './Settings.svelte';
  import { getLatestWeatherData, getSensorData } from '$lib/tauri';
  import { listen } from '@tauri-apps/api/event';
  import type { SensorData } from '$lib/tauri';

  // Update data periodically
  async function updateData() {
    try {
      const [weatherData, sensorData] = await Promise.all([
        getLatestWeatherData(),
        getSensorData()
      ]);
      
      if (weatherData) {
        currentWeatherData.set(weatherData);
      }
      if (sensorData) {
        currentSensorData.set(sensorData);
      }
    } catch (error) {
      console.error('Failed to update data:', error);
    }
  }

  let sensorDataUnlisten: (() => void) | null = null;

  onMount(async () => {
    // Load configuration first
    await loadConfig();
    
    // Initial data load
    updateData();
    
    // Update every 30 seconds (fallback)
    const interval = setInterval(updateData, 30000);
    
    // Listen for real-time sensor data updates
    try {
      console.log('Setting up sensor data event listener...');
      sensorDataUnlisten = await listen<SensorData>('sensor-data-updated', (event) => {
        console.log('🚀 Real-time sensor data received:', event.payload);
        console.log('Updating currentSensorData store with:', event.payload);
        currentSensorData.set(event.payload);
        console.log('✅ Store updated successfully');
      });
      console.log('✅ Sensor data listener setup successfully');
    } catch (error) {
      console.error('❌ Failed to setup sensor data listener:', error);
    }
    
    return () => {
      clearInterval(interval);
      if (sensorDataUnlisten) {
        sensorDataUnlisten();
      }
    };
  });

  onDestroy(() => {
    if (sensorDataUnlisten) {
      sensorDataUnlisten();
    }
  });
</script>

{#if $activeTab === 'dashboard'}
  <Dashboard />
{:else if $activeTab === 'weather'}
  <WeatherAPI />
{:else if $activeTab === 'mqtt'}
  <MQTTControl />
{:else if $activeTab === 'alerts'}
  <Alerts />
{:else if $activeTab === 'settings'}
  <Settings />
{/if}