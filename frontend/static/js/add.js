const form = document.getElementById("addForm");

form.addEventListener("submit", e => {
    e.preventDefault();

    const data = {
        item_id: itemId.value,
        event_type: eventType.value,
        location_id: locationId.value,
        owner: owner.value,
    };

    fetch("http://10.75.55.175:8000/add-item/", {
  method: "POST",
  body: JSON.stringify(data),
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
});
    console.log(data);
    alert("Item added successfully!");
    form.reset();
});