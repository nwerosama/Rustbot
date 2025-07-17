FROM scratch AS base
WORKDIR /builder
COPY . .

FROM archlinux:base@sha256:e5d672031f7479b0ce222486f1b5c8b07c931327d16050ad6078a8cd68cf870f
LABEL org.opencontainers.image.source="https://github.com/nwerosama/Rustbot"
ENV RUST_LOG=debug
RUN pacman -Syu --noconfirm && \
  rm -rf /var/cache/pacman/pkg/** && \
  rm -rf /usr/share/{man,doc,info}
WORKDIR /rustbot
COPY --from=base /builder/target/release/rustbot .
CMD [ "./rustbot" ]
