#!/usr/bin/env bash
set -euo pipefail

BOLD='\033[1m'
DIM='\033[2m'
BLUE='\033[34m'
GREEN='\033[32m'
RESET='\033[0m'

banner() { echo -e "\n${BOLD}${BLUE}  $*${RESET}\n"; }
ok()     { echo -e "  ${GREEN}✓${RESET}  $*"; }
info()   { echo -e "  ${DIM}→${RESET}  $*"; }

PREFIX="${PREFIX:-/usr/local}"
BINDIR="$PREFIX/bin"
APPDIR="$PREFIX/share/applications"
ICONDIR="$PREFIX/share/icons/hicolor/scalable/apps"

banner "Velo Launcher — Uninstaller"

info "Removing files from $PREFIX..."
sudo rm -f "$BINDIR/velo-launcher" "$BINDIR/velo-launcher-uninstall"
sudo rm -f "$APPDIR/velo-launcher.desktop"
sudo rm -f "$ICONDIR/velo-launcher.svg"
sudo update-desktop-database "$APPDIR" 2>/dev/null || true
sudo gtk-update-icon-cache -f -t "$PREFIX/share/icons/hicolor" 2>/dev/null || true

ok "Velo Launcher removed from $PREFIX"

echo ""
echo -e "${BOLD}  Velo Launcher has been uninstalled.${RESET}"
echo ""
