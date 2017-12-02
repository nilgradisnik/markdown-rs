// http://gtk-rs.org

extern crate gio;
extern crate gtk;
extern crate comrak;

mod markdown;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

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

    let open_button: gtk::ToolButton = builder.get_object("open_button").expect("Couldn't get builder");
    let render_button: gtk::ToolButton = builder.get_object("render_button").expect("Couldn't get builder");

    let text_view: gtk::TextView = builder.get_object("text_view").expect("Couldn't get text_view");
    let markdown_view: gtk::TextView = builder.get_object("markdown_view").expect("Couldn't get text_view");

    open_button.connect_clicked(clone!(window, text_view => move |_| {
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

            text_view.get_buffer().expect("Couldn't get window").set_text(&contents);
        }

        file_chooser.destroy();
    }));

    render_button.connect_clicked(clone!(text_view => move |_| {
        let buffer = text_view.get_buffer().expect("Error getting text");
        let (start, end) = buffer.get_bounds();
        let text = buffer.get_text(&start, &end, false);

        let html = markdown::to_html(text);
        markdown_view.get_buffer().expect("Couldn't get window").set_text(&html);
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
