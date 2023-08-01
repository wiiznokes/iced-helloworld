#!/bin/bash
set -e
clear

if [ "$1" == "-f" ]; then
    cargo clippy --all --fix --allow-dirty --allow-staged && cargo fmt --all
fi

if [ "$1" == "-ff" ]; then
    cargo clippy --all --fix --allow-dirty --allow-staged && cargo fmt --all
    exit 0
fi



if [ "$1" == "-b" ]; then
    cargo build
    exit 0
fi


if [ "$1" == "-l" ]; then
    ./target/release/note $2
    exit 0
fi


if [ "$1" == "-c" ]; then
    ./create_files.sh
fi

if [ "$1" == "-r" ]; then
    cargo build --release
    ./target/release/note tmp
else
    cargo build
    ./target/debug/note tmp
fi


