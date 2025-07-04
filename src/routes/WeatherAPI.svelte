<script lang="ts">
  import { fetchWeatherApi, fetchWeatherWithDefaultKey, publishWeatherData, startAutomatedWeatherPublishing, stopAutomatedWeatherPublishing, isAutoPublishing, refreshWeatherCache } from '$lib/tauri';
  import { currentWeatherData, apiConfig, mqttConnected, showMessage } from '$lib/stores';
  import WeatherCard from '$lib/WeatherCard.svelte';
  import { onMount } from 'svelte';

  let apiKey = '959aa734172f631b6ceb521badee9dbf'; // Hard-coded in Rust backend
  let latitude = 48.7758;
  let longitude = 9.1829;
  let isLoading = false;
  let autoPublishing = false;

  async function fetchWeather() {
    isLoading = true;
    try {
      // Use the backend API key instead of the frontend input
      const weatherData = await fetchWeatherWithDefaultKey(latitude, longitude);
      currentWeatherData.set(weatherData);
      showMessage('Weather data fetched successfully from API (using backend API key)', false);
      
      // Optionally publish to MQTT if connected
      if ($mqttConnected) {
        try {
          await publishWeatherData(weatherData);
          showMessage('Weather data published to MQTT', false);
        } catch (error) {
          console.error('Failed to publish to MQTT:', error);
        }
      }
    } catch (error) {
      showMessage(`Failed to fetch weather data: ${error}`, true);
    } finally {
      isLoading = false;
    }
  }

  async function startAutomatedPublishing() {
    if (!$mqttConnected) {
      showMessage('MQTT not connected', true);
      return;
    }

    try {
      await startAutomatedWeatherPublishing(latitude, longitude);
      autoPublishing = true;
      showMessage('Started automated weather publishing every 5 seconds', false);
    } catch (error) {
      showMessage(`Failed to start automated publishing: ${error}`, true);
    }
  }

  async function stopAutomatedPublishing() {
    try {
      await stopAutomatedWeatherPublishing();
      autoPublishing = false;
      showMessage('Stopped automated weather publishing', false);
    } catch (error) {
      showMessage(`Failed to stop automated publishing: ${error}`, true);
    }
  }

  async function checkAutoPublishingStatus() {
    try {
      autoPublishing = await isAutoPublishing();
    } catch (error) {
      console.error('Failed to check auto-publishing status:', error);
    }
  }

  async function refreshCache() {
    try {
      await refreshWeatherCache(latitude, longitude);
      showMessage('Weather cache refreshed successfully', false);
    } catch (error) {
      showMessage(`Failed to refresh cache: ${error}`, true);
    }
  }

  // Check auto-publishing status on mount and when MQTT connection changes
  onMount(() => {
    checkAutoPublishingStatus();
  });

  $: if ($mqttConnected !== undefined) {
    checkAutoPublishingStatus();
  }

  // Save config
  $: {
    if (apiKey !== undefined && latitude !== undefined && longitude !== undefined) {
      apiConfig.set({ 
        api_key: apiKey, 
        latitude, 
        longitude, 
        auto_fetch_interval_minutes: 30 
      });
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h1 class="text-2xl font-bold text-gray-900 mb-2">Weather API</h1>
    <p class="text-gray-600">Fetch weather data from OpenWeatherMap and publish to IoT device</p>
  </div>

  <!-- API Configuration -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">API Configuration</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-4">
        <div>
          <label for="api-key" class="block text-sm font-medium text-gray-700 mb-1">
            OpenWeatherMap API Key
          </label>
          <input
            id="api-key"
            type="password"
            bind:value={apiKey}
            disabled
            class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-100 cursor-not-allowed"
            placeholder="API key configured in backend"
          />
          <p class="text-xs text-gray-500 mt-1">
            API key (959aa734172f631b6ceb521badee9dbf) is hard-coded in the Rust backend
          </p>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label for="latitude" class="block text-sm font-medium text-gray-700 mb-1">
              Latitude
            </label>
            <input
              id="latitude"
              type="number"
              step="0.0001"
              bind:value={latitude}
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="48.7758"
            />
          </div>

          <div>
            <label for="longitude" class="block text-sm font-medium text-gray-700 mb-1">
              Longitude
            </label>
            <input
              id="longitude"
              type="number"
              step="0.0001"
              bind:value={longitude}
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="9.1829"
            />
          </div>
        </div>

        <button
          on:click={fetchWeather}
          disabled={isLoading}
          class="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isLoading ? 'Fetching Weather...' : 'Fetch Weather Data (Backend API)'}
        </button>

        <div class="grid grid-cols-2 gap-2">
          <button
            on:click={startAutomatedPublishing}
            disabled={!$mqttConnected || autoPublishing}
            class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {autoPublishing ? 'Publishing...' : 'Start Auto-Publish'}
          </button>

          <button
            on:click={stopAutomatedPublishing}
            disabled={!autoPublishing}
            class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Stop Auto-Publish
          </button>
        </div>

        <button
          on:click={refreshCache}
          class="w-full px-4 py-2 bg-yellow-600 text-white rounded-md hover:bg-yellow-700 focus:outline-none focus:ring-2 focus:ring-yellow-500"
        >
          Refresh Cache Manually
        </button>
      </div>

      <div class="bg-gray-50 p-4 rounded-lg">
        <h3 class="font-medium text-gray-800 mb-2">Current Location</h3>
        <div class="text-sm text-gray-600 space-y-1">
          <div>📍 Latitude: {latitude}</div>
          <div>📍 Longitude: {longitude}</div>
          <div class="mt-3 pt-3 border-t border-gray-200">
            <div class="font-medium text-gray-700 mb-1">Auto-Publishing Status:</div>
            <div class="text-xs space-y-1">
              <div class="flex items-center gap-2">
                {#if autoPublishing}
                  <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span class="text-green-600">Publishing every 5 seconds (Rust backend)</span>
                {:else}
                  <div class="w-2 h-2 bg-gray-400 rounded-full"></div>
                  <span class="text-gray-600">Stopped</span>
                {/if}
              </div>
              <div class="text-xs text-gray-500">
                {#if !$mqttConnected}
                  (Connect to MQTT to enable auto-publishing)
                {/if}
              </div>
            </div>
          </div>
          <div class="mt-3 pt-3 border-t border-gray-200">
            <div class="font-medium text-gray-700 mb-1">Features:</div>
            <ul class="text-xs space-y-1">
              <li>• Current weather conditions</li>
              <li>• 5-day forecast</li>
              <li>• Historical temperature data</li>
              <li>• Wind speed and direction</li>
              <li>• Humidity and pressure</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Weather Data Display -->
  <WeatherCard weatherData={$currentWeatherData} title="API Weather Data" />

  <!-- API Information -->
  <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
    <h3 class="text-lg font-medium text-blue-800 mb-2">How it works</h3>
    <div class="text-sm text-blue-700 space-y-2">
      <p>1. API key is hard-coded in the Rust backend (959aa734172f631b6ceb521badee9dbf)</p>
      <p>2. Weather data is cached daily - API is only called once per day</p>
      <p>3. Auto-publishing reads cached data from file every 5 seconds (no API calls)</p>
      <p>4. Set your GPS coordinates (latitude and longitude)</p>
      <p>5. Use "Start Auto-Publish" to read and send cached data every 5 seconds</p>
      <p>6. Use "Refresh Cache Manually" to force update cache with fresh API data</p>
      <p>7. Cache is stored in weather_cache.json in your user data directory</p>
    </div>
  </div>
</div>