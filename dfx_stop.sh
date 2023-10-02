#!/bin/sh

set -e

echo "Stopping DFX if running"
dfx stop
killall icx-proxy || echo "Process was not running."
