#!/usr/bin/env python3
"""
Simple MQTT sensor data publisher using umqtt (M5Stack compatible)
This simulates an M5Stack sending sensor data to test the desktop app.
"""

import json
import time
import random
from datetime import datetime, timezone

# Try to import umqtt (M5Stack style)
try:
    from umqtt import MQTTClient
    print("✅ Using umqtt (M5Stack compatible)")
except ImportError:
    # Fallback to paho-mqtt if umqtt not available
    try:
        import paho.mqtt.client as mqtt
        print("✅ Using paho-mqtt (fallback)")
        umqtt_available = False
    except ImportError:
        print("❌ No MQTT library available. Install with: pip install paho-mqtt")
        exit(1)
    umqtt_available = False
else:
    umqtt_available = True

# MQTT Configuration  
MQTT_BROKER = "192.168.137.1"
MQTT_PORT = 1883
CLIENT_ID = "test_sensor_publisher"
SENSOR_DATA_TOPIC = "weather/sensor_data"

def generate_sensor_data():
    """Generate realistic sensor data matching M5Stack ENV III sensor"""
    # Simulate realistic sensor readings
    base_temp = 22.0
    base_humidity = 60
    base_pressure = 1013
    
    # Add some random variation to simulate real sensor readings
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

def publish_with_umqtt():
    """Publish sensor data using umqtt (M5Stack style)"""
    print(f"🌡️  MQTT Sensor Data Publisher (umqtt)")
    print("=" * 40)
    print(f"Broker: {MQTT_BROKER}:{MQTT_PORT}")
    print(f"Topic: {SENSOR_DATA_TOPIC}")
    print(f"Client ID: {CLIENT_ID}")
    print("=" * 40)
    
    try:
        # Create MQTT client (umqtt style)
        client = MQTTClient(CLIENT_ID, MQTT_BROKER, port=MQTT_PORT)
        
        # Connect to MQTT broker
        print("🔌 Connecting to MQTT broker...")
        client.connect()
        print("✅ Connected to MQTT broker!")
        
        print("🚀 Starting to publish sensor data...")
        print("Press Ctrl+C to stop")
        
        publish_count = 0
        
        while True:
            # Generate and publish sensor data
            sensor_data = generate_sensor_data()
            
            # Convert to JSON
            json_payload = json.dumps(sensor_data)
            
            # Publish to MQTT (umqtt uses bytes)
            client.publish(SENSOR_DATA_TOPIC, json_payload.encode())
            
            publish_count += 1
            print(f"📊 Published #{publish_count}: Temp={sensor_data['sensor_temp']}°C, "
                  f"Humidity={sensor_data['sensor_humidity']}%, "
                  f"Pressure={sensor_data['sensor_pressure']}hPa")
            
            # Wait 10 seconds before next reading (same as M5Stack)
            time.sleep(10)
            
    except KeyboardInterrupt:
        print("\n🛑 Stopping sensor data publisher...")
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        try:
            client.disconnect()
            print("👋 Disconnected from MQTT broker")
        except:
            pass

def publish_with_paho():
    """Publish sensor data using paho-mqtt (fallback)"""
    print(f"🌡️  MQTT Sensor Data Publisher (paho-mqtt)")
    print("=" * 40)
    print(f"Broker: {MQTT_BROKER}:{MQTT_PORT}")
    print(f"Topic: {SENSOR_DATA_TOPIC}")
    print("=" * 40)
    
    def on_connect(client, userdata, flags, rc):
        if rc == 0:
            print("✅ Connected to MQTT broker!")
        else:
            print(f"❌ Failed to connect. Return code: {rc}")

    def on_publish(client, userdata, mid):
        print(f"📤 Message published (id: {mid})")

    try:
        # Create MQTT client
        client = mqtt.Client()
        client.on_connect = on_connect
        client.on_publish = on_publish
        
        # Connect to MQTT broker
        print("🔌 Connecting to MQTT broker...")
        client.connect(MQTT_BROKER, MQTT_PORT, 60)
        client.loop_start()
        
        # Wait for connection
        time.sleep(2)
        
        print("🚀 Starting to publish sensor data...")
        print("Press Ctrl+C to stop")
        
        publish_count = 0
        
        while True:
            # Generate and publish sensor data
            sensor_data = generate_sensor_data()
            
            # Convert to JSON
            json_payload = json.dumps(sensor_data)
            
            # Publish to MQTT
            result = client.publish(SENSOR_DATA_TOPIC, json_payload)
            
            if result.rc == mqtt.MQTT_ERR_SUCCESS:
                publish_count += 1
                print(f"📊 Published #{publish_count}: Temp={sensor_data['sensor_temp']}°C, "
                      f"Humidity={sensor_data['sensor_humidity']}%, "
                      f"Pressure={sensor_data['sensor_pressure']}hPa")
            else:
                print(f"❌ Failed to publish. Error code: {result.rc}")
            
            # Wait 10 seconds before next reading
            time.sleep(10)
            
    except KeyboardInterrupt:
        print("\n🛑 Stopping sensor data publisher...")
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        try:
            client.loop_stop()
            client.disconnect()
            print("👋 Disconnected from MQTT broker")
        except:
            pass

def main():
    """Main function - choose appropriate MQTT library"""
    if umqtt_available:
        publish_with_umqtt()
    else:
        publish_with_paho()

if __name__ == "__main__":
    main()