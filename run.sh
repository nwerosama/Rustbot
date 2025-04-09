#!/bin/bash

export $(grep -v '^#' .env.bot | xargs)
clear && cargo fmt && cargo run
unset $(grep -v '^#' .env.bot | cut -d= -f1)
