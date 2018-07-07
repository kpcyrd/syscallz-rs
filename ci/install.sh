#!/bin/sh
sudo apt-get update -qq
sudo apt-get install -yq libseccomp-dev
rustup target add "$TARGET" || true
