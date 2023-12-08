FROM rust:1.74-alpine3.18 AS compiler
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache openssl-dev musl-dev 
WORKDIR /usr/src/rustbot
RUN cargo build || true
COPY . .
RUN cargo fetch && cargo build -r

FROM alpine:3.19
RUN apk add --no-cache openssl-dev libgcc
WORKDIR /rustbot
COPY --from=compiler /usr/src/rustbot/target/release/rustbot .
CMD [ "./rustbot" ]
