#!/bin/bash

rm -rf target
rm Cargo.lock

cargo build

# update custom denpendency
cargo update -p webpki --precise 0.18.1
cargo update -p hyper-rustls --precise 0.15.1
cargo update -p hyper --precise 0.12.19
cargo update -p http --precise 0.1.14

cargo build

cargo test

# run examples
for example in `ls examples`
do
    len=${#example}
    example=${example:0:$len-3}
    cargo run --example $example > /dev/null
done
