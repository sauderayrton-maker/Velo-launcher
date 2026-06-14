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

## Build & run

```sh
cargo build --release
./target/release/velo-launcher
```

## Install

```sh
./install.sh
```

This installs the binary, `.desktop` entry and icon under `/usr/local`. Run
`./uninstall.sh` (or `velo-launcher-uninstall`) to remove it again.

### Dependencies

- GTK4 (4.12+)
- [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell)

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
