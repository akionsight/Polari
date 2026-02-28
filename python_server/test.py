# import struct

# data = (
#     b"""\x01\x00\x00\x00\x00\x00\x00\x00#\x00\x00\x00\x00\x00\x00\x00
#     2026-02-28T09:47:40.389485426+00:00
#     (\x03\x00\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00\x00\x00
#     0004613f490a1cc9553fafac1bbd3e166f8f59789178773fde794d4a7b59f68d
#     \x07\x00\x00\x00\x00\x00\x00\x00ECID123
#     \x0b\x00\x00\x00\x00\x00\x00\x00manufacture
#     \x18\x00\x00\x00\x00\x00\x00\x00Factory, Gharroli, Delhi
#     #\x00\x00\x00\x00\x00\x00\x002026-02-28T09:47:40.389476909+00:00
#     \x06\x00\x00\x00\x00\x00\x00\x00Miner1
#     \x12\x00\x00\x00\x00\x00\x00\x00sha256_cert_abc123
#     @\x00\x00\x00\x00\x00\x00\x00
#     00c296e000086ee1a6bc417106cd4c82f12694348327b27aad7a605911803592"""
# )

# def parse_length_prefixed(data: bytes) -> list[str]:
#     """
#     Parses a binary blob where each field is:
#       [8-byte little-endian uint64 length][N bytes UTF-8 string]
#     Non-decodable/header bytes are skipped one byte at a time.
#     """
#     results = []
#     pos = 0
#     while pos < len(data):
#         if pos + 8 > len(data):
#             break
#         length = struct.unpack_from("<Q", data, pos)[0]
#         pos += 8
#         if 0 < length <= len(data) - pos:
#             chunk = data[pos : pos + length]
#             try:
#                 results.append(chunk.decode("utf-8"))
#                 pos += length
#                 continue
#             except UnicodeDecodeError:
#                 pass
#         pos -= 7  # Rewind to advance only 1 byte and retry

#     return [s for s in results if len(s) > 1]  # Drop single-char header artifacts


# fields = parse_length_prefixed(data)

# # Fields in order: prev_hash, item_id, event_type, location, timestamp, owner, cert, document_hash
# result = {
#     "item_id":       fields[1],
#     "event_type":    fields[2],
#     "location":      fields[3],
#     "timestamp":     fields[4],
#     "owner":         fields[5],
#     "document_hash": fields[7],
# }

# print(result)

# # def hex_to_ascii(bytt):
# #     return chr(int(str(byt), 16))


# # def parse_log_to_dict(raw_data):
# #     data = raw_data.strip()

# #     pattern = re.compile(
# #         r'#(?P<timestamp1>[^\(]+)'           # #timestamp1
# #         r'\(@(?P<hash1>[0-9a-f]+)'           # (@hash1
# #         r'(?P<ecid>ECID\w+)\s+'             # ECID123
# #         r'(?P<location>[^#]+?)\s*'           # location (trimmed)
# #         r'#(?P<timestamp2>[^\s]+)'           # #timestamp2
# #         r'(?P<miner_id>\w+?)_cert_'          # miner_id
# #         r'(?P<cert>[^@]+)'                   # cert
# #         r'@(?P<hash2>[0-9a-f]+)',            # @hash2
# #         re.DOTALL
# #     )

# #     match = pattern.search(data)
# #     if not match:
# #         raise ValueError("Log format not recognized")

# #     result = match.groupdict()
# #     # Strip whitespace from all values
# #     return {k: v.strip() for k, v in result.items()}

# # def parse_log_to_dict(raw_data):
# #     data = raw_data.strip()

# #     pattern = re.compile(
# #         r'#(?P<timestamp1>[^\(]+)'
# #         r'\(@(?P<hash1>[0-9a-f]+)'
# #         r'(?P<ecid>ECID\w+)\s+'
# #         r'(?P<location>[^#]+?)\s*'
# #         r'#(?P<timestamp2>[^\s]+)'
# #         r'(?P<miner_id>\w+?)_cert_'
# #         r'(?P<cert>[^@]+)'
# #         r'@(?P<hash2>[0-9a-f]+)',
# #         re.DOTALL
# #     )

