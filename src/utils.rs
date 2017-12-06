use gtk::*;
use std::path::Path;
use gtk::TextBuffer;

pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(filename);
    }
}

pub fn buffer_to_string(buffer: Option<TextBuffer>) -> Option<String> {
    match buffer {
      Some(buffer) => {
        let (start, end) = buffer.get_bounds();
        buffer.get_text(&start, &end, false)
      },
      None => panic!("Error getting string from buffer")
    }
}
