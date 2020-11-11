#!/bin/bash

set -ex

# sudo xcode-select -s /Applications/Xcode_12.2.app

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
        -o build/test/a \
        -g

    # $(xcode-select -p)/usr/bin/python3
    $(xcode-select -p)/usr/bin/python3 lldb_batchmode.py build/test/a pretty-std-collections.debugger.script
done
