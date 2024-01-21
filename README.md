# Installation

## Install Rust

Follow the instructions at https://www.rust-lang.org/tools/install

## Install the WebAssembly target

`rustup target add wasm32-unknown-unknown`

## Install Trunk

`cargo install --locked trunk`

# Deployment

## Copy new airspace files

Copy the files yaixm.json, openair.txt, overlay_105.txt,
overlay_195.txt, and overlay_atzdz.txt to the data directory.

# Build

`trunk build --release`

## Deploy

`./deploy.sh`
