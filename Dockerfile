FROM rust:1.88-bullseye

WORKDIR /bot

COPY . .

RUN apt-get update && apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --profile release

CMD ["./target/release/the-guy-bot", "bot"]
