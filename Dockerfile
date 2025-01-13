# Базовый образ с Rust
FROM rust:1.75 as builder

WORKDIR /usr/src/app

# Установка необходимых зависимостей
RUN apt-get update && apt-get install -y \
    postgresql-client \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-watch

# Создаем структуру проекта
RUN mkdir -p migration/src
COPY migration/Cargo.toml migration/
COPY migration/src/* migration/src/

# Копируем файлы проекта
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src/

# Собираем проект
RUN cargo build --release

# Финальный образ
FROM debian:bullseye-slim

# Установка runtime зависимостей
RUN apt-get update && apt-get install -y \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Копируем собранный бинарный файл из builder
COPY --from=builder /usr/src/app/target/release/shopist .

EXPOSE 3000

CMD ["./shopist"]
