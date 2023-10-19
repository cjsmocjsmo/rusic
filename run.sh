#!/bin/bash

echo "stoping rusic.service";
if [ -f /etc/systemd/system/rusic.service ]; then
    sudo systemctl stop rusic.service;
fi

RUSIC="/usr/share/rusic/rusic"

sudo rm -rf $RUSIC/thumbs;
mkdir $RUSIC/thumbs;

sudo rm -rf $RUSIC/db;
mkdir $RUSIC/db;
touch $RUSIC/db/rusic.db;

sudo rm -rf $RUSIC/nfo;
mkdir $RUSIC/nfo;

cd $RUSIC;
git pull;
cargo build --release --bin rusic;
sudo mv ./target/release/rusic /usr/bin/;
sudo chmod +xr /usr/bin/rusic;
sudo chown root:root /usr/bin/rusic;
sudo systemctl daemon-reload;
sudo systemctl start rusic.service;
sudo systemctl status rusic.service
