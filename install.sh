#!/usr/bin/env bash

check=0

if test $UID -eq $check
then
    echo "[Kalem.rs] Use without super-user permissions"
    echo "bash install.sh"
    exit 0
fi

$HOME/.cargo/bin/cargo install --path .

sudo sh scripts/stl.sh
