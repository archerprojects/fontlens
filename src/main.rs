// src/main.rs
// FontLens v1.0.0

slint::include_modules!();

use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

// ── Theme Detection ───────────────────────────────────────────────────────────

struct LensTheme {
    accent:       slint::Color,
    panel_bg:     slint::Color,
    hover_bg:     slint::Color,
    chrome_bg:    slint::Color,
    view_bg:      slint::Color,
    border:       slint::Color,
    button_bg:    slint::Color,
    button_hover: slint::Color,
    text:         slint::Color,
    subtext:      slint::Color,
}

impl LensTheme {
    // Lean Linux dark palette — the confirmed default look.
    fn lean_dark() -> Self {
        Self {
            accent:       slint::Color::from_argb_u8(255, 75,  139, 212),
            panel_bg:     slint::Color::from_argb_u8(255, 46,  46,  46),
            hover_bg:     slint::Color::from_argb_u8(102, 75,  139, 212),
            chrome_bg:    slint::Color::from_argb_u8(255, 26,  26,  26),
            view_bg:      slint::Color::from_argb_u8(255, 30,  30,  30),
            border:       slint::Color::from_argb_u8(255, 68,  68,  68),
            button_bg:    slint::Color::from_argb_u8(255, 42,  42,  42),
            button_hover: slint::Color::from_argb_u8(255, 58,  58,  58),
            text:         slint::Color::from_argb_u8(255, 220, 220, 220),
            subtext:      slint::Color::from_argb_u8(255, 160, 160, 160),
        }
    }

    fn generic_dark() -> Self {
        Self {
            accent:       slint::Color::from_argb_u8(255, 75,  139, 212),
            panel_bg:     slint::Color::from_argb_u8(255, 45,  45,  45),
            hover_bg:     slint::Color::from_argb_u8(102, 75,  139, 212),
            chrome_bg:    slint::Color::from_argb_u8(255, 26,  26,  26),
            view_bg:      slint::Color::from_argb_u8(255, 30,  30,  30),
            border:       slint::Color::from_argb_u8(255, 68,  68,  68),
            button_bg:    slint::Color::from_argb_u8(255, 42,  42,  42),
            button_hover: slint::Color::from_argb_u8(255, 58,  58,  58),
            text:         slint::Color::from_argb_u8(255, 220, 220, 220),
            subtext:      slint::Color::from_argb_u8(255, 160, 160, 160),
        }
    }

    fn generic_light() -> Self {
        Self {
            accent:       slint::Color::from_argb_u8(255, 75,  139, 212),
            panel_bg:     slint::Color::from_argb_u8(255, 245, 245, 245),
            hover_bg:     slint::Color::from_argb_u8(64,  75,  139, 212),
            chrome_bg:    slint::Color::from_argb_u8(255, 236, 236, 236),
            view_bg:      slint::Color::from_argb_u8(255, 255, 255, 255),
            border:       slint::Color::from_argb_u8(255, 204, 204, 204),
            button_bg:    slint::Color::from_argb_u8(255, 224, 224, 224),
            button_hover: slint::Color::from_argb_u8(255, 214, 214, 214),
            text:         slint::Color::from_argb_u8(255, 26,  26,  26),
            subtext:      slint::Color::from_argb_u8(255, 106, 106, 106),
        }
    }
}

fn detect_theme() -> LensTheme {
    let de = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    if de.contains("kde") { return read_kde_theme(); }
    if de.contains("gnome") || de.contains("x-cinnamon") || de.contains("xfce") {
        return read_gsettings_theme();
    }
    if de.contains("cosmic") { return LensTheme::generic_dark(); }
    if let Ok(gtk_theme) = std::env::var("GTK_THEME") {
        return map_gtk_theme_name(&gtk_theme);
    }
    LensTheme::generic_dark()
}

fn read_gsettings_theme() -> LensTheme {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output();
    match output {
        Ok(out) => {
            let name = String::from_utf8_lossy(&out.stdout)
                .trim()
                .trim_matches('\'')
                .to_lowercase();
            map_gtk_theme_name(&name)
        }
        Err(_) => LensTheme::generic_dark(),
    }
}

