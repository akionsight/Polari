let triggered = false;

function startAnimation() {
    if (triggered) return;
    triggered = true;

    document.querySelector(".big").classList.add("zoom");
    document.querySelector(".small").classList.add("zoom");

    // Match these times with your CSS transition duration
    setTimeout(() => {
        document.getElementById("intro").style.background = "white";
    }, 1400);

    setTimeout(() => {
        document.getElementById("intro").remove();
    }, 1800);
}

window.addEventListener("click", startAnimation);
window.addEventListener("wheel", startAnimation);
window.addEventListener("touchstart", startAnimation);