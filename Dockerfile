FROM scratch AS base
WORKDIR /builder
COPY . .

FROM adelielinux/adelie:1.0-beta6@sha256:7126a96c19be064a487ba7176baa58200a26ce8fda02b851b4a0bef760b1f469
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=base /builder/target/x86_64-unknown-linux-musl/release/rustbot .
CMD [ "./rustbot" ]
