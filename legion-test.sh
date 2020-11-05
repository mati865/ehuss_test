#!/bin/bash

ulimit -c unlimited

git clone https://github.com/amethyst/legion.git
cd legion
git checkout v0.3.1
rustc -V

for i in {1..500}
do
    echo $i
    touch src/lib.rs
    cargo check
done
