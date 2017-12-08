use gtk::*;
use sourceview::*;
use std::path::Path;

pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(filename);
    }
}

pub fn buffer_to_string(buffer: &Buffer) -> Option<String> {
    let (start, end) = buffer.get_bounds();
    buffer.get_text(&start, &end, false)
}

pub fn configure_sourceview(buff: &Buffer) {
    LanguageManager::new()
        .get_language("markdown")
        .map(|markdown| buff.set_language(&markdown));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("classic")
        .map(|theme| buff.set_style_scheme(&theme));
}
