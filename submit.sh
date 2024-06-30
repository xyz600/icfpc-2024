#!/bin/bash

set -eux

cargo build --release

for i in `seq 1 13`; do
    ./target/release/message-sender efficiency-get --problem-id ${i} > dataset/problem/efficiency/${i}.txt
    sleep 5
done
