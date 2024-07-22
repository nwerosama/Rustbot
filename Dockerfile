FROM rust:1.79-alpine3.19@sha256:b5d5bb4fa17a7fd926f64937ea0f7f57dcabf73de62f4fa89fd83f4758cdc904 AS chef
ENV RUSTFLAGS -C target-feature=-crt-static
RUN apk add --no-cache openssl-dev musl-dev
RUN cargo install cargo-chef 
WORKDIR /usr/src/rustbot

FROM chef AS planner
COPY . .
RUN mkdir -p .cargo && \
  printf '[registries.gitea]\nindex = "sparse+https://git.toast-server.net/api/packages/toast/cargo/"\ntoken = "Bearer %s"\n' "$CARGO_TOKEN" >> .cargo/config.toml
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /usr/src/rustbot/recipe.json recipe.json
RUN cargo chef cook --release
COPY . .
RUN cargo build -r

FROM alpine:3.20@sha256:a59bbcb5abd79dd098c5f2de7f603aa7a90edf2d04f7c4a6fecc4eac08836290
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /usr/src/rustbot/target/release/rustbot .
CMD ./rustbot
