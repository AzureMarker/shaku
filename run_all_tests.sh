#!/bin/bash
cd shaku
cargo test
cd ../shaku_derive
cargo test
cd ../examples/autofac
cargo run