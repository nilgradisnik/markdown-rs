// http://gtk-rs.org

extern crate comrak;
extern crate gio;
extern crate gtk;
#[macro_use]
extern crate horrorshow;
extern crate sourceview;
extern crate webkit2gtk;

mod preview;
mod utils;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;
use gio::MenuExt;

use webkit2gtk::*;

use std::env::args;

use utils::{buffer_to_string, configure_sourceview, open_file, set_title};

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

fn build_system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();

    menu.append("About", "app.about");
    menu.append("Quit", "app.quit");

    application.set_app_menu(&menu);
}

fn add_actions(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    about_dialog: &gtk::AboutDialog,
) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(window => move |_, _| {
        window.destroy();
    }));

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(about_dialog => move |_, _| {
        about_dialog.show();
    }));

    application.add_action(&about);
    application.add_action(&quit);
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("gtk-ui.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Builder couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);

    let header_bar: gtk::HeaderBar = builder.get_object("header_bar").unwrap();
    header_bar.set_title(NAME);

    let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();

    let text_buffer: sourceview::Buffer = builder.get_object("text_buffer").unwrap();
    configure_sourceview(&text_buffer);

    let web_context = WebContext::get_default().unwrap();
    let web_view = WebView::new_with_context(&web_context);

    let markdown_view: gtk::ScrolledWindow = builder.get_object("scrolled_window_right").unwrap();
    markdown_view.add(&web_view);

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

    text_buffer.connect_changed(clone!(web_view => move |buffer| {
        let markdown = buffer_to_string(buffer).unwrap();
        web_view.load_html(&preview::render(&markdown), None);
    }));

    web_view.connect_decide_policy(move |view, decision, _| {
        if view.get_uri().unwrap() != "about:blank" {
            decision.ignore();
        }
        true
    });
    web_view.connect_load_failed(move |_, _, _, _| true);

    open_button.connect_clicked(clone!(header_bar, text_buffer => move |_| {
        file_chooser.show();

        if file_chooser.run() == gtk::ResponseType::Ok.into() {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let contents = open_file(&filename);
            set_title(&header_bar, &filename);
            text_buffer.set_text(&contents);
        }

        file_chooser.hide();
    }));

    about_dialog.connect_delete_event(clone!(about_dialog => move |_, _| {
        about_dialog.hide();
        Inhibit(true)
    }));

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    build_system_menu(application);
    add_actions(application, &window, &about_dialog);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new("com.github.markdown-rs", gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
