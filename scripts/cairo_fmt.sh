#!/bin/bash

option="--check"

if [ "$1" == "--fix" ]; then
    option=""
fi

scarb --manifest-path Scarb.toml fmt $option
