FROM rust:1.75-slim

# Install pkg-config and libssl-dev for ureq if needed (though ureq is pure rust mostly)
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy source code
COPY . .

# Build dependencies and code
RUN cargo build --locked

# Default command
CMD ["cargo", "test"]
