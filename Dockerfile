# Get the latest rust builder image
FROM rust:1.78.0-buster AS builder

# Set the working directory
WORKDIR /usr/src/rocks

# Copy the Rust application
COPY src ./src
COPY Cargo.toml .
COPY .cargo/config.docker.toml ./.cargo/config.toml
COPY .env .

# Build the Rust application
RUN cargo install --path .

# Create a lightweight image for the Rust application
FROM debian:bullseye-slim

# Add any dependencies needed to run the Rust application
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*

# Copy the build over from the builder image
COPY --from=builder /usr/local/cargo/bin/rocks /usr/local/bin/rocks

# Run the Rust application
CMD ["rocks"]