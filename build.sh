#!/bin/bash

cargo build --locked -rF production && \
docker compose build bot
