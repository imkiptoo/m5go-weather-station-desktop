<script lang="ts">
  import type { WeatherData } from './tauri';

  export let weatherData: WeatherData | null;
  export let title: string = 'Current Weather';

  function formatDate(timestamp: string): string {
    return new Date(timestamp).toLocaleString();
  }

  function getWeatherIcon(icon: string): string {
    // Map weather icons to emoji or return default
    const iconMap: { [key: string]: string } = {
      '01d': '☀️', '01n': '🌙',
      '02d': '⛅', '02n': '⛅',
      '03d': '☁️', '03n': '☁️',
      '04d': '☁️', '04n': '☁️',
      '09d': '🌧️', '09n': '🌧️',
      '10d': '🌦️', '10n': '🌦️',
      '11d': '⛈️', '11n': '⛈️',
      '13d': '❄️', '13n': '❄️',
      '50d': '🌫️', '50n': '🌫️'
    };
    return iconMap[icon] || '🌤️';
  }
</script>

<div class="bg-white rounded-lg shadow-md p-6">
  <h2 class="text-xl font-semibold text-gray-800 mb-4">{title}</h2>
  
  {#if weatherData}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- Main weather info -->
      <div class="space-y-2">
        <div class="flex items-center space-x-2">
          <span class="text-3xl">{getWeatherIcon(weatherData.current_icon)}</span>
          <div>
            <div class="text-2xl font-bold text-gray-900">
              {weatherData.current_temp.toFixed(1)}°C
            </div>
            <div class="text-sm text-gray-600 capitalize">
              {weatherData.condition}
            </div>
          </div>
        </div>
        
        <div class="text-sm text-gray-600">
          <div>📍 {weatherData.location}</div>
          <div>🕒 {formatDate(weatherData.timestamp)}</div>
        </div>
      </div>

      <!-- Weather details -->
      <div class="grid grid-cols-2 gap-4 text-sm">
        <div class="bg-blue-50 p-3 rounded">
          <div class="text-blue-600 font-medium">Humidity</div>
          <div class="text-lg font-semibold">{weatherData.humidity}%</div>
        </div>
        
        <div class="bg-green-50 p-3 rounded">
          <div class="text-green-600 font-medium">Pressure</div>
          <div class="text-lg font-semibold">{weatherData.pressure} hPa</div>
        </div>
        
        <div class="bg-yellow-50 p-3 rounded">
          <div class="text-yellow-600 font-medium">Wind Speed</div>
          <div class="text-lg font-semibold">{weatherData.wind_speed.toFixed(1)} m/s</div>
        </div>
        
        <div class="bg-purple-50 p-3 rounded">
          <div class="text-purple-600 font-medium">Wind Dir</div>
          <div class="text-lg font-semibold">{weatherData.wind_direction}</div>
        </div>
      </div>
    </div>

    <!-- Forecast -->
    {#if weatherData.forecast && weatherData.forecast.length > 0}
      <div class="mt-6">
        <h3 class="text-lg font-medium text-gray-800 mb-3">5-Day Forecast</h3>
        <div class="grid grid-cols-5 gap-2">
          {#each weatherData.forecast.slice(0, 5) as day}
            <div class="text-center bg-gray-50 p-2 rounded">
              <div class="text-xs font-medium text-gray-600">{day.day}</div>
              <div class="text-xs text-gray-500">{day.date}</div>
              <div class="text-lg my-1">{getWeatherIcon(day.icon)}</div>
              <div class="text-sm font-semibold">{day.temp.toFixed(0)}°</div>
              <div class="text-xs text-gray-500">{day.humidity}%</div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {:else}
    <div class="text-center py-8 text-gray-500">
      <div class="text-4xl mb-2">🌤️</div>
      <div>No weather data available</div>
      <div class="text-sm">Connect to MQTT or fetch from API</div>
    </div>
  {/if}
</div>