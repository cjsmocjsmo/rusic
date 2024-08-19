#!/bin/sh

# Check if the number of arguments is 2
# If not, print an error message and exit the script
if [ "$#" -ne 2 ]; then
    echo "You must enter exactly 2 arguments:\n\tarchitecture (32 or 64) and version (e.g., 0.0.1)"
    exit 1
fi

# Check if the first argument is either 32 or 64
# If not, print an error message and exit the script
if [ "$1" != "32" ] && [ "$1" != "64" ]; then
    echo "The first argument must be either 32 or 64"
    exit 1
fi

# Check if the second argument is a valid version string
# If not, print an error message and exit the script
if ! echo "$2" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "The second argument must be a valid version string (e.g., 0.0.1)"
    exit 1
fi

# If all checks pass, print the arguments
echo "Architecture: $1"
echo "Version: $2"

# Check if the rusicsetup directory exists
# If not, print an error message and exit the script
if [ ! -d /usr/share/rusicsetup/rusicsetup ]; then
    echo "rusicsetup must be installed before running this script"
    exit 1
fi

if [ "$1" = "32" ]; then
    if [ "$(uname -m)" = "aarch64" ]; then
        echo "ERROR: This is a 64-bit ARM system."
        exit 1
    fi
fi

if [ "$1" = "64" ]; then
    if [ "$(uname -m)" = "armv7l" ]; then
        echo "ERROR: This is a 32-bit ARM system."
        exit 1
    fi
fi

git pull https://github.com/cjsmocjsmo/rusic.git

# If the first argument is 32, execute the following commands
if [ "$1" = "32" ]; then
    cp -pvr RPI/32/Dockerfile .
fi

if [ "$1" = "64" ]; then
    cp -pvr RPI/64/Dockerfile .
fi

count1=$(echo "$2" | sed 's/\.//g' )
count=$((count1+1-1))
minusone=$((count-1))

echo "Version: $2";
echo "rusic:$2";
echo "rusic$count";
echo "rusic$minusone";

if [ "$minusone" -eq 0 ]; then
    docker build -t rusic:$2 .

    # Run the Docker container
    docker run \
        --name rusic1
        -e RUS_DB_PATH=/usr/share/rusic/rusic/db/rusic.db \
        -e RUS_DB_CHECK_FILE_PATH=/usr/share/rusic/rusic/db/db_check_file.txt \
        -e RUS_THUMBS=/usr/share/rusic/rusic/thumbnails \
        -e RUS_NFOS=/usr/share/rusic/rusic/nfo \
        -e RUS_RAW_HTTP=10.0.4.31 \
        -e RUS_HTTP_ADDR=http://10.0.4.31 \
        -e RUS_PORT=:8080 \
        -d \
        -p 8080:8080 \
        -v /usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/ \
        -v /usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/ \
        -v $HOME/Music:/usr/share/rusic/rusic/Music \
        rusic:"$2" 

    # Remove the Dockerfile
    rm Dockerfile
fi

if [ "$minusone" -eq 1 ]; then
    docker stop rusic1;

    docker rm rusic1;

    docker build -t rusic:$2 .

    # Run the Docker container
    docker run \
        --name rusic$count
        -e RUS_DB_PATH=/usr/share/rusic/rusic/db/rusic.db \
        -e RUS_DB_CHECK_FILE_PATH=/usr/share/rusic/rusic/db/db_check_file.txt \
        -e RUS_THUMBS=/usr/share/rusic/rusic/thumbnails \
        -e RUS_NFOS=/usr/share/rusic/rusic/nfo \
        -e RUS_RAW_HTTP=192.168.0.91 \
        -e RUS_HTTP_ADDR=http://10.0.4.31 \
        -e RUS_PORT=:8080 \
        -d \
        -p 8080:8080 \
        -v /usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/ \
        -v /usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/ \
        -v $HOME/Music:/usr/share/rusic/rusic/Music \
        rusic:"$2" 

    # Remove the Dockerfile
    rm Dockerfile
fi 


if [ "$minusone" -gt 1 ]; then
    docker stop rusic$count;

    docker rm rusic$count;

    docker build -t rusic:$2 .

    # Run the Docker container
    docker run \
        --name rusic$count
        -e RUS_DB_PATH=/usr/share/rusic/rusic/db/rusic.db \
        -e RUS_DB_CHECK_FILE_PATH=/usr/share/rusic/rusic/db/db_check_file.txt \
        -e RUS_THUMBS=/usr/share/rusic/rusic/thumbnails \
        -e RUS_NFOS=/usr/share/rusic/rusic/nfo \
        -e RUS_RAW_HTTP=192.168.0.91 \
        -e RUS_HTTP_ADDR=http://10.0.4.31 \
        -e RUS_PORT=:8080 \
        -d \
        -p 8080:8080 \
        -v /usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/ \
        -v /usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/ \
        -v $HOME/Music:/usr/share/rusic/rusic/Music \
        rusic:"$2" 

    # Remove the Dockerfile
    rm Dockerfile
fi 