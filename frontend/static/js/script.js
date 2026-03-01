let triggered=false;

function startAnimation(){

    if(triggered) return;
    triggered=true;

    const star=document.querySelector(".big");
    const intro=document.getElementById("intro");

    star.classList.add("zoom");

    star.addEventListener("transitionend",()=>{

        intro.style.transition="opacity .8s ease";
        intro.style.opacity="0";

        setTimeout(()=>intro.remove(),800);

    },{once:true});
}

window.addEventListener("click",startAnimation);
window.addEventListener("wheel",startAnimation);
window.addEventListener("touchstart",startAnimation);


/* BUTTON */

document.addEventListener("DOMContentLoaded",()=>{

    const btn=document.getElementById("trackBtn");
    const input=document.getElementById("itemInput");
    const arrow=document.querySelector(".arrow");

    btn.onclick=()=>{
    const id=input.value.trim();
    if(!id) return;

    arrow.classList.add("move");

    document.getElementById("landing").classList.add("transitioning");

    setTimeout(()=>{
        window.location.href="track.html?id="+encodeURIComponent(id);
    },500);
};

});