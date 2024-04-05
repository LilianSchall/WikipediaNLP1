FROM rust:1.67 as builder
WORKDIR /usr/src/category_fetcher
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/category_fetcher /usr/local/bin/category_fetcher
CMD ["category_fetcher"]

