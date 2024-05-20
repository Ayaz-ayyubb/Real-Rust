# Use a Rust base image
FROM rustlang/rust:nightly

# Set the working directory
WORKDIR /Real-Rust

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml ./

# Create an empty src directory to trick cargo into believing it's a Rust project
RUN mkdir src && \
    echo "fn main() { println!(\"dummy\") }" > src/main.rs

# Build the dependencies
RUN cargo build --release

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Start a new stage to keep the final image lightweight
FROM debian:buster-slim

# Set the working directory
WORKDIR /Real-Rust

# Copy the built binary from the previous stage
COPY --from=builder /Real-Rust/target/release/main .

# Update the package index and install necessary dependencies
RUN apt-get update && \
    apt-get install -y git

# Expose any ports your application may use
EXPOSE 8080

# Command to run the application
CMD ["./main"]
