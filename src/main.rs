// http://gtk-rs.org

extern crate gio;
extern crate gtk;
extern crate sourceview;
extern crate comrak;
#[macro_use]
extern crate horrorshow;

mod state;
mod preview;
mod utils;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

use utils::{buffer_to_string, set_title};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("gtk-ui.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Builder couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);

    let header_bar: gtk::HeaderBar = builder.get_object("header_bar").unwrap();
    header_bar.set_title(NAME);

    let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();
    let render_button: gtk::ToolButton = builder.get_object("render_button").unwrap();
    let live_button: gtk::ToggleToolButton = builder.get_object("live_button").unwrap();
    let about_button: gtk::ToolButton = builder.get_object("about_button").unwrap();

    let text_view: sourceview::View = builder.get_object("text_view").unwrap();
    let markdown_view: gtk::TextView = builder.get_object("markdown_view").unwrap();

    let file_chooser: gtk::FileChooserDialog = builder.get_object("file_chooser").unwrap();
    file_chooser.add_buttons(&[
        ("Open", gtk::ResponseType::Ok.into()),
        ("Cancel", gtk::ResponseType::Cancel.into()),
    ]);

    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
    about_dialog.set_program_name(NAME);
    about_dialog.set_version(VERSION);
    about_dialog.set_authors(&[AUTHORS]);
    about_dialog.set_comments(DESCRIPTION);

    open_button.connect_clicked(clone!(header_bar, text_view, markdown_view => move |_| {
        file_chooser.show();

        if file_chooser.run() == gtk::ResponseType::Ok.into() {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            set_title(&header_bar, &filename);
            if let Some(parent) = filename.parent() {
                let subtitle: &str = &parent.to_string_lossy();
                header_bar.set_subtitle(subtitle);
            }

            text_view.get_buffer().unwrap().set_text(&contents);
            markdown_view.get_buffer().unwrap().set_text(&preview::render(&contents));
        }

        file_chooser.hide();
    }));

    text_view.connect_key_release_event(clone!(text_view, markdown_view, live_button => move |_, _| {
        if live_button.get_active() {
            let markdown = buffer_to_string(text_view.get_buffer()).unwrap();
            markdown_view.get_buffer().unwrap().set_text(&preview::render(&markdown));
        }
        Inhibit(true)
    }));

    render_button.connect_clicked(clone!(text_view, markdown_view => move |_| {
        let markdown = buffer_to_string(text_view.get_buffer()).unwrap();
        markdown_view.get_buffer().unwrap().set_text(&preview::render(&markdown));
    }));

    about_button.connect_clicked(clone!(about_dialog => move |_| {
        about_dialog.show();
    }));

    about_dialog.connect_delete_event(clone!(about_dialog => move |_, _| {
        about_dialog.hide();
        Inhibit(true)
    }));

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.markdown-rs", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
