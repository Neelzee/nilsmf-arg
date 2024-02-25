# Use a rust base image
FROM rust:latest

COPY . .

# Build dependencies (this is a separate step to leverage Docker cache)
RUN cargo build --release

# Specify the command to run on container start
CMD ["./target/release/nilsmf_backend"]
