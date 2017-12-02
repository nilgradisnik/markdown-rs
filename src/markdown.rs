// https://github.com/kivikakk/comrak

use comrak::{markdown_to_html, ComrakOptions};

pub fn to_html(text: Option<String>) -> String {
    let options = ComrakOptions {
        hardbreaks: true,
        ext_table: true,
        ext_strikethrough: true,
        ..ComrakOptions::default()
    };

    markdown_to_html(&text.unwrap(), &options)
}
