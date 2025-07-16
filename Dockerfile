FROM ubuntu:24.10

RUN apt-get update && \
    apt-get install -y --fix-missing clang pkg-config libvips-dev libglib2.0-dev

# Set the working directory inside the container
WORKDIR /app

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"