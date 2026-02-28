// const container = document.getElementById("recordsContainer");

// /* GET ITEM ID FROM URL */

// const params = new URLSearchParams(window.location.search);
// const itemId = params.get("id");


// /* ================= FETCH DATA ================= */

// async function loadData(){

//     try{

//         const res = await fetch(`/api/item?id=${itemId}`);

//         if(!res.ok) throw new Error("Server error");

//         const data = await res.json();

//         if(!Array.isArray(data))
//             throw new Error("Invalid JSON");

//         render(data);

//     }catch(err){
//         console.error(err);
//         container.innerHTML="<p style='color:white'>Failed to load data</p>";
//     }
// }


// /* ================= RENDER ================= */

// function render(records){

//     container.innerHTML="";

//     if(records.length===0){
//         container.innerHTML="<p style='color:white'>No records found</p>";
//         return;
//     }

//     records.forEach(r=>{

//         const card=document.createElement("div");
//         card.className="record";

//         card.innerHTML=`
//             ${field("Item ID",r.item_id)}
//             ${field("Event",r.event_type)}
//             ${field("Location",r.location)}
//             ${field("Timestamp",r.timestamp)}
//             ${field("Owner",r.owner)}
//             ${field("Hash",shorten(r.document_hash))}
//         `;

//         container.appendChild(card);
//     });
// }


// /* ================= HELPERS ================= */

// function field(label,value){
//     return `
//         <div class="field-block">
//             <div class="icon"></div>
//             <div class="label">${label}</div>
//             <div class="value">${value ?? ""}</div>
//         </div>
//     `;
// }

// function shorten(hash){
//     if(!hash) return "";
//     if(hash.length<12) return hash;
//     return hash.slice(0,6)+"..."+hash.slice(-4);
// }


// /* ================= START ================= */

// loadData();
const container = document.getElementById("recordsContainer");


/* ================= ICON MAP ================= */

const iconMap = {
    item_id: "../static/images/item.svg",
    event_type: "../static/images/event.svg",
    location: "../static/images/location.svg",
    timestamp: "../static/images/time.svg",
    owner: "../static/images/owner.svg",
    document_hash: "../static/images/hash.svg"
};


/* ================= DUMMY TEST DATA ================= */

const mockData = [
    {
        item_id:"POL-001",
        event_type:"Manufactured",
        location:"Seoul",
        timestamp:"2026-03-01 10:15",
        owner:"Factory A",
        document_hash:"0xA91F3B8821DF11AB"
    },
    {
        item_id:"POL-001",
        event_type:"Quality Check",
        location:"Seoul",
        timestamp:"2026-03-02 09:30",
        owner:"QC Dept",
        document_hash:"0xB72F8D91AC3312FF"
    },
    {
        item_id:"POL-001",
        event_type:"Exported",
        location:"Busan Port",
        timestamp:"2026-03-03 14:05",
        owner:"Logistics Co",
        document_hash:"0xC91AB8821DF99EAA"
    },
    {
        item_id:"POL-001",
        event_type:"Delivered",
        location:"Customer Address",
        timestamp:"2026-03-08 13:10",
        owner:"End User",
        document_hash:"0xF9921CC881EE7722"
    },
    {
        item_id:"POL-001",
        event_type:"Delivered",
        location:"Customer Address",
        timestamp:"2026-03-08 13:10",
        owner:"End User",
        document_hash:"0xF9921CC881EE7722"
    }
];


/* ================= RENDER FUNCTION ================= */

function render(records){

    container.innerHTML = "";

    if(records.length === 0){
        container.innerHTML = "<p style='color:white'>No records found</p>";
        return;
    }

    records.forEach(r => {

        const card = document.createElement("div");
        card.className = "record";

        card.innerHTML = `
            ${field("Item ID", r.item_id, "item_id")}
            ${field("Event", r.event_type, "event_type")}
            ${field("Location", r.location, "location")}
            ${field("Timestamp", r.timestamp, "timestamp")}
            ${field("Owner", r.owner, "owner")}
            ${field("Hash", shorten(r.document_hash), "document_hash")}
        `;

        container.appendChild(card);
    });
}


/* ================= FIELD BUILDER ================= */

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


/* ================= HASH SHORTENER ================= */

function shorten(hash){
    if(!hash) return "";
    if(hash.length < 12) return hash;
    return hash.slice(0,6) + "..." + hash.slice(-4);
}


/* ================= START ================= */

render(mockData);