FROM scratch AS base
WORKDIR /builder
COPY . .

FROM alpine:3.21@sha256:eb37f58646a901dc7727cf448cae36daaefaba79de33b5058dab79aa4c04aefb
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=base /builder/target/x86_64-unknown-linux-musl/release/rustbot .
CMD [ "./rustbot" ]
