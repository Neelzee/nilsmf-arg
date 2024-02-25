# Use a rust base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the local Cargo.toml and Cargo.lock into the container
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this is a separate step to leverage Docker cache)
RUN cargo build --release

# Copy the rest of the source code
COPY src ./src

# Build your application
RUN cargo build --release

# Specify the command to run on container start
CMD ["./target/release/nilsmf_backend"]
