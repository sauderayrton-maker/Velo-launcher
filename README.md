# Velo Launcher

A `rofi`/`wofi`-style app launcher for Hyprland, styled to match **Velo
Player**, **Velo Browser** and **Velo Files** — dark glass panels, the same
`#8ab4d4` accent, and a rounded search bar.

Unlike rofi's centered window, Velo Launcher docks a rounded search pill to
the **bottom-center** of the screen. Start typing and a fuzzy-matched list of
apps (with icons) slides up above the bar.

## Features

- **drun-style app search** — every visible `.desktop` entry on the system,
  fuzzy-matched as you type (icons included).
- **Run arbitrary commands** — the last result is always "Run '<your text>'",
  executed via `sh -c` if you hit Enter on it.
- **Keyboard-driven** — type to filter, <kbd>↑</kbd>/<kbd>↓</kbd> to move the
  selection, <kbd>Enter</kbd> to launch, <kbd>Esc</kbd> (or focus loss) to
  dismiss.
- Anchored to the bottom of the screen via `gtk4-layer-shell`, centered
  horizontally, sized to fit its content.

## Requirements

- GTK4 (4.12+)
- [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell)
- A recent [Rust toolchain](https://rustup.rs) (stable, 2021 edition)

| Component       | Arch package      | Debian/Ubuntu package            | Fedora package         |
|------------------|-------------------|------------------------------------|--------------------------|
| Build tools      | `base-devel`      | `build-essential`, `pkg-config`   | `gcc`, `pkg-config`     |
| GTK4             | `gtk4`            | `libgtk-4-dev`                     | `gtk4-devel`            |
| gtk4-layer-shell | `gtk4-layer-shell`| `libgtk4-layer-shell-dev`          | `gtk4-layer-shell-devel`|

## Installation

### Quick install (recommended)

```bash
git clone https://github.com/sauderayrton-maker/Velo-launcher.git
cd Velo-launcher
./install.sh
```

This detects your package manager, installs the system dependencies above,
builds `velo-launcher` in release mode, and installs it plus a desktop entry
and icon via `make install` (requires `sudo` for the final install step).

### Manual build

```bash
cargo build --release                  # the launcher
sudo make install PREFIX=/usr/local    # install + desktop entry
```

Run it with `velo-launcher`.

To remove everything Velo Launcher installed:

```bash
make uninstall            # or: ./uninstall.sh
```

This prompts for `sudo` itself, removes `velo-launcher`, the desktop entry,
and the icon (refreshing the icon cache). A copy is also installed as
`velo-launcher-uninstall`, so it works even if you've deleted this cloned repo.

### Run without installing

```bash
cargo run --release
```

## Hyprland setup

Bind a key to toggle the launcher, e.g. in `keybindings.conf`:

```ini
bind = $mainMod, D, exec, pkill -x velo-launcher || velo-launcher
```

Velo Launcher sets its layer-shell namespace to `velo-launcher`, so you can
target it with a `layerrule` if you want to tweak Hyprland's blur/animation
for it:

```ini
layerrule {
    name = "velo_launcher"
    match:namespace = ^(velo-launcher)$
    blur = on
}
```

No `windowrule` is needed — the launcher is a layer-shell surface, not a
regular window, and positions itself.
