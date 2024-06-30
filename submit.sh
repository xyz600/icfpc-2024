#!/bin/bash

set -eux

for i in `seq 1 20`; do
    ./target/release/message-sender lambdaman-submit --problem-id ${i} --filepath ./dataset/solution/lambdaman/${i}.txt
    sleep 5
done