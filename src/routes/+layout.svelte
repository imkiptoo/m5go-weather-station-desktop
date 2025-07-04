<script lang="ts">
  import { activeTab, errorMessage, successMessage } from '$lib/stores';
  import '../app.css';

  const tabs = [
    { id: 'dashboard', name: 'Dashboard', icon: '🏠' },
    { id: 'weather', name: 'Weather API', icon: '🌤️' },
    { id: 'mqtt', name: 'MQTT Control', icon: '📡' },
    { id: 'alerts', name: 'Alerts', icon: '⚠️' },
    { id: 'settings', name: 'Settings', icon: '⚙️' }
  ];

  function setActiveTab(tabId: string) {
    activeTab.set(tabId);
  }
</script>

<svelte:head>
  <title>Weather Station Desktop</title>
</svelte:head>

<div class="min-h-screen bg-gray-100">
  <!-- Header -->
  <header class="bg-white shadow-sm border-b">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <div class="flex items-center space-x-4">
          <div class="text-2xl">🌦️</div>
          <h1 class="text-xl font-semibold text-gray-900">Weather Station Desktop</h1>
        </div>
        
        <nav class="flex space-x-1">
          {#each tabs as tab}
            <button
              on:click={() => setActiveTab(tab.id)}
              class="px-3 py-2 rounded-md text-sm font-medium transition-colors duration-200 {
                $activeTab === tab.id
                  ? 'bg-blue-100 text-blue-700'
                  : 'text-gray-600 hover:text-gray-900 hover:bg-gray-50'
              }"
            >
              <span class="mr-1">{tab.icon}</span>
              {tab.name}
            </button>
          {/each}
        </nav>
      </div>
    </div>
  </header>

  <!-- Messages -->
  {#if $errorMessage}
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-4">
      <div class="bg-red-50 border border-red-200 rounded-md p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <span class="text-red-400">❌</span>
          </div>
          <div class="ml-3">
            <p class="text-sm text-red-800">{$errorMessage}</p>
          </div>
        </div>
      </div>
    </div>
  {/if}

  {#if $successMessage}
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-4">
      <div class="bg-green-50 border border-green-200 rounded-md p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <span class="text-green-400">✅</span>
          </div>
          <div class="ml-3">
            <p class="text-sm text-green-800">{$successMessage}</p>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Main content -->
  <main class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
    <slot />
  </main>
</div>