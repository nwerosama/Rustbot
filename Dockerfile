FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:d3e52901b28d7a9bdd1fa47ec2a14e3ad5008c18cd1fb4d6961ce8ad90a7ec3c
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
