#!/bin/bash

export DOCKER_HOSTNAME=$(hostname)
export $(cat .env.bot | xargs)
clear && cargo run
unset DOCKER_HOSTNAME
