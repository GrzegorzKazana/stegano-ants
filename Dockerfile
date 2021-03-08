FROM rust:1.50 as builder

WORKDIR /usr/src/stegano-ants
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/main.rs
RUN cargo fetch

COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner

WORKDIR /usr/src/stegano-ants
COPY --from=builder /usr/local/cargo/bin/stegano-ants /usr/local/bin/stegano-ants

CMD ["stegano-ants"]