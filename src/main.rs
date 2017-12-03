// http://gtk-rs.org

extern crate gio;
extern crate gtk;
extern crate sourceview;
extern crate comrak;

mod markdown;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

use markdown::{string_to_html, buffer_to_html};

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
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);

    let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();
    let render_button: gtk::ToolButton = builder.get_object("render_button").unwrap();
    let live_button: gtk::ToggleToolButton = builder.get_object("live_button").unwrap();
    let about_button: gtk::ToolButton = builder.get_object("about_button").unwrap();

    let text_view: sourceview::View = builder.get_object("text_view").unwrap();
    let markdown_view: gtk::TextView = builder.get_object("markdown_view").unwrap();

    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
    about_dialog.set_program_name(NAME);
    about_dialog.set_version(VERSION);
    about_dialog.set_authors(&[AUTHORS]);
    about_dialog.set_comments(DESCRIPTION);

    open_button.connect_clicked(clone!(window, text_view, markdown_view => move |_| {
        let file_chooser = gtk::FileChooserDialog::new(Some("Open File"), Some(&window), gtk::FileChooserAction::Open);
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into()),
        ]);
        if file_chooser.run() == gtk::ResponseType::Ok.into() {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            text_view.get_buffer().unwrap().set_text(&contents);
            markdown_view.get_buffer().unwrap().set_text(&string_to_html(contents));
        }

        file_chooser.destroy();
    }));

    text_view.connect_key_release_event(clone!(text_view, markdown_view, live_button => move |_, _| {
        if live_button.get_active() {
            let buffer = text_view.get_buffer().unwrap();
            markdown_view.get_buffer().unwrap().set_text(&buffer_to_html(buffer));
        }
        Inhibit(true)
    }));

    render_button.connect_clicked(clone!(text_view, markdown_view => move |_| {
        let buffer = text_view.get_buffer().unwrap();
        markdown_view.get_buffer().unwrap().set_text(&buffer_to_html(buffer));
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
