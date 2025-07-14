FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:122c41e04f907f4d962e0a2750d6b09c540a81dd45aace62a899d268e507b1f6
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
