FROM scratch AS base
WORKDIR /builder
COPY . .

FROM alpine:3.21@sha256:56fa17d2a7e7f168a043a2712e63aed1f8543aeafdcee47c58dcffe38ed51099
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=base /builder/target/x86_64-unknown-linux-musl/release/rustbot .
CMD [ "./rustbot" ]
