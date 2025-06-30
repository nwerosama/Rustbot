FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:6e644b0d7ac1543ce6368d0cd9f919d6d234c718a721fdabd132f50acb0488b2
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
