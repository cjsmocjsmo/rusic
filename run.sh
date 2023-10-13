#!/bin/bash
RUSIC="/usr/share/rusic/rusic"
echo "stoping rusic.service";
if [ -f /etc/systemd/system/rusic.service ]; then
    sudo systemctl stop rusic.service;
fi
cd $RUSIC;
sudo rm -rf $RUSIC/thumbnails;
sudo rm -rf $RUSIC/db;
mkdir $RUSIC/db;
touch $RUSIC/db/rusic.db;
git pull;
cargo build --release --bin rusic;
sudo mv ./target/release/rusic /usr/bin/;
sudo chmod +xr /usr/bin/rusic;
sudo chown root:root /usr/bin/rusic;
sudo systemctl daemon-reload;
sudo systemctl start rusic.service;
sudo systemctl status rusic.service
