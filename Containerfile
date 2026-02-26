#
# Build stage
#

FROM rust:1.91-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

COPY src ./src

RUN cargo build --release

#
# Run stage
#

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libgcc-s1 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/banner-rs /app/banner

EXPOSE 8080

CMD ["/app/banner"]
