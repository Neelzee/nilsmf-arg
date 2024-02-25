# Use a rust base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /var/www/nilsmf/nilsmf_backend

# Copy the local Cargo.toml and Cargo.lock into the container
COPY . /var/www/nilsmf/nilsmf_backend

# Build dependencies (this is a separate step to leverage Docker cache)
RUN cargo build --release

# Copy the certificates
COPY /etc/letsencrypt/live/nilsmf.com /etc/letsencrypt/live/nilsmf.com

# Build your application
RUN cargo install --path .

# Specify the command to run on container start
CMD ["your_binary_name"]
