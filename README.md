# powermenu
I wanted to try myself on a powermenu for window managers using rust as a language

## Notable features
- logout
- shutdown
- restart
- optional: lockscreen

## Currently Supported WMs:
- Hyprland
- Qtile(depending on wayland or not u need to adjust the CSS part in main.rs)
- DWM if installed as cahdwm (look in logic.rs for exat ussage)

## Dependencies
- Rust
- cargo

## Installation:
> [!NOTE] Select a theme before running the compile command
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
   sudo cp target/debug/powermenu /usr/bin/
   ```
