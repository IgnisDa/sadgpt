FROM rust:1.70 as builder
WORKDIR /usr/src/app
RUN cargo install cargo-leptos@0.1.8
COPY . .
RUN cargo leptos build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/server/release/sadgpt /usr/local/bin/sadgpt
COPY --from=builder /usr/src/app/static /opt/static
COPY --from=builder /usr/src/app/target/site/pkg /opt/
COPY --from=builder /usr/src/app/Cargo.toml /opt/Cargo.toml
WORKDIR /opt
ENV RUST_LOG=info
CMD ["sadgpt"]
