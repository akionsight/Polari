import redis
import re
import struct
import json


redis_instance = redis.Redis(host='localhost', port=6379, db=0)


def get_data(id):
    return redis_instance.get(f'block:{id}')




# print(get_data(5))


def all_events_of_itemid(itemid):
    cursor = 0
    fin_dat = []
    while True:
        data = get_data(cursor)
        if data == None:
            # print('here')
            break
        else:
            dat = json.loads(data)
            if dat['data']['item_id'] == itemid:
                fin_dat.append(dat['data'])
                cursor += 1
            else:
                cursor += 1
                continue

    return fin_dat
        
# print(all_events_of_itemid('ECID123')) 



# fields = parse_length_prefixed(data)

# Fields in order: prev_hash, item_id, event_type, location, timestamp, owner, cert, document_hash

## LIST OF ALL THE EVENTS FOR ONE SUPPLY CHAIN THINGY

