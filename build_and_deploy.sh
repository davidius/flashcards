#!/bin/bash
# Build and deploy the flashcards binary to /usr/local/bin
cargo build --release
cp target/release/flashcards /usr/local/bin