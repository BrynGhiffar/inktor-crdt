#!/bin/bash
root=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

function wasm-build() {
    wasm-pack build --scope brynghiffar --out-name index --features debug
}

function wasm-publish() {
    if [[ -z "$1" ]]; then
        echo "Please specify version.";
        return 0;
    fi
    read -p "The published version will be $1 (y/n) " ans
    if [ "$ans" != 'y' ]; then
        echo 'Cancelled publishing';
        return 0;
    fi
    cd $root/pkg
    jq '.version = $version' --arg version $1 package.json > tmp.$$.json && mv tmp.$$.json package.json
    wasm-pack publish
    cd ..
}

# rm pkg/.gitignore
# rm vect-crdt/src/vect-crdt-rs/package.json