from fastapi import FastAPI
import get_data_redis
from fastapi.middleware.cors import CORSMiddleware
import subprocess
from datetime import datetime, timezone
import os
from pydantic import BaseModel
import random
from typing import Any
from fastapi import Body

def get_hash():
    # Equivalent of: TIMESTAMP=$(date -Iseconds -u)
    timestamp = datetime.now(timezone.utc).isoformat(timespec="seconds")


    unix_ts = int(datetime.now(timezone.utc).timestamp())
    rand_val = random.randint(0, 32767)  # similar range to $RANDOM
    doc_hash = f"auto_{unix_ts}_{rand_val}"


    return doc_hash

app = FastAPI()

class Item(BaseModel):
    item_id: str
    event_id: str
    location: str
    owner: str


app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/items/")
def read_item(itemid: str):
    data = get_data_redis.all_events_of_itemid(itemid)
    return {'item_id': itemid, "data": data}

@app.post('/add-item/')
def read_item(data: dict[str, Any] = Body(...)):
    print(data)

    subprocess.run(["cargo", "run", "--bin", 
    "polari", "--", "add", f"{data['item_id']}", f"{data['event_type']}", 
    f"{data['location_id']}", f"{data['owner']}", f"{get_hash()}"]) 
