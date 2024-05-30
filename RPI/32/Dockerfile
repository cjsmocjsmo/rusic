FROM arm32v7/debian:bookworm

RUN \
    apt-get update && \
    apt-get dist-upgrade && \
    apt-get -y install golang nano git && \
    apt-get -y autoremove && \
    apt-get -y autoclean

RUN \
    mkdir /usr/share/rusic && \
    chmod -R 755 /usr/share/rusic && \
    mkdir /usr/share/rusic/rusic && \
    chmod -R 755 /usr/share/rusic/rusic && \
    mkdir /usr/share/rusic/rusic/db && \
    chmod -R 755 /usr/share/rusic/rusic/db && \
    touch /usr/share/rusic/rusic/db/rusic.db && \
    mkdir /usr/share/rusic/rusic/thumbnails && \
    chmod -R 755 /usr/share/rusic/rusic/thumbnails && \
    mkdir /usr/share/rusic/rusic/Music && \
    chmod -R 755 /usr/share/rusic/rusic/Music && \
    mkdir /usr/share/rusic/rusic/nfo && \
    chmod -R 755 /usr/share/rusic/rusic/nfo && \
    touch /usr/share/rusic/rusic/log.txt

COPY playing.jpg /usr/share/rusic/rusic/thumbnails/playing.jpg
COPY go.mod /usr/share/rusic/rusic/go.mod
COPY go.sum /usr/share/rusic/rusic/go.sum
COPY main.go /usr/share/rusic/rusic/main.go
COPY rusic.go /usr/share/rusic/rusic/rusic.go

WORKDIR  /usr/share/rusic/rusic

# RUN go mod download

RUN go build -o main .

EXPOSE 8080

CMD ["./main"]