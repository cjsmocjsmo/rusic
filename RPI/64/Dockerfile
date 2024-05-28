FROM golang:bookworm

RUN \
    mkdir /usr/share/rusic && \
    chmod -R 755 /usr/share/rusic && \
    mkdir /usr/share/rusic/rusic && \
    chmod -R 755 /usr/share/rusic/rusic && \
    mkdir /usr/share/rusic/rusic/db && \
    chmod -R 755 /usr/share/rusic/rusic/db && \
    touch /usr/share/rusic/rusic/db/rusic.db && \
    mkdir /usr/share/rusic/rusic/assets && \
    chmod -R 755 /usr/share/rusic/rusic/assets && \
    mkdir /usr/share/rusic/rusic/assets/thumbnails && \
    chmod -R 755 /usr/share/rusic/rusic/assets/thumbnails && \
    mkdir /usr/share/rusic/rusic/nfo && \
    chmod -R 755 /usr/share/rusic/rusic/nfo

COPY assets/playing.jpg /usr/share/rusic/rusic/assets/playing.jpg
COPY assets/playing2.jpg /usr/share/rusic/rusic/assets/playing2.jpg
COPY go.mod /usr/share/rusic/rusic/go.mod
COPY go.sum /usr/share/rusic/rusic/go.sum
COPY main.go /usr/share/rusic/rusic/main.go
COPY rusic.go /usr/share/rusic/rusic/rusic.go
COPY rus.env /usr/share/rusic/rusic/rus.env
COPY rusicsetup /usr/share/rusic/rusic/rusicsetup

WORKDIR  /usr/share/rusic/rusic

RUN go mod download

RUN go build -o main .

EXPOSE 8080

CMD ["./main"]