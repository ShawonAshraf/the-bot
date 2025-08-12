# the-guy-bot

> A random emoji generator bot in Rust


[![Test](https://github.com/ShawonAshraf/summoner-emoji-bot/actions/workflows/test.yml/badge.svg)](https://github.com/ShawonAshraf/summoner-emoji-bot/actions/workflows/test.yml)

I have to approve a lot of pull requests, and I thought it would be fun to have a random emoji generator to use as an
approval message. Then I was suggested to make a discord bot based on it. There are two parts to this project:

1. **Emoji Generator CLI**: A CLI program that generates a random set of emojis as a string (currently hardcoded to 5
   emojis) from a predefined list and then copies it to the
   clipboard.
2. **Discord Bot**: A Discord bot that listens for specific commands and then responds with a random set of emojis
   or messages.

## Usage

> [!NOTE]
> Make sure to create a discord bot application on the discord developer portal with the following permission and scope
> first!:
> `bot` and `send messages`. Also enable `GUILD_MESSAGES` permissions in the previeleged access section so that the bot
> can read
> instructions.


Build the project locally (check [Local Build](#local-build)) and then run:

```bash
chmod +x target/release/the-guy-bot

# for emoji generation cli
./the-guy-bot emoji

# the file_dir should contain at least one file in fortune format
# check https://github.com/umpire274/rFortune?tab=readme-ov-file#-file-format

# for discord bot
export DISCORD_TOKEN=your_token_here
./the-guy-bot bot file_dir

# for guysay (a fortune powered cowsay)
./the-guy-bot guysay file_dir
```

### Local Build

```bash
cargo build --release
```

> [!NOTE]
> In case you don't have access to a machine running linux and would like to run the bot on some linux server, use the
> docker image for building the project for linux.

```bash
# make a directory at the project root named penguin to store the build
mkdir -p penguin
# Build the image
docker build -f penguin.Dockerfile -t guybot-linux-builder .

# Run the container with volume mount to save binaries locally
docker run -v $(pwd):/bot -v $(pwd)/penguin:/penguin guybot-linux-builder
```

### Testing

```bash
cargo test
```

### Docker container

To run the bot as a docker container:

```bash
docker build -t the-guy-bot:latest .

docker run -e DISCORD_TOKEN=your_token_here the-guy-bot:latest
```
