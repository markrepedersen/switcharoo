FROM rust:1.50.0-slim-buster as dev
WORKDIR /usr/src
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo install --target x86_64-unknown-linux-gnu --path .

FROM debian:buster-slim as prod
COPY --from=dev /usr/local/cargo/bin/switcharoo /usr/local/bin/switcharoo
CMD ["switcharoo"]
