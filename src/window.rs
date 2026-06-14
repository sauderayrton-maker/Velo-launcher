use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gio::prelude::*;
use gtk4::prelude::*;
use gtk4::{gdk, glib};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use crate::apps::{self, AppEntry};
use crate::launch;

/// A row in the results list: either a matched application, or a fallback
/// "run this text as a shell command" entry.
enum ResultKind {
    App(gio::AppInfo),
    Command(String),
}

const MAX_APP_RESULTS: usize = 7;
const SEARCH_WIDTH: i32 = 520;

pub fn build_window(app: &gtk4::Application) -> gtk4::ApplicationWindow {
    load_css();

    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Velo Launcher")
        .decorated(false)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_namespace(Some("velo-launcher"));
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    for edge in [Edge::Left, Edge::Right, Edge::Top] {
        window.set_anchor(edge, false);
    }
    window.set_anchor(Edge::Bottom, true);
    window.set_margin(Edge::Bottom, 64);

    let apps = Rc::new(apps::load_apps());
    let results: Rc<RefCell<Vec<ResultKind>>> = Rc::new(RefCell::new(Vec::new()));

    let results_list = gtk4::ListBox::builder()
        .css_classes(vec!["results-list"])
        .selection_mode(gtk4::SelectionMode::Browse)
        .build();

    let results_panel = gtk4::ScrolledWindow::builder()
        .css_classes(vec!["results-panel"])
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_width(SEARCH_WIDTH)
        .max_content_height(420)
        .propagate_natural_height(true)
        .child(&results_list)
        .visible(false)
        .build();

    let entry = gtk4::Entry::builder()
        .css_classes(vec!["search-bar"])
        .placeholder_text("Search apps or type a command…")
        .width_request(SEARCH_WIDTH)
        .build();
    entry.set_icon_from_icon_name(gtk4::EntryIconPosition::Primary, Some("system-search-symbolic"));

    let root = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .css_classes(vec!["launcher-root"])
        .build();
    root.append(&results_panel);
    root.append(&entry);

    window.set_child(Some(&root));

    // ── Live fuzzy filtering as the user types ──
    entry.connect_changed(glib::clone!(
        #[strong]
        apps,
        #[strong]
        results,
        #[weak]
        results_list,
        #[weak]
        results_panel,
        move |entry| {
            update_results(&entry.text(), &apps, &results, &results_list, &results_panel);
        }
    ));

    // ── Up/Down to move the selection, Escape to dismiss ──
    let key_controller = gtk4::EventControllerKey::new();
    key_controller.connect_key_pressed(glib::clone!(
        #[weak]
        window,
        #[weak]
        results_list,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_, key, _, _| match key {
            gdk::Key::Escape => {
                window.close();
                glib::Propagation::Stop
            }
            gdk::Key::Down => {
                select_relative(&results_list, 1);
                glib::Propagation::Stop
            }
            gdk::Key::Up => {
                select_relative(&results_list, -1);
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        }
    ));
    entry.add_controller(key_controller);

    // ── Enter launches the selected result (or runs the typed text) ──
    entry.connect_activate(glib::clone!(
        #[strong]
        results,
        #[weak]
        results_list,
        #[weak]
        window,
        move |entry| {
            let display = entry.display();
            let index = results_list.selected_row().map_or(0, |row| row.index());

            if let Some(kind) = results.borrow().get(index.max(0) as usize) {
                activate(kind, &display);
                window.close();
            } else if !entry.text().trim().is_empty() {
                launch::run_command(&entry.text());
                window.close();
            }
        }
    ));

    // ── Clicking a result launches it ──
    results_list.connect_row_activated(glib::clone!(
        #[strong]
        results,
        #[weak]
        window,
        #[weak]
        entry,
        move |_, row| {
            let display = entry.display();
            if let Some(kind) = results.borrow().get(row.index() as usize) {
                activate(kind, &display);
                window.close();
            }
        }
    ));

    // ── Dismiss when the launcher loses focus ──
    let became_active = Rc::new(Cell::new(false));
    window.connect_notify_local(
        Some("is-active"),
        glib::clone!(
            #[strong]
            became_active,
            move |window, _| {
                if window.is_active() {
                    became_active.set(true);
                } else if became_active.get() {
                    window.close();
                }
            }
        ),
    );

    window
}

fn activate(kind: &ResultKind, display: &gdk::Display) {
    match kind {
        ResultKind::App(info) => launch::launch_app(info, display),
        ResultKind::Command(cmd) => launch::run_command(cmd),
    }
}

/// Re-runs the fuzzy search for `query`, rebuilds the results list, and
/// shows/hides the results panel accordingly.
fn update_results(
    query: &str,
    apps: &[AppEntry],
    results: &Rc<RefCell<Vec<ResultKind>>>,
    results_list: &gtk4::ListBox,
    results_panel: &gtk4::ScrolledWindow,
) {
    while let Some(child) = results_list.first_child() {
        results_list.remove(&child);
    }

    let query = query.trim();
    let mut new_results = Vec::new();

    if !query.is_empty() {
        for entry in apps::filter_apps(apps, query).into_iter().take(MAX_APP_RESULTS) {
            new_results.push(ResultKind::App(entry.info.clone()));
        }
        new_results.push(ResultKind::Command(query.to_string()));
    }

    for kind in &new_results {
        results_list.append(&build_row(kind));
    }

    if let Some(first) = results_list.row_at_index(0) {
        results_list.select_row(Some(&first));
    }

    results_panel.set_visible(!new_results.is_empty());
    *results.borrow_mut() = new_results;
}

/// Moves the `ListBox` selection by `delta` rows, clamping at the ends.
fn select_relative(list: &gtk4::ListBox, delta: i32) {
    let Some(current) = list.selected_row() else {
        if let Some(row) = list.row_at_index(0) {
            list.select_row(Some(&row));
        }
        return;
    };

    if let Some(row) = list.row_at_index((current.index() + delta).max(0)) {
        list.select_row(Some(&row));
    }
}

/// Builds a single result row: an icon plus a title (and, for apps, a
/// description subtitle).
fn build_row(kind: &ResultKind) -> gtk4::ListBoxRow {
    let icon = gtk4::Image::new();
    icon.set_pixel_size(32);
    icon.add_css_class("result-icon");

    let title = gtk4::Label::new(None);
    title.add_css_class("result-title");
    title.set_halign(gtk4::Align::Start);
    title.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    let text_box = gtk4::Box::builder().orientation(gtk4::Orientation::Vertical).spacing(1).build();
    text_box.set_valign(gtk4::Align::Center);
    text_box.append(&title);

    match kind {
        ResultKind::App(info) => {
            match info.icon() {
                Some(gicon) => icon.set_from_gicon(&gicon),
                None => icon.set_icon_name(Some("application-x-executable")),
            }
            title.set_label(&info.name());

            if let Some(description) = info.description() {
                let subtitle = gtk4::Label::new(Some(&description));
                subtitle.add_css_class("result-subtitle");
                subtitle.set_halign(gtk4::Align::Start);
                subtitle.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                text_box.append(&subtitle);
            }
        }
        ResultKind::Command(cmd) => {
            icon.set_icon_name(Some("utilities-terminal-symbolic"));
            title.set_label(cmd);

            let subtitle = gtk4::Label::new(Some("Run command"));
            subtitle.add_css_class("result-subtitle");
            subtitle.set_halign(gtk4::Align::Start);
            text_box.append(&subtitle);
        }
    }

    let row_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    row_box.add_css_class("result-row");
    row_box.append(&icon);
    row_box.append(&text_box);

    let row = gtk4::ListBoxRow::new();
    row.set_child(Some(&row_box));
    row
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(include_str!("style.css"));
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}
