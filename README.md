# powermenu
I wanted to try myself on a powermenu for window managers using rust as a language

## Notable features
- logout
- shutdown
- restart
- optional: lockscreen
- seamless integration with your gtk4 theme

## Currently Supported WMs:
- Hyprland
- Qtile(depending on wayland or not u need to adjust the CSS part in main.rs)
- DWM if installed as cahdwm (look in logic.rs for exat ussage)

## Dependencies
- Rust
- cargo


## CSS Styling and Themes

The application uses external CSS files for easy theming and customization:

### CSS Structure

```
styles/
├── main.css           # Currently active theme (rename desired theme to this)
├── catppuccin.css     # Catppuccin Mocha theme
├── gruvbox.css        # Gruvbox Material theme
├── nord.css           # Nord theme
├── dracula.css        # Dracula theme
├── onedark.css        # OneDark theme
└── solarized.css      # Solarized Dark theme

```

### Theme Switching

To switch between themes, rename the desired theme file to `main.css`:

1. **Backup current theme** (optional):
   ```bash
   mv styles/main.css styles/main.css.backup
   ```

2. **Choose your desired theme** and rename it:
   ```bash
   # For Catppuccin theme
   cp styles/catppuccin.css styles/main.css
   
   # For Gruvbox theme
   cp styles/gruvbox.css styles/main.css
   
   # For Nord theme
   cp styles/nord.css styles/main.css
   
   # For Dracula theme
   cp styles/dracula.css styles/main.css
   
   # For OneDark theme
   cp styles/onedark.css styles/main.css
   
   # For Solarized theme
   cp styles/solarized.css styles/main.css
   
   # For Caelestia theme
   cp styles/caelestia.css styles/main.css
   ```

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
