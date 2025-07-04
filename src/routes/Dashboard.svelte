<script lang="ts">
  import { currentWeatherData, currentSensorData, mqttConnected } from '$lib/stores';
  import WeatherCard from '$lib/WeatherCard.svelte';
  import ConnectionStatus from '$lib/ConnectionStatus.svelte';
  import { testEmitSensorData } from '$lib/tauri';

  function formatDate(timestamp: string): string {
    return new Date(timestamp).toLocaleString();
  }

  // Debug store changes
  $: if ($currentSensorData) {
    console.log('📊 Dashboard: currentSensorData updated:', $currentSensorData);
  }

  async function handleTestEvent() {
    try {
      console.log('🧪 Testing sensor data event emission...');
      const result = await testEmitSensorData();
      console.log('✅ Test result:', result);
    } catch (error) {
      console.error('❌ Test failed:', error);
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <div class="flex justify-between items-center mb-2">
      <h1 class="text-2xl font-bold text-gray-900">Weather Station Dashboard</h1>
      <button 
        on:click={handleTestEvent}
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 text-sm"
      >
        Test Event
      </button>
    </div>
    <p class="text-gray-600">Real-time monitoring and control of your IoT weather station</p>
  </div>

  <!-- Connection Status -->
  <ConnectionStatus />

  <!-- Weather Data Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
    <!-- MQTT Weather Data -->
    <WeatherCard weatherData={$currentWeatherData} title="MQTT Weather Data" />

    <!-- Local Sensor Data -->
    <div class="bg-white rounded-lg shadow-md p-6">
      <h2 class="text-xl font-semibold text-gray-800 mb-4">Local Sensor Data</h2>
      
      {#if $currentSensorData}
        <div class="grid grid-cols-1 gap-4">
          <div class="bg-blue-50 p-4 rounded-lg">
            <div class="flex items-center justify-between">
              <div>
                <div class="text-sm font-medium text-blue-600">Temperature</div>
                <div class="text-2xl font-bold text-blue-900">
                  {$currentSensorData.temperature.toFixed(1)}°C
                </div>
              </div>
              <div class="text-3xl">🌡️</div>
            </div>
          </div>

          <div class="bg-green-50 p-4 rounded-lg">
            <div class="flex items-center justify-between">
              <div>
                <div class="text-sm font-medium text-green-600">Humidity</div>
                <div class="text-2xl font-bold text-green-900">
                  {$currentSensorData.humidity.toFixed(1)}%
                </div>
              </div>
              <div class="text-3xl">💧</div>
            </div>
          </div>

          <div class="bg-purple-50 p-4 rounded-lg">
            <div class="flex items-center justify-between">
              <div>
                <div class="text-sm font-medium text-purple-600">Pressure</div>
                <div class="text-2xl font-bold text-purple-900">
                  {$currentSensorData.pressure.toFixed(1)} hPa
                </div>
              </div>
              <div class="text-3xl">📊</div>
            </div>
          </div>

          <div class="text-xs text-gray-500 mt-2">
            Last updated: {formatDate($currentSensorData.timestamp)}
          </div>
        </div>
      {:else}
        <div class="text-center py-8 text-gray-500">
          <div class="text-4xl mb-2">📡</div>
          <div>No sensor data available</div>
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

  <!-- Quick Stats -->
  <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
    <div class="bg-white rounded-lg shadow-md p-4">
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <div class="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
            <span class="text-blue-600 text-sm">📡</span>
          </div>
        </div>
        <div class="ml-3">
          <div class="text-sm font-medium text-gray-500">MQTT Status</div>
          <div class="text-lg font-semibold {$mqttConnected ? 'text-green-600' : 'text-red-600'}">
            {$mqttConnected ? 'Connected' : 'Disconnected'}
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-lg shadow-md p-4">
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <div class="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center">
            <span class="text-green-600 text-sm">🌡️</span>
          </div>
        </div>
        <div class="ml-3">
          <div class="text-sm font-medium text-gray-500">Current Temp</div>
          <div class="text-lg font-semibold text-gray-900">
            {$currentWeatherData ? $currentWeatherData.current_temp.toFixed(1) + '°C' : '--'}
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-lg shadow-md p-4">
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <div class="w-8 h-8 bg-yellow-100 rounded-full flex items-center justify-center">
            <span class="text-yellow-600 text-sm">💧</span>
          </div>
        </div>
        <div class="ml-3">
          <div class="text-sm font-medium text-gray-500">Humidity</div>
          <div class="text-lg font-semibold text-gray-900">
            {$currentWeatherData ? $currentWeatherData.humidity + '%' : '--'}
          </div>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-lg shadow-md p-4">
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <div class="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center">
            <span class="text-purple-600 text-sm">📊</span>
          </div>
        </div>
        <div class="ml-3">
          <div class="text-sm font-medium text-gray-500">Pressure</div>
          <div class="text-lg font-semibold text-gray-900">
            {$currentWeatherData ? $currentWeatherData.pressure + ' hPa' : '--'}
          </div>
        </div>
      </div>
    </div>
  </div>
</div>