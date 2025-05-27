FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:5519bb4afc78843352ea5afb3ea022c76c93b22f6c762c87cdc2b6885860965b
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
