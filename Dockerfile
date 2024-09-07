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

FROM alpine:3.20@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /builder/target/release/rustbot .
CMD ./rustbot
