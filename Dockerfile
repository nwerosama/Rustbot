FROM rust:1.75-alpine3.18@sha256:9b3aae3c442e2023440a42f4897b888a0fcc78819bffbdf751582ec7467a2eda AS compiler
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache openssl-dev musl-dev 
WORKDIR /usr/src/rustbot
RUN cargo build || true
COPY . .
RUN cargo fetch && cargo build -r

FROM alpine:3.19@sha256:51b67269f354137895d43f3b3d810bfacd3945438e94dc5ac55fdac340352f48
RUN apk add --no-cache openssl-dev libgcc
WORKDIR /rustbot
COPY --from=compiler /usr/src/rustbot/target/release/rustbot .
CMD [ "./rustbot" ]
