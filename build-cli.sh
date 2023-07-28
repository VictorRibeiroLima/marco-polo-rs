#!/bin/sh

cargo build --release --package marco-polo-rs-cli --bin marco-polo-rs-cli && cp target/release/marco-polo-rs-cli ./bin