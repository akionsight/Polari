from fastapi import FastAPI
import get_data_redis
from fastapi.middleware.cors import CORSMiddleware


app = FastAPI()



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

# @app.post('/add-item/')
# def read_item():