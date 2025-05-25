#!/bin/bash

. $HOME/.cargo/env

rm -rf output/
mkdir -p output/

# Build with cargo
pushd kernel
cargo +nightly build --release --target nvptx64-nvidia-cuda
popd

# Find the generated .ll file (cargo puts it in a nested directory)
find kernel/target/nvptx64-nvidia-cuda/release/deps -name "*.ll" -exec cp {} output/kernel.ll \;

# Convert .ll to .ptx
llc -march=nvptx64 -mcpu=sm_75 output/kernel.ll -o output/kernel.ptx

# Convert .ptx to a C string
echo 'const char* ptx_code = R"(' > output/ptx_code.h
cat output/kernel.ptx >> output/ptx_code.h
echo ')";' >> output/ptx_code.h

# Compile the runner
nvcc -arch=sm_75 -lcuda runner.cu -o output/runner

