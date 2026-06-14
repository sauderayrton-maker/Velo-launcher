PREFIX  ?= /usr/local
BINDIR   = $(DESTDIR)$(PREFIX)/bin
APPDIR   = $(DESTDIR)$(PREFIX)/share/applications
ICONDIR  = $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps

.PHONY: all build install install-bin uninstall clean run

all: build

build:
	cargo build --release

# Just the file-copy step, with no build dependency — used by `install` and
# by uninstall.sh.
install-bin:
	install -Dm755 target/release/velo-launcher              "$(BINDIR)/velo-launcher"
	install -Dm755 uninstall.sh                              "$(BINDIR)/velo-launcher-uninstall"
	install -Dm644 assets/velo-launcher.desktop              "$(APPDIR)/velo-launcher.desktop"
	install -Dm644 assets/velo-launcher.svg                  "$(ICONDIR)/velo-launcher.svg"
	@update-desktop-database "$(APPDIR)" 2>/dev/null || true
	@gtk-update-icon-cache -f -t "$(DESTDIR)$(PREFIX)/share/icons/hicolor" 2>/dev/null || true
	@echo ""
	@echo "  Velo Launcher installed to $(PREFIX)"
	@echo "  Run: velo-launcher"

install: build install-bin

uninstall:
	@PREFIX=$(PREFIX) bash uninstall.sh

clean:
	cargo clean

run:
	cargo run
