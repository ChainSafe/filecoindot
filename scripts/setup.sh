#!/bin/bash

# paths
readonly ROOT="$(dirname $(cd $(dirname $0) && pwd))"
readonly SCRIPTS="${ROOT}/scripts"
readonly CONFIG="${SCRIPTS}/setup.json"

# build javascript packages
run_setup_script() {
    if [ -x "$(command -v yarn)" ]; then
        yarn && yarn setup $@;
    elif [ -x "$(command -v npm)" ]; then
        npm install && npm run setup $@;
    else
        echo "Error: could not find npm or yarn for building setup scripts";
        exit 1;
    fi
}

# run program
main() {
    cd $ROOT

    run_setup_script $CONFIG

    exit 0;
}

main
