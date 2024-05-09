# Bevy-Tests-2D
A repository containins some first attempts at making a game in Bevy.

This repo is based on the following [video series](https://www.youtube.com/watch?v=B6ZFuYYZCSY&list=PL2wAo2qwCxGDp9fzBOTy_kpUTSwM1iWWd&index=1).

Assets from [poly.pizza](https://poly.pizza/bundle/Ultimate-Space-Kit-YWh743lqGX).

## Compiling and run the game (development)
Install rust and run the command:
```bash
cargo run
```

## Compiling the game (Production to OS version)
Remove the dynamic_linking feature from Cargo.toml:
```
bevy = { version = "0.13.0", features = ["dynamic_linking"] }
```
Run the following command:
```bash
cargo build --release
```