#!/usr/bin/env python3
"""
Simple MQTT test using basic socket connection to test sensor data publishing.
No external libraries required.
"""

import socket
import json
import time
import random
from datetime import datetime, timezone

def connect_mqtt(host, port, client_id):
    """Basic MQTT connection using raw socket"""
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect((host, port))
        
        # Simple MQTT CONNECT packet
        client_id_bytes = client_id.encode('utf-8')
        remaining_length = 10 + len(client_id_bytes)
        
        connect_packet = bytearray([
            0x10,  # CONNECT packet type
            remaining_length,  # Remaining length
            0x00, 0x04, 0x4D, 0x51, 0x54, 0x54,  # Protocol name "MQTT"
            0x04,  # Protocol level
            0x00,  # Connect flags (clean session)
            0x00, 0x3C,  # Keep alive (60 seconds)
            0x00, len(client_id_bytes)  # Client ID length
        ])
        connect_packet.extend(client_id_bytes)
        
        sock.send(connect_packet)
        
        # Read CONNACK
        response = sock.recv(4)
        if len(response) >= 4 and response[0] == 0x20 and response[3] == 0x00:
            print("✅ Connected to MQTT broker")
            return sock
        else:
            print("❌ MQTT connection failed")
            return None
            
    except Exception as e:
        print(f"❌ Connection error: {e}")
        return None

def publish_mqtt(sock, topic, message):
    """Publish message to MQTT topic"""
    try:
        topic_bytes = topic.encode('utf-8')
        message_bytes = message.encode('utf-8')
        
        # Calculate remaining length
        remaining_length = 2 + len(topic_bytes) + len(message_bytes)
        
        # Build PUBLISH packet
        publish_packet = bytearray([
            0x30,  # PUBLISH packet type
            remaining_length,  # Remaining length
            0x00, len(topic_bytes)  # Topic length
        ])
        publish_packet.extend(topic_bytes)
        publish_packet.extend(message_bytes)
        
        sock.send(publish_packet)
        return True
        
    except Exception as e:
        print(f"❌ Publish error: {e}")
        return False

def generate_sensor_data():
    """Generate realistic sensor data"""
    base_temp = 22.0
    base_humidity = 60
    base_pressure = 1013
    
    temp_variation = random.uniform(-3.0, 8.0)
    humidity_variation = random.randint(-15, 20)
    pressure_variation = random.randint(-20, 20)
    
    sensor_data = {
        "sensor_temp": round(base_temp + temp_variation, 1),
        "sensor_humidity": max(0, min(100, base_humidity + humidity_variation)),
        "sensor_pressure": base_pressure + pressure_variation,
        "timestamp": int(time.time())  # Unix timestamp as integer like M5Stack
    }
    
    return sensor_data

def main():
    """Main test function"""
    print("🌡️  Simple MQTT Sensor Data Test")
    print("=" * 40)
    print("Broker: 192.168.137.1:1883")
    print("Topic: weather/sensor_data")
    print("=" * 40)
    
    # Connect to MQTT
    sock = connect_mqtt("192.168.137.1", 1883, "test_sensor_simple")
    if not sock:
        print("❌ Cannot connect to MQTT broker")
        return
    
    try:
        print("🚀 Publishing sensor data...")
        print("Press Ctrl+C to stop")
        
        count = 0
        while True:
            # Generate sensor data
            sensor_data = generate_sensor_data()
            json_message = json.dumps(sensor_data)
            
            # Publish to MQTT
            if publish_mqtt(sock, "weather/sensor_data", json_message):
                count += 1
                print(f"📊 Published #{count}: Temp={sensor_data['sensor_temp']}°C, "
                      f"Humidity={sensor_data['sensor_humidity']}%, "
                      f"Pressure={sensor_data['sensor_pressure']}hPa")
            else:
                print("❌ Failed to publish")
                break
            
            # Wait 10 seconds
            time.sleep(10)
            
    except KeyboardInterrupt:
        print("\n🛑 Stopping...")
    except Exception as e:
        print(f"❌ Error: {e}")
    finally:
        sock.close()
        print("👋 Disconnected")

if __name__ == "__main__":
    main()