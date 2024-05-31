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

# If the first argument is 32, execute the following commands
if [ "$1" = "32" ]; then
    if [ "$(uname -m)" = "aarch64" ]; then
        echo "This is a 64-bit ARM system."
        exit 1
    fi
    # Copy the Dockerfile for the 32-bit architecture to the current directory
    cp -pvr RPI/32/Dockerfile .
    # Build the Docker image
    docker build -t rusic:$2 .
    # Run the Docker container
    docker run \
    -e RUS_DB_PATH=/usr/share/rusicsetup/rusicsetup/db/rusic.db \
    -e RUS_DB_CHECK_FILE_PATH=/usr/share/rusicsetup/rusicsetup/db/db_check_file.txt \
    -e RUS_THUMBS=/usr/share/rusicsetup/rusicsetup/thumbnails \
    -e RUS_NFOS=/usr/share/rusicsetup/rusicsetup/nfo \
    -e RUS_RAW_HTTP=192.168.0.91 \
    -e RUS_HTTP_ADDR=http://192.168.0.91 \
    -e RUS_PORT=:8080 \
    -d \
    -p 8080:8080 \
    -v /usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/ \
    -v /usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/ \
    -v $HOME/Music:/usr/share/rusic/rusic/Music \
    rusic:$2 
    # Remove the Dockerfile
    rm Dockerfile
    
else
    if [ "$(uname -m)" = "armv7l" ]; then
        echo "This is a 32-bit ARM system."
        exit 1
    fi
    # Copy the Dockerfile for the 64-bit architecture to the current directory
    cp -pvr RPI/64/Dockerfile .
    # Build the Docker image
    docker build -t rusic:$2 .
    # Run the Docker container
    docker run \
    -e RUS_DB_PATH=/usr/share/rusicsetup/rusicsetup/db/rusic.db \
    -e RUS_DB_CHECK_FILE_PATH=/usr/share/rusicsetup/rusicsetup/db/db_check_file.txt \
    -e RUS_THUMBS=/usr/share/rusicsetup/rusicsetup/thumbnails \
    -e RUS_NFOS=/usr/share/rusicsetup/rusicsetup/nfo \
    -e RUS_RAW_HTTP=192.168.0.91 \
    -e RUS_HTTP_ADDR=http://192.168.0.91 \
    -e RUS_PORT=:8080 \
    -d \
    -p 8080:8080 \
    -v /usr/share/rusicsetup/rusicsetup/db/:/usr/share/rusic/rusic/db/ \
    -v /usr/share/rusicsetup/rusicsetup/thumbnails/:/usr/share/rusic/rusic/thumbnails/ \
    -v $HOME/Music:/usr/share/rusic/rusic/Music \
    rusic:$2 
    # Remove the Dockerfile
    rm Dockerfile
fi