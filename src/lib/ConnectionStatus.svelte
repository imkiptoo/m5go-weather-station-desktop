<script lang="ts">
  import { mqttConnected, apiConnected } from './stores';
  import { getMqttStatus, connectMqtt, disconnectMqtt } from './tauri';
  import { showMessage } from './stores';

  let brokerHost = '192.168.137.1';
  let brokerPort = 1883;
  let isConnecting = false;

  // Check MQTT status periodically
  async function checkMqttStatus() {
    try {
      const connected = await getMqttStatus();
      mqttConnected.set(connected);
    } catch (error) {
      console.error('Failed to check MQTT status:', error);
    }
  }

  // Connect to MQTT
  async function handleConnect() {
    if (isConnecting) return;
    
    isConnecting = true;
    try {
      const result = await connectMqtt(brokerHost, brokerPort);
      showMessage(`MQTT: ${result}`, false);
      await checkMqttStatus();
    } catch (error) {
      showMessage(`MQTT connection failed: ${error}`, true);
    } finally {
      isConnecting = false;
    }
  }

  // Disconnect from MQTT
  async function handleDisconnect() {
    try {
      const result = await disconnectMqtt();
      showMessage(`MQTT: ${result}`, false);
      await checkMqttStatus();
    } catch (error) {
      showMessage(`MQTT disconnect failed: ${error}`, true);
    }
  }

  // Check status on component mount and periodically
  checkMqttStatus();
  setInterval(checkMqttStatus, 5000);
</script>

<div class="bg-white rounded-lg shadow-md p-6">
  <h2 class="text-xl font-semibold text-gray-800 mb-4">Connection Status</h2>
  
  <!-- Status indicators -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
    <div class="flex items-center space-x-3">
      <div class="flex items-center space-x-2">
        <div class="w-3 h-3 rounded-full {$mqttConnected ? 'bg-green-500' : 'bg-red-500'}"></div>
        <span class="font-medium">MQTT</span>
      </div>
      <span class="text-sm text-gray-600">
        {$mqttConnected ? 'Connected' : 'Disconnected'}
      </span>
    </div>
    
    <div class="flex items-center space-x-3">
      <div class="flex items-center space-x-2">
        <div class="w-3 h-3 rounded-full {$apiConnected ? 'bg-green-500' : 'bg-gray-400'}"></div>
        <span class="font-medium">Weather API</span>
      </div>
      <span class="text-sm text-gray-600">
        {$apiConnected ? 'Active' : 'Ready'}
      </span>
    </div>
  </div>

  <!-- MQTT Connection Controls -->
  <div class="border-t pt-4">
    <h3 class="text-lg font-medium text-gray-800 mb-3">MQTT Configuration</h3>
    
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
      <div>
        <label for="broker-host" class="block text-sm font-medium text-gray-700 mb-1">
          Broker Host
        </label>
        <input
          id="broker-host"
          type="text"
          bind:value={brokerHost}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="192.168.137.1"
          disabled={$mqttConnected}
        />
      </div>
      
      <div>
        <label for="broker-port" class="block text-sm font-medium text-gray-700 mb-1">
          Port
        </label>
        <input
          id="broker-port"
          type="number"
          bind:value={brokerPort}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="1883"
          disabled={$mqttConnected}
        />
      </div>
      
      <div class="flex items-end">
        {#if $mqttConnected}
          <button
            on:click={handleDisconnect}
            class="w-full px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
          >
            Disconnect
          </button>
        {:else}
          <button
            on:click={handleConnect}
            disabled={isConnecting}
            class="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isConnecting ? 'Connecting...' : 'Connect'}
          </button>
        {/if}
      </div>
    </div>
    
    <div class="text-sm text-gray-600">
      <p class="mb-1">
        <strong>Status:</strong> 
        {#if $mqttConnected}
          <span class="text-green-600">Connected to {brokerHost}:{brokerPort}</span>
        {:else}
          <span class="text-gray-500">Not connected</span>
        {/if}
      </p>
      <p class="text-xs">
        Connect to receive real-time weather data from your IoT device and send commands.
      </p>
    </div>
  </div>
</div>