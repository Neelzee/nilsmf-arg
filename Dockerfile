# Use a rust base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /var/www/nilsmf/nilsmf_backend

COPY . /var/www/nilsmf/nilsmf_backend

# Build dependencies (this is a separate step to leverage Docker cache)
RUN cargo build --release

# Copy the certificates
COPY /etc/letsencrypt/live/nilsmf.com/ .

# Build your application
RUN cargo install --path .

EXPOSE 8080

# Specify the command to run on container start
CMD ["your_binary_name"]
