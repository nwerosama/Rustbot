FROM rust:1.81-alpine3.20@sha256:e4ab5bdd6d6c93e984ba5d320691d7f4bddb1e061102a1def6ec203de8547472 AS chef
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

FROM alpine:3.20@sha256:6eee963cdd9be4b3423dd221bc4b5b0458a9c459990c0b5095b5aee7c43e92a2
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /builder/target/release/rustbot .
CMD ./rustbot
