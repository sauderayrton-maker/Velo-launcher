use gio::prelude::*;
use gtk4::gdk;
use gtk4::prelude::*;

/// Launches a desktop application via its `.desktop` entry, using a Wayland
/// launch context so the compositor gets proper startup notification.
pub fn launch_app(info: &gio::AppInfo, display: &gdk::Display) {
    let context = display.app_launch_context();
    if let Err(err) = info.launch(&[], Some(&context)) {
        eprintln!("velo-launcher: failed to launch {}: {err}", info.name());
    }
}

/// Runs an arbitrary shell command, detached from the launcher.
pub fn run_command(command: &str) {
    let command = command.trim();
    if command.is_empty() {
        return;
    }

    if let Err(err) = std::process::Command::new("sh").arg("-c").arg(command).spawn() {
        eprintln!("velo-launcher: failed to run '{command}': {err}");
    }
}
