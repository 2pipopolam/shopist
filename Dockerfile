FROM rust:1.84.0 as builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    postgresql-client \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-watch

RUN mkdir -p migration/src templates static

COPY migration/Cargo.toml migration/
COPY migration/src/* migration/src/
COPY Cargo.toml Cargo.lock ./
COPY templates templates/
COPY static static/
COPY src src/

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/shopist .
COPY --from=builder /usr/src/app/templates /usr/local/bin/templates
COPY --from=builder /usr/src/app/static /usr/local/bin/static

EXPOSE 3000

CMD ["./shopist"]
