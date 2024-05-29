# rusicsetup

![Project Screenshot](screenshot.png "width=400px")

rusicsetup is part of the rusicsetup and rusic trio (rusic, rusicsetup, rusic-svelte).
Rusic has been designed to run on the raspberry pi 3 and above.

Rust was choosen for it's speed.  Rusicsetup takes approx 3min to go through 2100 mp3s.

## Prerequisites

1. raspberrypiOS (bookworm)
2. rust



## Usage

1. Create the directory /usr/share/rusicsetup
2. Adjust permissions as needed to do the next step
3. Git clone this repository to /usr/share/rusicsetup/
4. Edit .env as needed for your setup (pagination, address, port, etc...)
5. Execute RUN.sh

```bash

#insure rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


```