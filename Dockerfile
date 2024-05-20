# Use the official Rust nightly image as the base
FROM rustlang/rust:nightly AS builder

# Set up the working directory
WORKDIR /usr/src/real-rust

# Copy the Cargo.toml and Cargo.lock files to optimize caching
COPY Cargo.toml ./

# Build the dependencies
RUN cargo build --release

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Set up the working directory
WORKDIR /usr/src/real-rust

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/real-rust/target/release/real-rust .

# Expose any ports the app needs
EXPOSE 8080

# Command to run the application
CMD ["./real-rust"]
