use gtk4::prelude::*;

mod apps;
mod launch;
mod window;

fn main() -> glib::ExitCode {
    let app = gtk4::Application::builder()
        .application_id("com.velo.Launcher")
        .build();

    app.connect_activate(|app| {
        window::build_window(app).present();
    });

    app.run()
}
