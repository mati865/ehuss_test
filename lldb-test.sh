#!/bin/bash

set -ex

echo $(xcode-select -p)
lldb --version

export PYTHONUNBUFFERED=1
export PYTHONPATH=$(lldb -P)
# export DYLD_LIBRARY_PATH=`pwd`

mkdir -p build/test

for i in {1..10}
do
    echo run $i
    rustc pretty-std-collections.rs \
        -C prefer-dynamic \
        -o build/test/a \
        -Crpath \
        -g

    # $(xcode-select -p)/usr/bin/python3
    $(xcode-select -p)/usr/bin/python3 lldb_batchmode.py build/test/a pretty-std-collections.debugger.script
done
