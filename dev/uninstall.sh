#!/bin/bash

set -e

echo removing oiff...
sudo rm /usr/local/bin/oiff

echo removing oiff viewer...
sudo rm /usr/share/applications/oiff-viewer.desktop /usr/share/icons/hicolor/scalable/apps/oiff-viewer.svg

echo removing oiff thumbnailer...
sudo rm /usr/share/thumbnailers/oiff.thumbnailer /usr/share/mime/packages/oiff.xml

echo refreshing thumbnails...

rm -rf ~/.cache/thumbnails/*
sudo update-mime-database /usr/share/mime

echo Uninstalled!
