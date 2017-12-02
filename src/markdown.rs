// https://github.com/kivikakk/comrak

use gtk::prelude::*;
use gtk::TextBuffer;

use comrak::{markdown_to_html, ComrakOptions};

pub fn string_to_html(text: String) -> String {
    let options = ComrakOptions {
        hardbreaks: true,
        ext_table: true,
        ext_strikethrough: true,
        ..ComrakOptions::default()
    };

    markdown_to_html(&text, &options)
}

pub fn buffer_to_html(buffer: TextBuffer) -> String {
    let (start, end) = buffer.get_bounds();
    let text = buffer.get_text(&start, &end, false);

    string_to_html(text.unwrap())
}
