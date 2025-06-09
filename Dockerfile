FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:d2094076ea97044f0a0d7e8ea7ce025cc4fb9409b3bd5c4749c21728c204d490
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
