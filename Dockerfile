FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:7ca06cad29fe509ea1b4a0fb40485ca410fe5fdbea39888c5f3169b4961b2b14
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
