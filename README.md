# bevy-multiplayer

This is a simple example of how multiplayer can be achieved in [bevy](https://bevyengine.org/) using [renet](https://github.com/lucaspoffo/renet).

## Inspiration

* https://github.com/lucaspoffo/renet/tree/master/demo_bevy
* https://github.com/lucaspoffo/renet/blob/master/renet/examples/echo.rs

## Demo

https://github.com/CuddlyBunion341/bevy-multiplayer/assets/53896675/a33706ad-8e44-46af-aa05-b0d046d8509e

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
