#!/bin/bash

option="--check"

if [ "$1" == "--fix" ]; then
    option=""
    shift
fi

cargo +stable fmt $option --all -- "$@"
