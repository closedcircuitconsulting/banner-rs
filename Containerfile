# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Closed Circuit Consulting

FROM docker.io/library/rust:1.100.0-slim-trixie AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --release --locked --target "$(rustc --print host-tuple)" && \
    cp target/*/release/banner-rs /banner && \
    strip /banner

FROM scratch
LABEL org.opencontainers.image.title="banner"
LABEL org.opencontainers.image.description="Serves the Closed Circuit Consulting banner over HTTP"
LABEL org.opencontainers.image.vendor="Closed Circuit Consulting"
LABEL org.opencontainers.image.authors="unitexe"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://git.closedcircuitconsulting.com/banner"
LABEL org.opencontainers.image.url="https://git.closedcircuitconsulting.com"
LABEL maintainer="unitexe"
COPY --from=builder /banner /usr/local/bin/banner
USER 65534:65534
EXPOSE 8080
CMD ["/usr/local/bin/banner"]
