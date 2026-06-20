# rusic

![Project Screenshot](screenshot.png "width=400px")

rusic is part of the rusic trio (rusicsetup, rusic, rusic-svelte).
Rusic has been designed to run on the raspberry pi 3b+, 4 and 5.
This project will probably run on any debian based system.

Install sequence:

1. Run rusic
2. Run rusic-svelte

## Prerequisites

1. raspberrypiOS (bookworm/trixie)
2. docker installed and running

## Usage

1. Create the directories:
    /usr/share/rusic
    /usr/share/rusic/db
    /usr/share/rusic/thumbs
2. Adjust permissions as needed to do the next step
3. Git clone this repository to /usr/share/rusic/
    ```bash
    cd /usr/share/rusic
    git clone https://github.com/cjsmocjsmo/rusic
    ```
   Your should have the folder layout such as:
    /usr/share/rusic/rusic
4. Execute rusic.py:
    ```bash
    python3 rusic.py -i 0.0.1

```bash
mkdir /usr/share/rusic
mkdir /usr/share/rusic/db
mkdir /usr/share/rusic/thumbs

cd /usr/share/rusic

git clone https://github.com/cjsmocjsmo/rusic.git

cd /usr/share/rusic/rusic

python3 rusic -i 0.0.1

```

The source code for the rust binaries in /rusic/setup can be found at:
    https://github.com/cjsmocjsmo/rusicsetup

## Updating

This is still a work in progress

## Warning

Rusic was designed to run on your home network and not on the wider internet.  It has no authentication system or TLS support, so you have been warned!!!

Rusic is very much a work in progress so there may be some breaking changes in the future.