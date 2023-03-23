FROM ignisda/archlinux:latest

USER root
RUN npm install --global sass tailwindcss

USER $USERNAME
RUN cargo install --locked cargo-leptos cargo-generate cargo-make
RUN rustup toolchain install nightly --allow-downgrade
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

