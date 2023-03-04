#!/bin/bash

if command -v apt-get &> /dev/null; then
    echo "Package manager: apt"
    sudo apt-get install build-essential npm docker docker-compose
elif command -v dnf &> /dev/null; then
    echo "Package manager: dnf"
    sudo dnf install openssl-devel npm docker docker-compose gcc
elif command -v pacman &> /dev/null; then
    echo "Package manager: pacman"
    sudo pacman -S npm docker docker-compose base-devel
else
    echo "Your package manager has either not been found or isn't supported, please report this issue on github"
    exit
fi

if ! command -v rustup &> /dev/null
then
    echo "the rustup command could not be found, please ensure you installed it"
    exit
else
    rustup target add wasm32-unknown-unknown &> /dev/null
fi

if ! command -v cargo &> /dev/null
then
    echo "the cargo command could not be found, please ensure you installed it"
    exit
else
    cargo install trunk
    cargo install cargo-watch
fi
npx tailwindcss &> /dev/null

echo "Everything has been installed with success !" 