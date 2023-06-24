var AudioContex = window.AudioContex || window.webkitAudioContex;
var audioCtx = new AudioContext();
var audioElem= document.querySelector("audio");
var audioSourse = audioCtx.createMediaElementSource(audioElem);
var strtBtn = document.querySelector("button");
var valumeSlider = document.querySelector(".volume");
var gainNode = audioCtx.createGain();
var flag = true;

audioSourse.connect(gainNode).connect(audioCtx.destination);

strtBtn.addEventListener("click", () => {
    if (audioCtx.state == "suspended") {
        audioCtx.resume();
    }
    if (flag) {
        audioElem.play().then(() => {
            flag = false;
            strtBtn.textContent = "Pause";
        })
    }
    else {
        audioElem.pause();
        // .then(() => {
            flag = true;
            strtBtn.textContent = "Play";
        // })
    }
})



valumeSlider.addEventListener("input",() => {
    gainNode.gain.value = valumeSlider.value;
})





