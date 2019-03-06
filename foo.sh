#!/usr/bin/env bash

echo start sccache
SCCACHE_IDLE_TIMEOUT=10800 sccache --start-server || true

sleep 10

echo "tasklist:"
tasklist
echo -n "location of sh: "
where sh
