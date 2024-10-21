#!/bin/bash

export GIT_COMMIT_HASH=$(git rev-parse HEAD) && \
cargo zigbuild --target x86_64-unknown-linux-musl --locked -rF production && \
docker compose build bot
