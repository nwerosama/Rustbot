FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:1a39198fcde68348c49a3fd78a54ced553af8168252c6222451f3fe943a4f7ec
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
