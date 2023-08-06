#!/bin/sh

rm -rf ./nfo;
rm -rf ./thumbs;
rm -rf ./db;

mkdir ./nfo;
mkdir ./thumbs;
mkdir ./db;
touch ./db/rusic.db;
sudo chmod +rw ./db/rusic.db