#!/bin/bash

if ! which llc >/dev/null 2>&1; then
  apt install -y llvm-19
fi


if [[ ! -f ~/.cargo/env ]]
then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . $HOME/.cargo/env
  rustup +nightly target add nvptx64-nvidia-cuda
fi
