#!/bin/bash

set -eux

cargo build --release

for i in `seq 1 21`; do
    encoded_path=dataset/encoded/lambdaman/${i}.txt
    decoded_path=dataset/decoded/lambdaman/${i}.txt

    ./target/release/message-sender --encode direct --message "get lambdaman${i}" > ${encoded_path}
    ./target/release/translator --file ${encoded_path} > ${decoded_path}

    echo "${i} finished"
done