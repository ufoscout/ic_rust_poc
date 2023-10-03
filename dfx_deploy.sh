#!/bin/sh

set -e

NETWORK="local"

start_canisters() {
    echo "Starting DFX"
    dfx start --clean --background --artificial-delay 0

    dfx deploy canister_b --network=$NETWORK --with-cycles 8000000000000

    # get canister_b principal
    canister_b_id=$(dfx canister --network=$NETWORK id canister_b)

    canister_a_init_args="(record { canister_b_principal=principal \"$canister_b_id\";})"

    dfx deploy canister_a --network=$NETWORK --with-cycles 8000000000000 --argument "$canister_a_init_args"
}

start_icx() {
    sleep 1

    # Get canister_a ID
    canister_a_id=$(dfx canister --network=$NETWORK id canister_a)

    # Start ICX Proxy
    dfx_local_port=$(dfx info replica-port)
    icx-proxy --fetch-root-key --address 127.0.0.1:8080 --dns-alias 127.0.0.1:$canister_a_id --replica http://localhost:$dfx_local_port &
}

./dfx_stop.sh

start_canisters

# icx proxy is needed to call the http_request endpoint from curl
# start_icx

