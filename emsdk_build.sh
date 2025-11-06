#!/usr/bin/env bash

git submodule update --init --recursive

rm -rf js_build
mkdir -p js_build
cd js_build

emcmake cmake -DCMAKE_BUILD_TYPE=Debug -G "Unix Makefiles" ..
emmake make