fn read_kde_theme() -> LensTheme {
    if let Some(path) = dirs::home_dir().map(|h| h.join(".config/kdeglobals")) {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if line.starts_with("ColorScheme=") {
                    let scheme = line.trim_start_matches("ColorScheme=").to_lowercase();
                    return if scheme.contains("dark") || scheme.contains("breeze-dark") {
                        LensTheme::generic_dark()
                    } else {
                        LensTheme::generic_light()
                    };
                }
            }
        }
    }
    LensTheme::generic_dark()
}

fn map_gtk_theme_name(name: &str) -> LensTheme {
    let n = name.to_lowercase();
    if n.contains("lean-theme-dark") { return LensTheme::lean_dark(); }
    if n.contains("dark") || n.contains("noir") || n.contains("black") {
        return LensTheme::generic_dark();
    }
    LensTheme::generic_light()
}

// ── Font Metadata ─────────────────────────────────────────────────────────────

fn read_font_metadata(path: &str) -> (String, String) {
    let p = PathBuf::from(path);
    if let Ok(data) = std::fs::read(&p) {
        if let Ok(face) = ttf_parser::Face::parse(&data, 0) {
            let family = face.names()
                .into_iter()
                .find(|n| n.name_id == ttf_parser::name_id::FAMILY && n.is_unicode())
                .and_then(|n| n.to_string())
                .unwrap_or_else(|| {
                    p.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Unknown")
                        .to_string()
                });
            let style = if face.is_bold() && face.is_italic() {
                "Bold Italic"
            } else if face.is_bold() {
                "Bold"
            } else if face.is_italic() {
                "Italic"
            } else {
                "Regular"
            };
            return (family, style.to_string());
        }
    }
    let name = p.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();
    (name, String::new())
}

// ── Font helpers ──────────────────────────────────────────────────────────────

fn is_font(p: &Path) -> bool {
    matches!(
        p.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .as_deref(),
        Some("ttf") | Some("otf")
    )
}

// ── Font Scanner — recursive walk, returns every font beneath `path` ───────────

fn scan_directory(path: &Path) -> Vec<FontEntry> {
    let mut entries: Vec<FontEntry> = Vec::new();
    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if is_font(entry.path()) {
            let path_str = entry.path().to_string_lossy().to_string();
            let (family, style) = read_font_metadata(&path_str);
            entries.push(FontEntry {
                filename: entry.file_name().to_string_lossy().to_string().into(),
                family:   family.into(),
                style:    style.into(),
                path:     path_str.into(),
            });
        }
    }
    entries
}

// ── Browser helpers ───────────────────────────────────────────────────────────

// Count fonts directly inside a directory (one level, cheap — used for badges).
fn shallow_font_count(dir: &Path) -> usize {
    let mut n = 0;
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.filter_map(|e| e.ok()) {
            if is_font(&e.path()) {
                n += 1;
            }
        }
    }
    n
}

// Does this directory contain a font within a bounded depth? Early-exit walk —
// only used when the "folders with fonts only" filter is active.
fn has_fonts_within(dir: &Path, depth: usize) -> bool {
    for e in walkdir::WalkDir::new(dir)
        .max_depth(depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if is_font(e.path()) {
            return true;
        }
    }
    false
}

// List the subdirectories of `dir`, annotated with a shallow font count.
// When `fonts_only` is set, hide directories with no fonts within depth 3.
fn list_dirs(dir: &Path, fonts_only: bool) -> Vec<BrowserEntry> {
    let mut rows: Vec<BrowserEntry> = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.filter_map(|e| e.ok()) {
            let p = e.path();
            if !p.is_dir() {
                continue;
            }
            let count = shallow_font_count(&p);
            if fonts_only && count == 0 && !has_fonts_within(&p, 3) {
                continue;
            }
            rows.push(BrowserEntry {
                name:       e.file_name().to_string_lossy().to_string().into(),
                path:       p.to_string_lossy().to_string().into(),
                font_count: count as i32,
            });
        }
    }
    rows.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    rows
}

// ── Path resolution — expands ~ to home dir ───────────────────────────────────

fn resolve_path(raw: &str) -> PathBuf {
    if raw == "~" {
        if let Some(home) = dirs::home_dir() {
            return home;
        }
    }
    if let Some(rest) = raw.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(raw)
}

// ── Font Loading — register the font with the shared collection so Slint can
//    render a live specimen from an arbitrary file path ─────────────────────────

