#!/bin/bash

set -eux

cargo build --release

pushd ../../

if [ ! -d dataset/encoded/spaceship ]; then
    mkdir -p dataset/encoded/spaceship
fi

if [ ! -d dataset/decoded/spaceship ]; then
    mkdir -p dataset/decoded/spaceship
fi


for i in `seq 21 25`; do
    encoded_path=dataset/encoded/spaceship/${i}.txt
    decoded_path=dataset/decoded/spaceship/${i}.txt

    ./target/release/message-sender --encode direct --message "get spaceship${i}" > ${encoded_path}
    ./target/release/translator --file ${encoded_path} > ${decoded_path}

    echo "${i} finished"
done

popd
