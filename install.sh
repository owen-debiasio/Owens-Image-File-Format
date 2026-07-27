#!/bin/bash

echo building...
rustup update
cargo update
cargo fmt
cargo build --release

echo copying oiff...
sudo cp target/release/oiff /usr/local/bin/oiff

echo copying oiff viewer...
sudo cp oiff-viewer/oiff-viewer.desktop /usr/share/applications/oiff-viewer.desktop

echo copying oiff viewer icon...
sudo cp oiff-viewer/oiff-viewer.svg /usr/share/icons/oiff-viewer.svg

echo copying thumbnailer...
sudo cp thumbnailer/oiff.thumbnailer /usr/share/thumbnailers/oiff.thumbnailer

echo copying thumbnailer xml...
sudo cp thumbnailer/oiff.xml /usr/share/mime/packages/oiff.xml

echo refreshing thumbnails...

rm -rf ~/.cache/thumbnails/*
sudo update-mime-database /usr/share/mime

echo installed!
