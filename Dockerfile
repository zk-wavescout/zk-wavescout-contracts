FROM rust:latest

WORKDIR /app

# Install Soroban CLI and WASM target
RUN rustup target add wasm32-unknown-unknown && \
    cargo install soroban-cli --version 21.3.0

# Copy contract code
COPY . .

# Build contract
RUN cargo check && soroban contract build

EXPOSE 8000

CMD ["soroban", "contract", "build"]
