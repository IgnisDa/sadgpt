FROM ignisda/archlinux:latest

USER root
RUN eget bram209/leptosfmt --to /usr/bin/leptosfmt

USER $USERNAME
RUN rustup toolchain install nightly --allow-downgrade
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

