FROM rust:1.81-alpine3.20@sha256:d6e876ca5fe200f4ac60312b95606f0b042699c4cf6a19493b7d2a2ebbfb337b AS chef
ENV RUSTFLAGS="-C target-feature=-crt-static"
ARG GIT_HASH
ENV GIT_COMMIT_HASH=${GIT_HASH}
RUN apk add --no-cache openssl-dev musl-dev
RUN cargo install cargo-chef 
WORKDIR /builder

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /builder/recipe.json recipe.json
RUN cargo chef cook --release
COPY . .
RUN cargo build --offline -rF production

FROM alpine:3.20@sha256:e72ad0747b9dc266fca31fb004580d316b6ae5b0fdbbb65f17bbe371a5b24cff
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /builder/target/release/rustbot .
CMD ./rustbot
