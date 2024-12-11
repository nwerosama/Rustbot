#!/bin/bash

export $(cat .env.bot | xargs)
clear && cargo fmt && cargo run
