FROM clux/muslrust:stable AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bins

# Final image
FROM alpine:3.23
WORKDIR /app
ENV PATH=/app:$PATH
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vpn-port-forward-manager /app
CMD ["vpn-qbittorrent-port-forward"]
