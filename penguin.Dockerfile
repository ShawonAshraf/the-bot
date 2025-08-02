FROM rust:1.88-bullseye

WORKDIR /bot

# Install system dependencies
RUN apt-get update && apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev && rm -rf /var/lib/apt/lists/*

# Note: Source code should be mounted as a volume at /bot
# Build will happen at runtime when source is available

# Set the default command to build and copy binaries to mounted volume
CMD ["sh", "-c", "cargo build --profile release && cp target/release/* /penguin/ 2>/dev/null || echo 'No binaries to copy' && echo 'Build completed. Binaries copied to /penguin/'"]
