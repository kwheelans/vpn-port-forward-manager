FROM clux/muslrust:stable AS base
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM base AS base-amd64
ENV DOCKER_TARGET_ARCH=x86_64

FROM base AS base-arm64
ENV DOCKER_TARGET_ARCH=aarch64

FROM base-${TARGETARCH} AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base-${TARGETARCH} AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target ${DOCKER_TARGET_ARCH}-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target ${DOCKER_TARGET_ARCH}-unknown-linux-musl
RUN cp "/app/target/${DOCKER_TARGET_ARCH}-unknown-linux-musl/release/vpn-port-forward-manager" "/app/vpn-port-forward-manager"

# Final image
FROM alpine:3.23
WORKDIR /app
ENV PATH=/app:$PATH
RUN apk add --no-cache tzdata
COPY --from=builder /app/vpn-port-forward-manager /app
CMD ["vpn-port-forward-manager"]
