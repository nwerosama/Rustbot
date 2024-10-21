FROM scratch AS base
WORKDIR /builder
COPY . .

FROM alpine:3.20@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=base /builder/target/x86_64-unknown-linux-musl/release/rustbot .
CMD [ "./rustbot" ]
