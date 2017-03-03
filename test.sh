#!/bin/bash

cargo build --verbose
cargo test --verbose
# cargo test -- --nocapture

while [ $? -eq 150 ]; do
    fg
done