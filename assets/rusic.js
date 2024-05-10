
function PlayMusic(path) {
    var audio = document.getElementById('audio1');
    audio.src = path;
    audio.play();
}

function PlayMusic(event) {
    var musid = event.target.getAttribute('data-musid');
    var url = 'http://192.168.0.97:8080/playmusic?musid=' + musid;
    console.log(url);

    fetch(url)
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(error => console.error('Error:', error));
}