FROM rust:latest as builder
WORKDIR /usr/src/app
RUN cargo install --locked cargo-leptos@0.1.8
COPY . .
RUN cargo leptos build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/server/release/sadgpt /usr/local/bin/sadgpt
WORKDIR /opt
COPY --from=builder /usr/src/app/static ./static
COPY --from=builder /usr/src/app/target/site/pkg ./pkg
COPY --from=builder /usr/src/app/Cargo.toml ./Cargo.toml
CMD ["sadgpt"]
