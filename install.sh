#!/bin/bash

echo building...
cargo build --release

echo copying file...
sudo cp target/release/oiff /usr/local/bin/oiff

echo installed!
