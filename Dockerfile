FROM clux/muslrust:stable AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cp "/app/target/$(uname -m)-unknown-linux-musl/release/vpn-port-forward-manager" "/app/vpn-port-forward-manager"
RUN ls -alh /app

# Final image
FROM alpine:3.23
WORKDIR /app
ENV PATH=/app:$PATH
COPY --from=builder /app/vpn-port-forward-manager /app
CMD ["vpn-port-forward-manager"]
