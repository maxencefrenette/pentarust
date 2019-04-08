#!/bin/bash

cargo build --release --features "baseline"
cp target/release/libpentarust.so java/data/libBaseline.so
cargo build --release
