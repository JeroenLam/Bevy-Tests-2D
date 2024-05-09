# Bevy-Tests-2D
A repository containins some first attempts at making a game in Bevy

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