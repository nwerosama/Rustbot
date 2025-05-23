FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:c0965d07320c79ca2e3a1cc9e303757f6b0055aa0437571523f5eedf78b15690
LABEL org.opencontainers.image.source="https://git.toast-server.net/nwerosama/Rustbot"
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
