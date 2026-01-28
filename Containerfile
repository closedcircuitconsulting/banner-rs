#
# Build stage
#

FROM rust:1.91-alpine3.23 AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

COPY src ./src

RUN cargo build --release

#
# Run stage
#

FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=builder /app/target/release/banner-rs /app/banner

EXPOSE 8080

CMD ["/app/banner"]
