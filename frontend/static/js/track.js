const container = document.getElementById("recordsContainer");

/* ICON MAP */

const iconMap = {
    item_id: "../static/images/item.svg",
    event_type: "../static/images/event.svg",
    location: "../static/images/location.svg",
    timestamp: "../static/images/time.svg",
    owner: "../static/images/owner.svg",
    document_hash: "../static/images/hash.svg"
};


const params = new URLSearchParams(window.location.search);
const itemId = params.get("id");


/* FETCH DATA */

async function loadData(){

    try{

        const response = await fetch(
            `http://10.75.55.175:8000/items/?itemid=${itemId}`
        );

        if(!response.ok)
            throw new Error("Server error");

        const result = await response.json();

        const records = result.data;

        if(!Array.isArray(records))
            throw new Error("Invalid data format");

        render(records);

    }catch(err){
        console.error(err);
        container.innerHTML =
            "<p style='color:white'>Failed to load data</p>";
    }
}


/* RENDER */

function render(records){

    container.innerHTML = "";

    if(records.length === 0){
        container.innerHTML =
            "<p style='color:white'>No records found</p>";
        return;
    }

    records.sort(
        (a,b)=> new Date(a.timestamp) - new Date(b.timestamp)
    );

    records.forEach(r => {

        const card = document.createElement("div");
        card.className = "record";

        card.innerHTML = `
            ${field("Item ID", r.item_id, "item_id")}
            ${field("Event", r.event_type, "event_type")}
            ${field("Location", r.location, "location")}
            ${field("Timestamp", formatTime(r.timestamp), "timestamp")}
            ${field("Miner", r.owner, "owner")}
            ${field("Hash", shorten(r.document_hash), "document_hash")}
        `;

        container.appendChild(card);
    });
}


/* FIELD BUILDER  */

function field(label, value, key){
    return `
        <div class="field-block">
            <div class="icon">
                <img src="${iconMap[key]}" alt="">
            </div>
            <div class="label">${label}</div>
            <div class="value">${value ?? ""}</div>
        </div>
    `;
}


/* UTILITIES  */

function shorten(hash){
    if(!hash) return "";
    if(hash.length < 12) return hash;
    return hash.slice(0,6) + "..." + hash.slice(-4);
}

function formatTime(ts){
    if(!ts) return "";
    const d = new Date(ts);
    return d.toLocaleString();
}


/* START */

if(itemId){
    loadData();
}else{
    container.innerHTML =
        "<p style='color:white'>No Item ID provided</p>";
}