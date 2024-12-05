FROM scratch AS base
WORKDIR /builder
COPY . .

FROM alpine:3.21@sha256:e323a465c03a31ad04374fc7239144d0fd4e2b92da6e3e0655580476d3a84621
LABEL org.opencontainers.image.source="https://git.toast-server.net/toast/Rustbot"
RUN apk add --no-cache libgcc
WORKDIR /rustbot
COPY --from=base /builder/target/x86_64-unknown-linux-musl/release/rustbot .
CMD [ "./rustbot" ]
