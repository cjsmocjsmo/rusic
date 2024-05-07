function PlayMovie(event) {
    var movid = event.target.getAttribute('data-movid');
    var url = 'http://192.168.0.97:8080/playmovie?movid=' + movid;

    fetch(url)
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(error => console.error('Error:', error));
}