fn load_font(path: &str) {
    use slint::fontique_07::fontique::Blob;
    use std::sync::Arc;
    let p = Path::new(path);
    if !p.exists() { return; }
    if let Ok(data) = std::fs::read(p) {
        let mut collection = slint::fontique_07::shared_collection();
        let arc: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(data);
        collection.register_fonts(Blob::new(arc), None);
    }
}

// ── Install — copy to ~/.fonts, refresh cache. No sudo. ───────────────────────

fn install_font(font_path: &str) -> bool {
    if let Some(home) = dirs::home_dir() {
        let fonts_dir = home.join(".fonts");
        if !fonts_dir.exists() {
            let _ = fs::create_dir_all(&fonts_dir);
        }
        let src = PathBuf::from(font_path);
        if let Some(fname) = src.file_name() {
            let dest = fonts_dir.join(fname);
            if fs::copy(&src, &dest).is_ok() {
                let _ = Command::new("fc-cache").arg("-f").arg(&fonts_dir).spawn();
                return true;
            }
        }
    }
    false
}

// ── Delete ────────────────────────────────────────────────────────────────────

enum DeleteOutcome {
    Removed,
    Denied,
    Failed,
}

fn delete_font(font_path: &str) -> DeleteOutcome {
    let p = PathBuf::from(font_path);
    match fs::remove_file(&p) {
        Ok(_) => {
            // Refresh the cache for the directory the font lived in.
            if let Some(parent) = p.parent() {
                let _ = Command::new("fc-cache").arg("-f").arg(parent).spawn();
            }
            DeleteOutcome::Removed
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => DeleteOutcome::Denied,
        Err(_) => DeleteOutcome::Failed,
    }
}

// ── Directory scope ───────────────────────────────────────────────────────────
// Protected system font roots are never removable from inside the app.
// Everything else — ~/.fonts, ~/.local/share/fonts, and any external
// collection the user owns — is fair game (the OS still enforces ownership).

fn dir_scope(path: &Path) -> &'static str {
    for root in ["/usr/share/fonts", "/usr/local/share/fonts"] {
        if path.starts_with(root) {
            return "system";
        }
    }
    "removable"
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() -> Result<(), slint::PlatformError> {
    let theme = detect_theme();
    let ui = AppWindow::new()?;

    ui.set_color_accent(theme.accent);
    ui.set_color_panel_bg(theme.panel_bg);
    ui.set_color_hover_bg(theme.hover_bg);
    ui.set_color_chrome_bg(theme.chrome_bg);
    ui.set_color_view_bg(theme.view_bg);
    ui.set_color_border(theme.border);
    ui.set_color_button_bg(theme.button_bg);
    ui.set_color_button_hover(theme.button_hover);
    ui.set_color_text(theme.text);
    ui.set_color_subtext(theme.subtext);

    // ── Browser: list a directory ─────────────────────────────────────────────

    ui.on_browser_open_at({
        let ui_handle = ui.as_weak();
        move |raw| {
            let ui = ui_handle.unwrap();
            let dir = resolve_path(raw.as_str());
            if !dir.is_dir() { return; }
            let fonts_only = ui.get_browser_fonts_only();
            let rows = list_dirs(&dir, fonts_only);
            ui.set_browser_path(dir.to_string_lossy().as_ref().into());
            ui.set_browser_entries(slint::ModelRc::new(slint::VecModel::from(rows)));
        }
    });

    // ── Browser: go up one level ──────────────────────────────────────────────

    ui.on_browser_up({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let current = PathBuf::from(ui.get_browser_path().to_string());
            if let Some(parent) = current.parent() {
                let fonts_only = ui.get_browser_fonts_only();
                let rows = list_dirs(parent, fonts_only);
                ui.set_browser_path(parent.to_string_lossy().as_ref().into());
                ui.set_browser_entries(slint::ModelRc::new(slint::VecModel::from(rows)));
            }
        }
    });

    // ── Browser: scan the chosen directory into the target panel ──────────────

    ui.on_browser_scan({
        let ui_handle = ui.as_weak();
        move |raw| {
            let ui = ui_handle.unwrap();
            let dir = resolve_path(raw.as_str());
            if !dir.is_dir() { return; }
            let scope = dir_scope(&dir);
            let entries = scan_directory(&dir);
            let path_str: slint::SharedString = dir.to_string_lossy().as_ref().into();
            let model = slint::ModelRc::new(slint::VecModel::from(entries));
            if ui.get_browser_target() == "a" {
                ui.set_current_dir_a(scope.into());
                ui.set_status_a(path_str);
                ui.set_fonts_a(model);
            } else {
                ui.set_current_dir_b(scope.into());
                ui.set_status_b(path_str);
                ui.set_fonts_b(model);
            }
        }
    });

    // ── Select font ───────────────────────────────────────────────────────────

    ui.on_select_font_a({
        let ui_handle = ui.as_weak();
        move |path, family, style| {
            let ui = ui_handle.unwrap();
            load_font(path.as_str());
            ui.set_preview_path_a(path);
            ui.set_preview_family_a(family);
            ui.set_preview_style_a(style);
            ui.set_install_status_a("".into());
        }
    });

    ui.on_select_font_b({
        let ui_handle = ui.as_weak();
        move |path, family, style| {
            let ui = ui_handle.unwrap();
            load_font(path.as_str());
            ui.set_preview_path_b(path);
            ui.set_preview_family_b(family);
            ui.set_preview_style_b(style);
            ui.set_install_status_b("".into());
        }
    });

    // ── Install ───────────────────────────────────────────────────────────────

    ui.on_install_font_a({
        let ui_handle = ui.as_weak();
        move |path| {
            let ui = ui_handle.unwrap();
            if install_font(path.as_str()) {
                ui.set_install_status_a("Installed.".into());
            } else {
                ui.set_install_status_a("Install failed.".into());
            }
        }
    });

    ui.on_install_font_b({
        let ui_handle = ui.as_weak();
        move |path| {
            let ui = ui_handle.unwrap();
            if install_font(path.as_str()) {
                ui.set_install_status_b("Installed.".into());
            } else {
                ui.set_install_status_b("Install failed.".into());
            }
        }
    });

    // ── Delete ────────────────────────────────────────────────────────────────

    ui.on_delete_font_a({
        let ui_handle = ui.as_weak();
        move |path| {
            let ui = ui_handle.unwrap();
            match delete_font(path.as_str()) {
                DeleteOutcome::Removed => {
                    ui.set_install_status_a("Removed.".into());
                    ui.set_preview_path_a("".into());
                    ui.set_preview_family_a("".into());
                    ui.set_preview_style_a("".into());
                    let dir = resolve_path(&ui.get_status_a().to_string());
                    if dir.is_dir() {
                        let entries = scan_directory(&dir);
                        ui.set_fonts_a(slint::ModelRc::new(slint::VecModel::from(entries)));
                    }
                }
                DeleteOutcome::Denied => {
                    ui.set_install_status_a("Cannot remove — system path needs root.".into());
                }
                DeleteOutcome::Failed => {
                    ui.set_install_status_a("Remove failed.".into());
                }
            }
        }
    });

    ui.on_delete_font_b({
        let ui_handle = ui.as_weak();
        move |path| {
            let ui = ui_handle.unwrap();
            match delete_font(path.as_str()) {
                DeleteOutcome::Removed => {
                    ui.set_install_status_b("Removed.".into());
                    ui.set_preview_path_b("".into());
                    ui.set_preview_family_b("".into());
                    ui.set_preview_style_b("".into());
                    let dir = resolve_path(&ui.get_status_b().to_string());
                    if dir.is_dir() {
                        let entries = scan_directory(&dir);
                        ui.set_fonts_b(slint::ModelRc::new(slint::VecModel::from(entries)));
                    }
                }
                DeleteOutcome::Denied => {
                    ui.set_install_status_b("Cannot remove — system path needs root.".into());
                }
                DeleteOutcome::Failed => {
                    ui.set_install_status_b("Remove failed.".into());
                }
            }
        }
    });

    // ── Open in file manager ──────────────────────────────────────────────────

    ui.on_open_in_filemanager({
        move |raw_path| {
            let path = resolve_path(raw_path.as_str());
            let target = if path.exists() {
                path.to_string_lossy().to_string()
            } else {
                raw_path.to_string()
            };
            let _ = Command::new("xdg-open").arg(&target).spawn();
        }
    });

    ui.run()
}
