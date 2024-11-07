#!/bin/bash -e

PI_IP=raspberrypi
TARGET=arm-unknown-linux-musleabihf
USER=daniel

# validate if cross is installed for cross builds
if ! command -v cross 2>&1 >/dev/null
then
    cargo install -f cross
fi

cross build --release --target $TARGET
scp -r ./target/$TARGET/release/hello-world $USER@$PI_IP:/tmp/
ssh $USER@$PI_IP /tmp/hello-world
