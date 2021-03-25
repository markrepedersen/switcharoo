FROM rust:1.50.0-slim-buster as dev
WORKDIR /usr/src
COPY ./Cargo.* ./
RUN cargo install --target x86_64-unknown-linux-musl --path .
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as prod
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/switcharoo /usr/local/bin/switcharoo
CMD ["switcharoo"]
