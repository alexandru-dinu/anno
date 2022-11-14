#!/usr/bin/env bash

FILE=/tmp/foo.bin


for i in $(seq 1 80)
do
    dd if=/dev/urandom of=${FILE} bs=128M count=${i} 2> /dev/null

    stat --printf="%s" ${FILE}
    time sha256sum ${FILE} > /dev/null

    echo
done
