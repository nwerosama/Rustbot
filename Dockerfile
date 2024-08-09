FROM rust:1.80-alpine3.20@sha256:1f5aff501e02c1384ec61bb47f89e3eebf60e287e6ed5d1c598077afc82e83d5 AS chef
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

FROM alpine:3.20@sha256:0a4eaa0eecf5f8c050e5bba433f58c052be7587ee8af3e8b3910ef9ab5fbe9f5
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=builder /builder/target/release/rustbot .
CMD ./rustbot
