<script lang="ts">
  import { mqttConfig, apiConfig, appSettings, saveMqttConfig, saveApiConfig, saveAppConfig } from '$lib/stores';
  import type { MqttSettings, WeatherApiSettings, AppSettings } from '$lib/tauri';

  // Local state for form inputs
  let localMqttConfig: MqttSettings | null = null;
  let localApiConfig: WeatherApiSettings | null = null;
  let localAppSettings: AppSettings | null = null;
  let settingsSaved = false;

  // Initialize local configs when store values change
  $: if ($mqttConfig && !localMqttConfig) {
    localMqttConfig = { ...$mqttConfig };
  }
  
  $: if ($apiConfig && !localApiConfig) {
    localApiConfig = { ...$apiConfig };
  }
  
  $: if ($appSettings && !localAppSettings) {
    localAppSettings = { ...$appSettings };
  }

  async function saveMqttSettings() {
    if (localMqttConfig) {
      await saveMqttConfig(localMqttConfig);
      settingsSaved = true;
      setTimeout(() => settingsSaved = false, 3000);
    }
  }

  async function saveApiSettings() {
    if (localApiConfig) {
      await saveApiConfig(localApiConfig);
      settingsSaved = true;
      setTimeout(() => settingsSaved = false, 3000);
    }
  }

  async function saveApplicationSettings() {
    if (localAppSettings) {
      await saveAppConfig(localAppSettings);
      settingsSaved = true;
      setTimeout(() => settingsSaved = false, 3000);
    }
  }

  function resetToDefaults() {
    localMqttConfig = {
      broker_host: '192.168.137.1',
      broker_port: 1883,
      username: undefined,
      password: undefined,
      client_id: `weather-desktop-${Date.now()}`,
      auto_connect: true
    };
    
    localApiConfig = {
      api_key: '',
      latitude: 48.7758,
      longitude: 9.1829,
      auto_fetch_interval_minutes: 30
    };

    localAppSettings = {
      auto_refresh_data: true,
      desktop_notifications: false,
      dark_mode: false,
      data_refresh_interval_seconds: 30
    };
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h1 class="text-2xl font-bold text-gray-900 mb-2">Settings</h1>
    <p class="text-gray-600">Configure your weather station desktop application</p>
  </div>

  <!-- MQTT Settings -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">MQTT Configuration</h2>
    
    {#if localMqttConfig}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div>
        <label for="mqtt-host" class="block text-sm font-medium text-gray-700 mb-1">
          Broker Host
        </label>
        <input
          id="mqtt-host"
          type="text"
          bind:value={localMqttConfig.broker_host}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="192.168.137.1"
        />
        <p class="text-xs text-gray-500 mt-1">IP address or hostname of your MQTT broker</p>
      </div>

      <div>
        <label for="mqtt-port" class="block text-sm font-medium text-gray-700 mb-1">
          Broker Port
        </label>
        <input
          id="mqtt-port"
          type="number"
          bind:value={localMqttConfig.broker_port}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="1883"
        />
        <p class="text-xs text-gray-500 mt-1">Default MQTT port is 1883</p>
      </div>
    </div>
    
    <div class="mt-4">
      <label class="flex items-center space-x-2">
        <input
          type="checkbox"
          bind:checked={localMqttConfig.auto_connect}
          class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        />
        <span class="text-sm text-gray-700">Auto-connect to MQTT on startup</span>
      </label>
    </div>

    <div class="mt-4">
      <button
        on:click={saveMqttSettings}
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        💾 Save MQTT Settings
      </button>
    </div>
    {:else}
    <div class="text-center py-8 text-gray-500">
      <div>Loading MQTT configuration...</div>
    </div>
    {/if}
  </div>

  <!-- Weather API Settings -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Weather API Configuration</h2>
    
    {#if localApiConfig}
    <div class="space-y-4">
      <div>
        <label for="api-key-settings" class="block text-sm font-medium text-gray-700 mb-1">
          OpenWeatherMap API Key
        </label>
        <input
          id="api-key-settings"
          type="password"
          bind:value={localApiConfig.api_key}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter your API key"
        />
        <p class="text-xs text-gray-500 mt-1">
          Get your free API key from <a href="https://openweathermap.org/api" target="_blank" class="text-blue-600 hover:underline">OpenWeatherMap</a>
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label for="latitude-settings" class="block text-sm font-medium text-gray-700 mb-1">
            Default Latitude
          </label>
          <input
            id="latitude-settings"
            type="number"
            step="0.0001"
            bind:value={localApiConfig.latitude}
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="48.7758"
          />
          <p class="text-xs text-gray-500 mt-1">GPS latitude coordinate</p>
        </div>

        <div>
          <label for="longitude-settings" class="block text-sm font-medium text-gray-700 mb-1">
            Default Longitude
          </label>
          <input
            id="longitude-settings"
            type="number"
            step="0.0001"
            bind:value={localApiConfig.longitude}
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            placeholder="9.1829"
          />
          <p class="text-xs text-gray-500 mt-1">GPS longitude coordinate</p>
        </div>
      </div>
    </div>
    {:else}
    <div class="text-center py-8 text-gray-500">
      <div>Loading Weather API configuration...</div>
    </div>
    {/if}
  </div>

  <!-- Application Settings -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Application Settings</h2>
    
    <div class="space-y-4">
      <div class="flex items-center justify-between py-3 border-b border-gray-200">
        <div>
          <div class="font-medium text-gray-800">Auto-refresh Data</div>
          <div class="text-sm text-gray-600">Automatically refresh weather and sensor data every 30 seconds</div>
        </div>
        <label class="relative inline-flex items-center cursor-pointer">
          <input type="checkbox" checked class="sr-only peer" disabled>
          <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
        </label>
      </div>

      <div class="flex items-center justify-between py-3 border-b border-gray-200">
        <div>
          <div class="font-medium text-gray-800">Desktop Notifications</div>
          <div class="text-sm text-gray-600">Show system notifications for alerts and connection status</div>
        </div>
        <label class="relative inline-flex items-center cursor-pointer">
          <input type="checkbox" class="sr-only peer" disabled>
          <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
        </label>
      </div>

      <div class="flex items-center justify-between py-3">
        <div>
          <div class="font-medium text-gray-800">Dark Mode</div>
          <div class="text-sm text-gray-600">Use dark theme for the application interface</div>
        </div>
        <label class="relative inline-flex items-center cursor-pointer">
          <input type="checkbox" class="sr-only peer" disabled>
          <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
        </label>
      </div>
    </div>

    <div class="mt-6 pt-4 border-t border-gray-200">
      <p class="text-xs text-gray-500 mb-3">
        Note: Some application settings are not yet implemented and will be available in future updates.
      </p>
    </div>
  </div>

  <!-- Actions -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Actions</h2>
    
    <div class="flex flex-wrap gap-4">
      <button
        on:click={saveApiSettings}
        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        💾 Save Weather API Settings
      </button>

      <button
        on:click={saveApplicationSettings}
        class="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
      >
        💾 Save App Settings
      </button>

      <button
        on:click={resetToDefaults}
        class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
      >
        🔄 Reset to Defaults
      </button>
    </div>

    {#if settingsSaved}
      <div class="mt-4 p-3 bg-green-50 border border-green-200 rounded-md">
        <div class="flex">
          <div class="flex-shrink-0">
            <span class="text-green-400">✅</span>
          </div>
          <div class="ml-3">
            <p class="text-sm text-green-800">Settings saved successfully!</p>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- About -->
  <div class="bg-gray-50 border border-gray-200 rounded-lg p-6">
    <h3 class="text-lg font-medium text-gray-800 mb-3">About Weather Station Desktop</h3>
    <div class="text-sm text-gray-600 space-y-2">
      <p><strong>Version:</strong> 0.1.0</p>
      <p><strong>Built with:</strong> Tauri, Rust, Svelte, TypeScript</p>
      <p><strong>Features:</strong></p>
      <ul class="list-disc list-inside ml-4 space-y-1">
        <li>Real-time MQTT communication with IoT devices</li>
        <li>Weather API integration (OpenWeatherMap)</li>
        <li>Alert system for weather notifications</li>
        <li>Cross-platform desktop application</li>
      </ul>
    </div>
  </div>
</div>