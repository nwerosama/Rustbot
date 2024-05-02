FROM rust:1.78-alpine3.19@sha256:9b421636ae538fda3c7b98d52a5048d2a09abb7ff76e0298709cc2509fd045a7 AS chef
ENV RUSTFLAGS -C target-feature=-crt-static
RUN apk add --no-cache openssl-dev musl-dev
RUN cargo install cargo-chef 
WORKDIR /usr/src/rustbot

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /usr/src/rustbot/recipe.json recipe.json
RUN cargo chef cook --release
COPY . .
RUN cargo build -r

FROM alpine:3.19@sha256:c5b1261d6d3e43071626931fc004f70149baeba2c8ec672bd4f27761f8e1ad6b
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /usr/src/rustbot/target/release/rustbot .
CMD ./rustbot
