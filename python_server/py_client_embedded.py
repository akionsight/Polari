import serial
import time
import get_data_redis

ser = serial.Serial('/dev/ttyACM0', 9600, timeout=1)

try:
    while True:
        if ser.in_waiting > 0:           # Data available?
            line = ser.readline()
            term_data = str(get_data_redis.all_events_of_itemid(str(line.decode('utf-8').strip()))).encode('utf-8')
            print(line.decode('utf-8').strip())
            # print(term_data)
            ser.write(term_data)
except KeyboardInterrupt:
    ser.close()
    print("Connection closed")