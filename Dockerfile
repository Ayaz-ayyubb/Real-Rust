# Use a Rust base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to trick cargo into believing it's a Rust project
RUN mkdir src && \
    echo "fn main() { println!(\"dummy\") }" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Remove the dummy source code
RUN rm -rf src

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Start a new stage to keep the final image lightweight
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the previous stage
COPY --from=builder /app/target/release/main.rs .

# Update the package index and install necessary dependencies
RUN apt-get update && \
    apt-get install -y git

# Expose any ports your application may use
EXPOSE 8080

# Command to run the application
CMD ["./main.rs"]