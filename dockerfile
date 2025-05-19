# Stage 1: Build environment
FROM rust:1.73-slim-bookworm AS builder

# Install SQLite for both APT and Pacman-based systems
RUN set -eux; \
    if command -v apt-get > /dev/null; then \
        apt-get update && \
        apt-get install -y libsqlite3-dev && \
        rm -rf /var/lib/apt/lists/*; \
    elif command -v pacman > /dev/null; then \
        pacman -Syu --noconfirm sqlite && \
        pacman -Scc --noconfirm; \
    else \
        echo "Unsupported package manager"; exit 1; \
    fi

WORKDIR /app
COPY . .

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies for both package managers
RUN set -eux; \
    if command -v apt-get > /dev/null; then \
        apt-get update && \
        apt-get install -y libsqlite3-0 ca-certificates && \
        rm -rf /var/lib/apt/lists/*; \
    elif command -v pacman > /dev/null; then \
        pacman -Syu --noconfirm sqlite && \
        pacman -Scc --noconfirm; \
    else \
        echo "Unsupported package manager"; exit 1; \
    fi

WORKDIR /app
COPY --from=builder /app/target/release/rspi-api .
COPY .env .

EXPOSE 8080
CMD ["./api"]
