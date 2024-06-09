# bevy-multiplayer

## Setup

### Prerequisites

* Make sure that you have cargo installed: \
https://doc.rust-lang.org/cargo/getting-started/installation.html

```bash
git clone git@github.com:CuddlyBunion341/bevy-multiplayer
cd bevy-multiplayer
```

## Usage

```bash
# start the server
cargo run --bin server

# start client (as many as you want)
cargo run --bin client
```

Optimizations:

```bash
# use release target for better performance
cargo run --release --bin server
cargo run --release --bin client
```
