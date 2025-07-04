<script lang="ts">
  import { sendAlert, type AlertLevel } from '$lib/tauri';
  import { mqttConnected, showMessage } from '$lib/stores';

  let alertMessage = '';
  let alertLevel: AlertLevel = 'info';
  let isSending = false;
  let predefinedAlerts = [
    { message: 'Weather station test alert', level: 'info' as AlertLevel },
    { message: 'High temperature warning', level: 'warning' as AlertLevel },
    { message: 'Low humidity alert', level: 'warning' as AlertLevel },
    { message: 'Strong wind detected', level: 'warning' as AlertLevel },
    { message: 'Pressure change alert', level: 'warning' as AlertLevel },
    { message: 'System maintenance required', level: 'info' as AlertLevel },
    { message: 'EMERGENCY: Extreme weather conditions detected!', level: 'emergency' as AlertLevel },
    { message: 'EMERGENCY: System failure detected!', level: 'emergency' as AlertLevel }
  ];

  const alertLevelColors = {
    info: 'bg-blue-50 border-blue-200 text-blue-800',
    warning: 'bg-yellow-50 border-yellow-200 text-yellow-800', 
    emergency: 'bg-red-50 border-red-200 text-red-800'
  };

  const alertLevelButtonColors = {
    info: 'bg-blue-600 hover:bg-blue-700 focus:ring-blue-500',
    warning: 'bg-yellow-600 hover:bg-yellow-700 focus:ring-yellow-500',
    emergency: 'bg-red-600 hover:bg-red-700 focus:ring-red-500'
  };

  const alertLevelIcons = {
    info: 'ℹ️',
    warning: '⚠️', 
    emergency: '🚨'
  };

  async function sendCustomAlert() {
    if (!alertMessage.trim()) {
      showMessage('Please enter an alert message', true);
      return;
    }

    if (!$mqttConnected) {
      showMessage('MQTT not connected', true);
      return;
    }

    isSending = true;
    try {
      await sendAlert(alertMessage, alertLevel);
      showMessage(`${alertLevel} alert sent: "${alertMessage}"`, false);
      alertMessage = '';
      alertLevel = 'info'; // Reset to default
    } catch (error) {
      showMessage(`Failed to send alert: ${error}`, true);
    } finally {
      isSending = false;
    }
  }

  async function sendPredefinedAlert(message: string, level: AlertLevel) {
    if (!$mqttConnected) {
      showMessage('MQTT not connected', true);
      return;
    }

    try {
      await sendAlert(message, level);
      showMessage(`${level} alert sent: "${message}"`, false);
    } catch (error) {
      showMessage(`Failed to send alert: ${error}`, true);
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h1 class="text-2xl font-bold text-gray-900 mb-2">Weather Alerts</h1>
    <p class="text-gray-600">Send alerts and notifications to your IoT weather station</p>
  </div>

  <!-- Alert Status -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Alert System Status</h2>
    
    <div class="flex items-center space-x-4 mb-4">
      <div class="flex items-center space-x-2">
        <div class="w-3 h-3 rounded-full {$mqttConnected ? 'bg-green-500' : 'bg-red-500'}"></div>
        <span class="font-medium">MQTT Connection</span>
      </div>
      <span class="text-sm text-gray-600">
        {$mqttConnected ? 'Ready to send alerts' : 'Connect to MQTT to send alerts'}
      </span>
    </div>

    {#if !$mqttConnected}
      <div class="bg-yellow-50 border border-yellow-200 rounded-md p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <span class="text-yellow-400">⚠️</span>
          </div>
          <div class="ml-3">
            <p class="text-sm text-yellow-800">
              MQTT connection required to send alerts to your weather station. 
              Please connect to MQTT first in the Dashboard or MQTT Control section.
            </p>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Custom Alert -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Send Custom Alert</h2>
    
    <div class="space-y-4">
      <div>
        <label for="alert-message" class="block text-sm font-medium text-gray-700 mb-1">
          Alert Message
        </label>
        <textarea
          id="alert-message"
          bind:value={alertMessage}
          rows="3"
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter your custom alert message..."
          disabled={!$mqttConnected}
        ></textarea>
      </div>

      <div>
        <label for="alert-level" class="block text-sm font-medium text-gray-700 mb-1">
          Alert Level
        </label>
        <select
          id="alert-level"
          bind:value={alertLevel}
          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          disabled={!$mqttConnected}
        >
          <option value="info">ℹ️ Info - General information</option>
          <option value="warning">⚠️ Warning - Attention required</option>
          <option value="emergency">🚨 Emergency - Immediate action needed</option>
        </select>
      </div>

      <button
        on:click={sendCustomAlert}
        disabled={!$mqttConnected || !alertMessage.trim() || isSending}
        class="px-6 py-2 text-white rounded-md focus:outline-none focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed {alertLevelButtonColors[alertLevel]}"
      >
        {isSending ? 'Sending Alert...' : `${alertLevelIcons[alertLevel]} Send ${alertLevel} Alert`}
      </button>
    </div>
  </div>

  <!-- Predefined Alerts -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Quick Alerts</h2>
    <p class="text-gray-600 mb-4">Click any button below to send a predefined alert message</p>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each predefinedAlerts as alert}
        <button
          on:click={() => sendPredefinedAlert(alert.message, alert.level)}
          disabled={!$mqttConnected}
          class="p-4 text-left border-2 rounded-md hover:shadow-md focus:outline-none focus:ring-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 {alertLevelColors[alert.level]}"
        >
          <div class="flex items-center justify-between mb-2">
            <span class="text-lg">{alertLevelIcons[alert.level]}</span>
            <span class="text-xs font-medium px-2 py-1 rounded {alert.level === 'emergency' ? 'bg-red-200 text-red-800' : alert.level === 'warning' ? 'bg-yellow-200 text-yellow-800' : 'bg-blue-200 text-blue-800'}">{alert.level}</span>
          </div>
          <div class="font-medium">{alert.message}</div>
        </button>
      {/each}
    </div>
  </div>

  <!-- Alert Information -->
  <div class="bg-blue-50 border border-blue-200 rounded-lg p-6">
    <h3 class="text-lg font-medium text-blue-800 mb-2">How Alerts Work</h3>
    <div class="text-sm text-blue-700 space-y-2">
      <p>• Alerts are sent via MQTT to the <code class="bg-blue-100 px-1 rounded">weather/alert_trigger</code> topic</p>
      <p>• Your IoT weather station will display the alert message on its screen</p>
      <p>• Alerts include a timestamp and are processed immediately by the device</p>
      <p>• Use alerts for weather warnings, system notifications, or testing communication</p>
    </div>
  </div>

  <!-- Recent Alerts Log (placeholder for future implementation) -->
  <div class="bg-white rounded-lg shadow-md p-6">
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Alert History</h2>
    <div class="text-center py-8 text-gray-500">
      <div class="text-3xl mb-2">📋</div>
      <div>Alert history coming soon</div>
      <div class="text-sm">This feature will show recently sent alerts</div>
    </div>
  </div>
</div>