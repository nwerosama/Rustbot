#!/bin/bash

export $(grep -v '^#' .env.bot | xargs)
clear && cargo fmt && RUST_LOG=debug cargo run
unset $(grep -v '^#' .env.bot | cut -d= -f1)
