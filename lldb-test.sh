#!/bin/bash

set -ex

echo $(xcode-select -p)
lldb --version

export PYTHONUNBUFFERED=1
export PYTHONPATH=$(lldb -P)
export DYLD_LIBRARY_PATH=`pwd`

for i in {1..1000}
do
    $(xcode-select -p)/usr/bin/python3 lldb_batchmode.py pretty-std-collections pretty-std-collections.debugger.script
done
