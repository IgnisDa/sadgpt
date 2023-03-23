FROM node:19.1.0 as style-builder
WORKDIR /usr/src/app
COPY . .
RUN yarn install
RUN yarn tailwind --input ./input.css --output ./output.css --minify

FROM rust:latest as app-builder
RUN rustup default nightly && rustup target add wasm32-unknown-unknown
WORKDIR /usr/src/app
RUN cargo install --locked cargo-leptos@0.1.8
COPY . .
COPY --from=style-builder /usr/src/app/output.css ./style/output.css
RUN cargo leptos build --release

FROM debian:buster-slim
ENV LEPTOS_OUTPUT_NAME=sadgpt
COPY --from=app-builder /usr/src/app/target/server/release/sadgpt /usr/local/bin/sadgpt
WORKDIR /opt
COPY --from=app-builder /usr/src/app/target/site ./
COPY --from=app-builder /usr/src/app/Cargo.toml ./Cargo.toml
CMD ["sadgpt"]
