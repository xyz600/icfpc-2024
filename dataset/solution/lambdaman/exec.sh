#!/bin/bash

set -eux

pushd ../../../

cargo build --release

for i in `seq 1 20`; do
    input_path=dataset/decoded/lambdaman/${i}.txt
    output_path=dataset/solution/lambdaman/${i}.txt
    ./target/release/lambdaman-solver < ${input_path} > ${output_path} 
done

popd