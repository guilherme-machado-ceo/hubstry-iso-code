# Multi-stage build for optimized production image
FROM rust:1.75-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY examples ./examples
COPY tests ./tests

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/release/hubstry-iso-code /usr/local/bin/hubstry-iso-code

# Copy examples for demonstration
COPY --from=builder /usr/src/app/examples ./examples

# Create directory for user code analysis
RUN mkdir -p /app/workspace && chown appuser:appuser /app/workspace

# Switch to non-root user
USER appuser

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Expose port for potential web interface
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD hubstry-iso-code --version || exit 1

# Default command
CMD ["hubstry-iso-code", "--help"]

# Labels for metadata
LABEL maintainer="Hubstry Deep Tech <contact@hubstry.com>"
LABEL description="ISO-Code Compliance Framework - Rust-based static analysis tool"
LABEL version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/hubstry/iso-code-framework"
LABEL org.opencontainers.image.documentation="https://github.com/hubstry/iso-code-framework/blob/main/README.md"
LABEL org.opencontainers.image.licenses="MIT"