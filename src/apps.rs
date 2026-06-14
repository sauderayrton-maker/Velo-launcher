use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gio::prelude::*;

/// A single launchable application, taken from the desktop's `.desktop`
/// entries (via `gio::AppInfo`).
#[derive(Clone)]
pub struct AppEntry {
    pub info: gio::AppInfo,
    pub name: String,
}

/// Loads every visible application known to the desktop (XDG `.desktop`
/// entries under `/usr/share/applications`, `~/.local/share/applications`,
/// Flatpak/Snap exports, etc.), sorted alphabetically.
pub fn load_apps() -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = gio::AppInfo::all()
        .into_iter()
        .filter(|info| info.should_show())
        .map(|info| {
            let name = info.name().to_string();
            AppEntry { info, name }
        })
        .collect();

    apps.sort_by_key(|entry| entry.name.to_lowercase());
    apps
}

/// Fuzzy-filters `apps` against `query`, returning matches ordered from best
/// to worst.
pub fn filter_apps<'a>(apps: &'a [AppEntry], query: &str) -> Vec<&'a AppEntry> {
    let matcher = SkimMatcherV2::default();

    let mut scored: Vec<(i64, &AppEntry)> = apps
        .iter()
        .filter_map(|entry| matcher.fuzzy_match(&entry.name, query).map(|score| (score, entry)))
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, entry)| entry).collect()
}
