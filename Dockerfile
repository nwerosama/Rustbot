FROM rust:1.77-alpine3.19@sha256:b6ea81b37c9ad8b2c875f64a963b5e418e86d33d41eed529671e86e86d30c884 AS compiler
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache openssl-dev musl-dev 
WORKDIR /usr/src/rustbot
RUN cargo build || true
COPY . .
RUN cargo fetch && cargo build -r

FROM alpine:3.19@sha256:c5b1261d6d3e43071626931fc004f70149baeba2c8ec672bd4f27761f8e1ad6b
RUN apk add --no-cache openssl-dev libgcc
WORKDIR /rustbot
COPY --from=compiler /usr/src/rustbot/target/release/rustbot .
CMD [ "./rustbot" ]
