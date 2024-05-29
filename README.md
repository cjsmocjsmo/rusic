# rusic

![Project Screenshot](screenshot.png "width=400px")

rusic is part of the rusic trio (rusicsetup, rusic, rusic-svelte).
Rusic has been designed to run on the raspberry pi 3 and above.

Install sequence:

1. Run rusicsetup
2. Run rusic
3. Run rusic-svelte

## Prerequisites

1. raspberrypiOS (bookworm)
2. docker

## Usage

1. Create the directory /usr/share/rusic
2. Adjust permissions as needed to do the next step
3. Git clone this repository to /usr/share/rusic/
4. Edit .env as needed for your setup (pagination, address, port, etc...)
5. Execute RUN.sh

```bash

#insure rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


```