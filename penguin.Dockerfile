FROM rust:1.88-bullseye

WORKDIR /bot

# Install system dependencies
RUN apt-get update && apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the project in release mode
RUN cargo build --profile release

# Create output directory for binaries
RUN mkdir -p /output

# Copy built binaries to output directory
RUN cp target/release/* /output/ 2>/dev/null || true

# Set the default command to copy binaries to mounted volume
CMD ["sh", "-c", "cp -r /output/* /penguin/ 2>/dev/null || echo 'No binaries to copy' && echo 'Build completed. Binaries copied to /penguin/'"]
