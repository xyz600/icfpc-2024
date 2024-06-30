#!/bin/bash

set -eux

pushd ../../../

cargo build --release

cp target/release/lambdaman-solver ./l-solver

parallel --result result-lambdaman --progress -j 8 "./l-solver < ./dataset/problem/lambdaman/{}.txt" ::: `seq 1 8`

popd