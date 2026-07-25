#!/bin/bash

echo building...
rustfmt src/*
cargo build --release

echo copying oiff...
sudo cp target/release/oiff /usr/local/bin/oiff

echo copying thumbnailer...
sudo cp thumbnailer/oiff.thumbnailer /usr/share/thumbnailers/oiff.thumbnailer

echo copying thumbnailer xml...
sudo cp thumbnailer/oiff.xml /usr/share/mime/packages/oiff.xml

echo refreshing thumbnails...

rm -rf ~/.cache/thumbnails/*
sudo update-mime-database /usr/share/mime

echo installed!
