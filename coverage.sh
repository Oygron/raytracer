#!/bin/sh
#source https://blog.rng0.io/how-to-do-code-coverage-in-rust/

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
rm *.profraw
firefox target/coverage/html/html/index.html &