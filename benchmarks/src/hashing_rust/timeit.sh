#!/usr/bin/env bash

FILE=/tmp/foo.bin

cargo clean
cargo build --release

for i in $(seq 1 80)
do
    dd if=/dev/urandom of=${FILE} bs=128M count=${i} 2> /dev/null

    stat --printf="%s" ${FILE}
    time ./target/release/sha256_rust > /dev/null

    echo
done
