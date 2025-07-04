<script lang="ts">
  import { currentWeatherData, currentSensorData, mqttConnected } from '$lib/stores';
  import { getLatestWeatherData, getSensorData, publishWeatherData } from '$lib/tauri';
  import { showMessage } from '$lib/stores';

  let isRefreshing = false;

  async function refreshData() {
    if (!$mqttConnected) {
      showMessage('MQTT not connected', true);
      return;
    }

    isRefreshing = true;
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

      showMessage('Data refreshed from MQTT', false);
    } catch (error) {
      showMessage(`Failed to refresh data: ${error}`, true);
    } finally {
      isRefreshing = false;
    }
  }

  async function republishWeatherData() {
    if (!$mqttConnected || !$currentWeatherData) {
      showMessage('MQTT not connected or no weather data available', true);
      return;
    }

    try {
      await publishWeatherData($currentWeatherData);
      showMessage('Weather data republished to MQTT', false);
    } catch (error) {
      showMessage(`Failed to republish data: ${error}`, true);
    }
  }

  function formatTimestamp(timestamp: string): string {
    return new Date(timestamp).toLocaleString();
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h1 class="text-2xl font-bold text-gray-900 mb-2">MQTT Control</h1>
    <p class="text-gray-600">Monitor and control MQTT data flow with your IoT weather station</p>
  </div>

  <!-- MQTT Status and Controls -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">MQTT Operations</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div class="text-center p-4 bg-gray-50 rounded-lg">
        <div class="text-2xl mb-2">
          {$mqttConnected ? '🟢' : '🔴'}
        </div>
        <div class="font-medium">Connection Status</div>
        <div class="text-sm text-gray-600">
          {$mqttConnected ? 'Connected' : 'Disconnected'}
        </div>
      </div>

      <div class="text-center p-4 bg-gray-50 rounded-lg">
        <div class="text-2xl mb-2">📨</div>
        <div class="font-medium">Weather Data</div>
        <div class="text-sm text-gray-600">
          {$currentWeatherData ? 'Available' : 'No data'}
        </div>
      </div>

      <div class="text-center p-4 bg-gray-50 rounded-lg">
        <div class="text-2xl mb-2">📊</div>
        <div class="font-medium">Sensor Data</div>
        <div class="text-sm text-gray-600">
          {$currentSensorData ? 'Available' : 'No data'}
        </div>
      </div>
    </div>

    <div class="flex flex-wrap gap-4">
      <button
        on:click={refreshData}
        disabled={!$mqttConnected || isRefreshing}
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isRefreshing ? 'Refreshing...' : 'Refresh Data'}
      </button>

      <button
        on:click={republishWeatherData}
        disabled={!$mqttConnected || !$currentWeatherData}
        class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Republish Weather Data
      </button>
    </div>
  </div>

  <!-- Current Data Overview -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- Weather Data from MQTT -->
    <div class="bg-white rounded-lg shadow-md p-6">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">Latest Weather Data (MQTT)</h3>
      
      {#if $currentWeatherData}
        <div class="space-y-3">
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Temperature</span>
            <span class="font-medium">{$currentWeatherData.current_temp.toFixed(1)}°C</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Humidity</span>
            <span class="font-medium">{$currentWeatherData.humidity}%</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Pressure</span>
            <span class="font-medium">{$currentWeatherData.pressure} hPa</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Wind</span>
            <span class="font-medium">{$currentWeatherData.wind_speed.toFixed(1)} m/s {$currentWeatherData.wind_direction}</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Condition</span>
            <span class="font-medium capitalize">{$currentWeatherData.condition}</span>
          </div>
          
          <div class="flex justify-between items-center py-2">
            <span class="text-sm text-gray-600">Last Updated</span>
            <span class="font-medium text-xs">{formatTimestamp($currentWeatherData.timestamp)}</span>
          </div>
        </div>
      {:else}
        <div class="text-center py-8 text-gray-500">
          <div class="text-3xl mb-2">📭</div>
          <div>No weather data received</div>
          <div class="text-sm">
            {#if $mqttConnected}
              Waiting for data from MQTT...
            {:else}
              Connect to MQTT to receive data
            {/if}
          </div>
        </div>
      {/if}
    </div>

    <!-- Sensor Data from MQTT -->
    <div class="bg-white rounded-lg shadow-md p-6">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">Latest Sensor Data (MQTT)</h3>
      
      {#if $currentSensorData}
        <div class="space-y-3">
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Sensor Temperature</span>
            <span class="font-medium">{$currentSensorData.temperature.toFixed(1)}°C</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Sensor Humidity</span>
            <span class="font-medium">{$currentSensorData.humidity.toFixed(1)}%</span>
          </div>
          
          <div class="flex justify-between items-center py-2 border-b border-gray-100">
            <span class="text-sm text-gray-600">Sensor Pressure</span>
            <span class="font-medium">{$currentSensorData.pressure.toFixed(1)} hPa</span>
          </div>
          
          <div class="flex justify-between items-center py-2">
            <span class="text-sm text-gray-600">Last Updated</span>
            <span class="font-medium text-xs">{formatTimestamp($currentSensorData.timestamp)}</span>
          </div>
        </div>
      {:else}
        <div class="text-center py-8 text-gray-500">
          <div class="text-3xl mb-2">📊</div>
          <div>No sensor data received</div>
          <div class="text-sm">
            {#if $mqttConnected}
              Waiting for sensor data from IoT device...
            {:else}
              Connect to MQTT to receive sensor data
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- MQTT Topics Information -->
  <div class="bg-gray-50 border border-gray-200 rounded-lg p-6">
    <h3 class="text-lg font-medium text-gray-800 mb-3">MQTT Topics</h3>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
      <div>
        <div class="font-medium text-gray-700 mb-1">📡 weather/data</div>
        <div class="text-gray-600">Weather information from API</div>
      </div>
      
      <div>
        <div class="font-medium text-gray-700 mb-1">📊 weather/sensor_data</div>
        <div class="text-gray-600">Local sensor readings from device</div>
      </div>
      
      <div>
        <div class="font-medium text-gray-700 mb-1">⚠️ weather/alert_trigger</div>
        <div class="text-gray-600">Weather alerts and notifications</div>
      </div>
    </div>
  </div>
</div>