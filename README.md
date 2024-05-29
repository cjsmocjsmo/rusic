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
4. Edit rus.env as needed for your setup (pagination, address, port, etc...)
5. Execute run.sh

```bash
mkdir /usr/share/rusic
git clone https://github.com/cjsmocjsmo/rusic.git
cs /usr/share/rusic/rusic

#rpi3
sh run.sh 32 0.0.1

#rpi4 and above
sh run.sh 64 0.0.1

```

## Updating

To update simply re run RUN.sh with a new version

```bash
sh RUN.sh 32 0.0.2

#or 

sh RUN.sh 64 0.0.2
```

## Warning

Rusic was designed to run on your home network and not on the wider internet.  It has no authentication system so you have been warned!!!

Rusic is very much a work in progress so there may be some breaking changes in the future.