FROM rust:1.77-alpine3.19@sha256:59aa190ab64f3c65e4335e2781488ee2f3c9b98636824b546a1b410c9e91123a AS compiler
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
