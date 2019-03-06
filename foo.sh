#!/usr/bin/env bash

echo start sccache
SCCACHE_IDLE_TIMEOUT=10800 sccache --start-server || true

for x in {1..10}
do
    echo "sleeping"
    date
    sleep 30
done

echo "tasklist:"
tasklist
echo -n "location of sh: "
where sh
