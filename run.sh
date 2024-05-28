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



# Check if Git is installed
# If not, print an error message and exit the script
if ! command -v git &> /dev/null; then
    echo "Git is not installed. Please install Git before running this script."
    exit 1
fi

# Check if Docker is installed
# If not, print an error message and exit the script
if ! command -v docker &> /dev/null; then
    echo "Docker is not installed. Please install Docker before running this script."
    exit 1
fi

# If the first argument is 32, execute the following commands
if [ "$1" = "32" ]; then
    
        
    # Copy the Dockerfile for the 32-bit architecture to the current directory
    cp -pvr RPI/32/Dockerfile .
    # Build the Docker image
    docker build -t rusic:$2 .
    # Run the Docker container
    docker run \
    -d \
    -p 8080:80 \
    -v /usr/share/rusic/rusic/db/rusic.db:/usr/share/rusic/rusic/db/rusic.db \
    -v /usr/share/rusic/rusic/assets/:/usr/share/rusic/rusic/assets/ \
    -v $HOME/Music:/usr/share/rusic/rusic/assets/ \
    rusic:$2 
    # Remove the Dockerfile
    rm Dockerfile
    
else
    
   
    # Copy the Dockerfile for the 64-bit architecture to the current directory
    cp -pvr RPI/64/Dockerfile .
    # Build the Docker image
    docker build -t rusic:$2 .
    # Run the Docker container
    docker run \
    -d \
    -p 8080:80 \
    -v /usr/share/rusic/rusic/db/rusic.db:/usr/share/rusic/rusic/db/rusic.db \
    -v /usr/share/rusic/rusic/assets/:/usr/share/rusic/rusic/assets/ \
    -v $HOME/Music:/usr/share/rusic/rusic/assets/ \
    rusic:$2 
    # Remove the Dockerfile
    rm Dockerfile
fi