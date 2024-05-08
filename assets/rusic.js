
function PlayMusic(path) {
    var audio = document.getElementById('audio1');
    audio.src = path;
    audio.play();
}

function PlayMusic2(path) {
    player = Audio(path);
    player.play();
}