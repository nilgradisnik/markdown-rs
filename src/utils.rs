use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use gtk::{prelude::*, HeaderBar, TextBuffer};

pub fn set_title(header_bar: &HeaderBar, path: &Path) {
    if let Some(file_name) = path.file_name() {
        let file_name: &str = &file_name.to_string_lossy();
        header_bar.set_title(Some(file_name));

        if let Some(parent) = path.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            header_bar.set_subtitle(Some(subtitle));
        }
    }
}

pub fn buffer_to_string(buffer: &TextBuffer) -> String {
    let (start, end) = buffer.bounds();
    buffer
        .text(&start, &end, false)
        .unwrap_or_default()
        .to_string()
}

pub fn open_file(filename: &Path) -> String {
    let file = File::open(filename).expect("Couldn't open file");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);

    contents
}

pub fn save_file(filename: &Path, text_buffer: &TextBuffer) {
    let contents = buffer_to_string(text_buffer);
    let mut file = File::create(filename).expect("Couldn't save file");
    file.write_all(contents.as_bytes())
        .expect("File save failed");
}

// http://gtk-rs.org/tuto/closures
macro_rules! clone {
    // Match `@strong` token and clone the variable
    (@strong $($n:ident),+ => move || $body:expr) => {
        {
            $(let $n = $n.clone();)+
            move || $body
        }
    };
    (@strong $($n:ident),+ => move |$($p:pat_param),*| $body:expr) => {
        {
            $(let $n = $n.clone();)+
            move |$($p),*| $body
        }
    };
    (@strong $($n:ident),+ => async move { $($body:tt)* }) => {
        {
            $(let $n = $n.clone();)+
            async move { $($body)* }
        }
    };
    // Fallback for other cases
    ($($body:tt)*) => {
        $($body)*
    };
}
