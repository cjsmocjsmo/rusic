

function PlayMusic(event) {
    var musid = event.target.getAttribute('data-musid');
    var url = 'http://192.168.0.97:8080/playmovie?musid=' + musid;

    fetch(url)
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(error => console.error('Error:', error));
}