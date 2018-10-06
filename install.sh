#!/bin/bash

set -e

cargo clean
cargo build --release

cp target/release/logg ~/bin