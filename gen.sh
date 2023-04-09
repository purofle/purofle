#!/usr/bin/env bash

cargo build --release
PATH=$PATH:$(pwd)/target/release vhs --publish purofle.tape