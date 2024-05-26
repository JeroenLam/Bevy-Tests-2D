# Bevy-Tests-2D
A repository containins some first attempts at making a game in Bevy.

Based on the following [tutorial](https://www.youtube.com/watch?v=1QI3cHVpqbs&list=PL6uRoaCCw7GMujF_6PtzvkrZBlB_ZKWyZ).

Assets from [here](https://pixelfrog-assets.itch.io/pixel-adventure-1?download).

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