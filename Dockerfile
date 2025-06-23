FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:a921b6864609280975d8269df323c2f4f478cdc9d0f5479e70c3fb5e710d1b11
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
