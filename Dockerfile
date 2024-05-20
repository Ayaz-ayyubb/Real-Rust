# Use the official Rust nightly image as the base
FROM rustlang/rust:nightly

# Set up the working directory
WORKDIR /Real-Rust

# Copy the Cargo.toml and Cargo.lock files to optimize caching
COPY Cargo.toml ./

RUN mkdir src && \
    echo "fn main() { println!(\"dummy\") }" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Set up the working directory
WORKDIR /Real-Rust

# Expose any ports the app needs
EXPOSE 8080

# Command to run the application
CMD ["./main"]
