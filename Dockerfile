FROM rust:1.79-alpine3.19@sha256:aeeebbf58f579ef6037fd59f0917b31c89f7842155e24332de7d604e0284956e AS chef
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

FROM alpine:3.20@sha256:5f48f60d043e6df88720dea5f954dcf507912368cd84bd08703325fdf269724e
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /usr/src/rustbot/target/release/rustbot .
CMD ./rustbot
