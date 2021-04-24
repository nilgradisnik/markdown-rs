// http://gtk-rs.org

extern crate comrak;
extern crate gio;
extern crate gtk;
#[macro_use]
extern crate horrorshow;
extern crate sourceview;
extern crate webkit2gtk;

mod preview;
#[macro_use]
mod utils;

use gio::prelude::*;
use gtk::Builder;
use gtk::functions::show_uri_on_window;
use gtk::prelude::*;

use webkit2gtk::*;

use std::env::args;
use std::time::{SystemTime, UNIX_EPOCH};

use preview::Preview;
use utils::{buffer_to_string, configure_sourceview, open_file, save_file, set_title};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn build_system_menu(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    about_dialog: &gtk::AboutDialog,
) {
    let menu = gio::Menu::new();

    menu.append(Some("About"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));

    application.set_app_menu(Some(&menu));

    let quit = gio::SimpleAction::new("quit", None);
    let about = gio::SimpleAction::new("about", None);
    quit.connect_activate(clone!(window => move |_, _| {
        window.close();
    }));
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
    window.set_application(Some(application));

    let header_bar: gtk::HeaderBar = builder.get_object("header_bar").unwrap();
    header_bar.set_title(Some(NAME));

    let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();
    let save_button: gtk::ToolButton = builder.get_object("save_button").unwrap();

    let text_buffer: sourceview::Buffer = builder.get_object("text_buffer").unwrap();
    configure_sourceview(&text_buffer);

    let web_context = WebContext::get_default().unwrap();
    let web_view = WebView::with_context(&web_context);

    let markdown_view: gtk::ScrolledWindow = builder.get_object("scrolled_window_right").unwrap();
    markdown_view.add(&web_view);

    let file_open: gtk::FileChooserDialog = builder.get_object("file_open").unwrap();
    file_open.add_buttons(&[("Open", gtk::ResponseType::Ok.into()), ("Cancel", gtk::ResponseType::Cancel.into())]);

    let file_save: gtk::FileChooserDialog = builder.get_object("file_save").unwrap();
    file_save.add_buttons(&[("Save", gtk::ResponseType::Ok.into()), ("Cancel", gtk::ResponseType::Cancel.into())]);

    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
    about_dialog.set_program_name(NAME);
    about_dialog.set_version(Some(VERSION));
    about_dialog.set_authors(&[AUTHORS]);
    about_dialog.set_comments(Some(DESCRIPTION));

    let preview = Preview::new();
    text_buffer.connect_changed(clone!(web_view, preview => move |buffer| {
        let markdown = buffer_to_string(buffer);
        web_view.load_html(&preview.render(&markdown), None);
    }));

    web_view.connect_decide_policy(clone!(window => move |view, decision, _| {
        let uri = view.get_uri().unwrap();
        if uri != "about:blank" {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            show_uri_on_window(Some(&window), &uri, timestamp.as_secs() as u32).unwrap();
            decision.ignore();
        }
        true
    }));
    web_view.connect_load_failed(move |_, _, _, _| true);

    open_button.connect_clicked(clone!(file_open, header_bar, text_buffer => move |_| {
        file_open.show();
        if file_open.run() == gtk::ResponseType::Ok.into() {
            let filename = file_open.get_filename().expect("Couldn't get filename");

            set_title(&header_bar, &filename);
            text_buffer.set_text(&open_file(&filename));
        }
        file_open.hide();
    }));

    save_button.connect_clicked(clone!(file_save, text_buffer => move |_| {
        file_save.show();
        if file_save.run() == gtk::ResponseType::Ok.into() {
            let filename = file_save.get_filename().expect("Couldn't get filename");

            set_title(&header_bar, &filename);
            save_file(&filename, &text_buffer);
        }
        file_save.hide();
    }));

    about_dialog.connect_delete_event(move |dialog, _| {
        dialog.hide();
        Inhibit(true)
    });

    window.connect_delete_event(move |win, _| {
        win.close();
        Inhibit(false)
    });

    build_system_menu(application, &window, &about_dialog);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.markdown-rs"), gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
