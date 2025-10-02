# ---- Build stage ----
FROM rust:1.82 as builder
WORKDIR /usr/src/app

# cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# build actual app
COPY . .
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/http-video-storage-server /usr/local/bin/

EXPOSE 3000
CMD ["http-video-storage-server"]
