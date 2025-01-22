# powermenu
I wanted to try myself on a powermenu for window managers using rust as a language

## Currently Supported WMs:
- Hyprland
- Qtile(depending on wayland or not u need to adjust the CSS part in main.rs)

## Dependencies
- Rust
- cargo

## Installation:
1. clone the repo and go in the directory
   ```bash
   git clone https://github.com/KyokoSpl/powermenu
   cd powermenu/
   ```
2. build using cargo
   ```bash
   cargo build
   ```
3. Copy the combiled binary to /usr/bin or whatever bin directory u want (this is optional so you just can type `powermenu` to run the programm otherwise u have to go in the cloned directory and use `cargo run`
   ```bash
   cp target/debug/powermenu /usr/bin/
   ```
