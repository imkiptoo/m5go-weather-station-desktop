#!/usr/bin/env python3
"""
Simple MQTT sensor data publisher to test the weather station desktop app.
This simulates an M5Stack or similar IoT device sending sensor data.
"""

import json
import time
import random
from datetime import datetime, timezone
import paho.mqtt.client as mqtt

# MQTT Configuration
MQTT_BROKER = "192.168.137.1"
MQTT_PORT = 1883
SENSOR_DATA_TOPIC = "weather/sensor_data"

def on_connect(client, userdata, flags, rc):
    if rc == 0:
        print(f"✅ Connected to MQTT broker {MQTT_BROKER}:{MQTT_PORT}")
    else:
        print(f"❌ Failed to connect to MQTT broker. Return code: {rc}")

def on_publish(client, userdata, mid):
    print(f"📤 Sensor data published (message id: {mid})")

def generate_sensor_data():
    """Generate realistic sensor data"""
    # Simulate realistic sensor readings
    base_temp = 22.0
    base_humidity = 60
    base_pressure = 1013
    
    # Add some random variation
    temp_variation = random.uniform(-3.0, 8.0)
    humidity_variation = random.randint(-15, 20)
    pressure_variation = random.randint(-20, 20)
    
    sensor_data = {
        "sensor_temp": round(base_temp + temp_variation, 1),
        "sensor_humidity": max(0, min(100, base_humidity + humidity_variation)),
        "sensor_pressure": base_pressure + pressure_variation,
        "timestamp": datetime.now(timezone.utc).isoformat()
    }
    
    return sensor_data

def main():
    print("🌡️  MQTT Sensor Data Publisher")
    print("=" * 40)
    print(f"Broker: {MQTT_BROKER}:{MQTT_PORT}")
    print(f"Topic: {SENSOR_DATA_TOPIC}")
    print("=" * 40)
    
    # Create MQTT client
    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_publish = on_publish
    
    try:
        # Connect to MQTT broker
        client.connect(MQTT_BROKER, MQTT_PORT, 60)
        client.loop_start()
        
        # Wait for connection
        time.sleep(2)
        
        print("🚀 Starting to publish sensor data...")
        print("Press Ctrl+C to stop")
        
        while True:
            # Generate and publish sensor data
            sensor_data = generate_sensor_data()
            
            # Convert to JSON
            json_payload = json.dumps(sensor_data, indent=2)
            
            # Publish to MQTT
            result = client.publish(SENSOR_DATA_TOPIC, json_payload)
            
            if result.rc == mqtt.MQTT_ERR_SUCCESS:
                print(f"📊 Published: Temp={sensor_data['sensor_temp']}°C, "
                      f"Humidity={sensor_data['sensor_humidity']}%, "
                      f"Pressure={sensor_data['sensor_pressure']}hPa")
            else:
                print(f"❌ Failed to publish sensor data. Error code: {result.rc}")
            
            # Wait 10 seconds before next reading
            time.sleep(10)
            
    except KeyboardInterrupt:
        print("\n🛑 Stopping sensor data publisher...")
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        client.loop_stop()
        client.disconnect()
        print("👋 Disconnected from MQTT broker")

if __name__ == "__main__":
    main()