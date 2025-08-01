# summoner-emoji-bot

> A random emoji generator bot in Rust


[![Rust](https://github.com/ShawonAshraf/random-approver/actions/workflows/build.yml/badge.svg)](https://github.com/ShawonAshraf/random-approver/actions/workflows/build.yml)

I have to approve a lot of pull requests, and I thought it would be fun to have a random emoji generator to use as an
approval message. Then I was suggested to make a discord bot based on it. There are two components to this project:

1. **Emoji Generator**: A Rust program that generates a random emoji from a predefined list and then copies it to the
   clipboard.
2. **Discord Bot**: A Discord bot that listens for a specific command and responds with a random emoji.

## Usage

### Build

```bash
cargo build --release
```

### Run

```bash
# for the emoji generator
./target/release/random-approver
# for the discord bot
./target/release/summoner-emoji-bot bot
```