# #     match = pattern.search(data)
# #     if not match:
# #         raise ValueError("Log format not recognized")

# #     return {k: v.strip() for k, v in match.groupdict().items()}



# # data_fin = []

# # last_char_space = False
# # for byt in bytes_to_c_arr(get_data(1)):
    
# #     char = hex_to_ascii(byt)
    
# #     if char == '\x00':
# #         if last_char_space:
# #             continue
# #         else:
# #             data_fin.append(char)
# #             last_char_space = True
    
# #     else:
# #         data_fin.append(char)
# #         last_char_space = False

# # # data_str = ""

# # # for char in data_fin:
# # #     if char != ' ':
# # #         data_str += char


import redis
import json
from dataclasses import dataclass
from typing import List


@dataclass
class SupplyChainData:
    item_id: str
    event_type: str
    location: str
    timestamp: str
    owner: str
    document_hash: str


@dataclass
class Block:
    index: int
    timestamp: str
    previous_hash: str
    hash: str
    data: SupplyChainData


def connect_redis(host: str = "127.0.0.1", port: int = 6379, db: int = 0) -> redis.Redis:
    return redis.Redis(host=host, port=port, db=db)


def decode_block(raw: bytes) -> Block:
    """Decode a JSON-encoded block from Redis into a Block object."""
    obj = json.loads(raw.decode("utf-8"))
    data = obj["data"]
    sc_data = SupplyChainData(
        item_id=data["item_id"],
        event_type=data["event_type"],
        location=data["location"],
        timestamp=data["timestamp"],
        owner=data["owner"],
        document_hash=data["document_hash"],
    )
    return Block(
        index=obj["index"],
        timestamp=obj["timestamp"],
        previous_hash=obj["previous_hash"],
        hash=obj["hash"],
        data=sc_data,
    )


def get_item_events_from_redis(
    r: redis.Redis,
    item_id: str,
    pattern: str = "block:*",
    count: int = 50,
) -> List[Block]:
    """
    Scan Redis for all block:* keys, decode blocks, and
    return those whose data.item_id == item_id, sorted by index.
    """
    cursor = 0
    result: List[Block] = []

    while True:
        cursor, keys = r.scan(cursor=cursor, match=pattern, count=count)
        for key in keys:
            raw = r.get(key)
            if not raw:
                continue

            try:
                block = decode_block(raw)
            except Exception:
                # Skip malformed entries
                continue

            if block.data.item_id == item_id:
                result.append(block)

        if cursor == 0:
            break

    # Sort by block index
    result.sort(key=lambda b: b.index)
    return result


if __name__ == "__main__":
    r = connect_redis()

    item_id = "ECID123"
    events = get_item_events_from_redis(r, item_id)

    print(f"Found {len(events)} events for item_id={item_id}")
    for b in events:
        print(
            f"Block {b.index}: {b.data.event_type} at {b.data.location} "
            f"by {b.data.owner} at {b.data.timestamp}"
        )


# def parse_length_prefixed(data: bytes) -> list[str]:
#     """
#     Parses a binary blob where each field is:
#       [8-byte little-endian uint64 length][N bytes UTF-8 string]
#     Non-decodable/header bytes are skipped one byte at a time.
#     """
#     results = []
#     pos = 0
#     while pos < len(data):
#         if pos + 8 > len(data):
#             break
#         length = struct.unpack_from("<Q", data, pos)[0]
#         pos += 8
#         if 0 < length <= len(data) - pos:
#             chunk = data[pos : pos + length]
#             try:
#                 results.append(chunk.decode("utf-8"))
#                 pos += length
#                 continue
#             except UnicodeDecodeError:
#                 pass
#         pos -= 7  # Rewind to advance only 1 byte and retry

#     fields = [s for s in results if len(s) > 1]  # Drop single-char header artifacts
#     result = {
#     "item_id":       fields[2],
#     "event_type":    fields[3],
#     "location":      fields[4],
#     "timestamp":     fields[5],
#     "owner":         fields[5],
#     "document_hash": fields[7],
#     }
#     print(fields)
#     return result