#!/bin/bash

set -eux

pushd ../../../

cargo build --release

cp target/release/lambdaman-solver ./l-solver

parallel --result result-lambdaman --progress -j 12 "./l-solver < ./dataset/decoded/lambdaman/{}.txt" ::: `seq 9 20`

popd