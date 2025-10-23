use preview::Preview;
use utils::{buffer_to_string, open_file, save_file, set_title};
use webkit2gtk::WebViewExt;

use gio::prelude::*;
use gtk::{
    prelude::*, AboutDialog, Application, ApplicationWindow, Box as GtkBox, Button,
    FileChooserAction, FileChooserDialog, HeaderBar, Orientation, ResponseType, TextBuffer,
    TextView,
};

mod preview;
#[macro_use]
mod utils;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title(NAME);
    window.set_default_size(1000, 700);

    // Main container
    let vbox = GtkBox::new(Orientation::Vertical, 0);

    // Header bar with integrated action buttons
    let header_bar = HeaderBar::new();
    header_bar.set_title(Some(NAME));
    header_bar.set_show_close_button(true);

    let open_button = Button::with_label("Open");
    let save_button = Button::with_label("Save");
    header_bar.pack_start(&open_button);
    header_bar.pack_start(&save_button);

    window.set_titlebar(Some(&header_bar));

    // Editor and preview panes
    let hbox = GtkBox::new(Orientation::Horizontal, 0);

    // Create text buffer and editor
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);

    let editor_view = TextView::with_buffer(&text_buffer);
    editor_view.set_monospace(true);

    let editor_scroll =
        gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    editor_scroll.add(&editor_view);
    editor_scroll.set_hexpand(true);
    editor_scroll.set_vexpand(true);

    // Create web view for preview
    let web_view = webkit2gtk::WebView::new();

    let preview_scroll =
        gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    preview_scroll.add(&web_view);
    preview_scroll.set_hexpand(true);
    preview_scroll.set_vexpand(true);

    hbox.add(&editor_scroll);
    hbox.add(&preview_scroll);
    hbox.set_vexpand(true);

    vbox.add(&hbox);

    window.add(&vbox);

    // File choosers
    let file_open =
        FileChooserDialog::new(Some("Open File"), Some(&window), FileChooserAction::Open);
    file_open.add_button("Open", ResponseType::Ok);
    file_open.add_button("Cancel", ResponseType::Cancel);

    let file_save =
        FileChooserDialog::new(Some("Save File"), Some(&window), FileChooserAction::Save);
    file_save.add_button("Save", ResponseType::Ok);
    file_save.add_button("Cancel", ResponseType::Cancel);

    // About dialog
    let about_dialog = AboutDialog::new();
    about_dialog.set_program_name(NAME);
    about_dialog.set_version(Some(VERSION));
    about_dialog.set_authors(&[AUTHORS]);
    about_dialog.set_comments(Some(DESCRIPTION));
    about_dialog.set_modal(true);
    about_dialog.set_transient_for(Some(&window));

    // Setup preview rendering
    let preview = Preview::new();

    text_buffer.connect_changed(clone!(@strong web_view, preview => move |buffer| {
        let markdown = buffer_to_string(buffer);
        web_view.load_html(&preview.render(&markdown), None);
    }));

    // Define unified actions
    let open_action = gio::SimpleAction::new("open", None);
    {
        let file_open_clone = file_open.clone();
        let header_bar_clone = header_bar.clone();
        let text_buffer_clone = text_buffer.clone();
        let window_clone = window.clone();

        open_action.connect_activate(move |_, _| {
            file_open_clone.set_transient_for(Some(&window_clone));
            if file_open_clone.run() == ResponseType::Ok {
                if let Some(filename) = file_open_clone.filename() {
                    set_title(&header_bar_clone, &filename);
                    let contents = open_file(&filename);
                    text_buffer_clone.set_text(&contents);
                }
            }
            file_open_clone.hide();
        });
    }
    application.add_action(&open_action);

    let save_action = gio::SimpleAction::new("save", None);
    {
        let file_save_clone = file_save.clone();
        let text_buffer_clone = text_buffer.clone();
        let window_clone = window.clone();

        save_action.connect_activate(move |_, _| {
            file_save_clone.set_transient_for(Some(&window_clone));
            if file_save_clone.run() == ResponseType::Ok {
                if let Some(filename) = file_save_clone.filename() {
                    save_file(&filename, &text_buffer_clone);
                }
            }
            file_save_clone.hide();
        });
    }
    application.add_action(&save_action);

    let about_action = gio::SimpleAction::new("about", None);
    {
        let about_dialog_clone = about_dialog.clone();
        about_action.connect_activate(move |_, _| {
            about_dialog_clone.show();
        });
    }
    application.add_action(&about_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    {
        let window_clone = window.clone();
        quit_action.connect_activate(move |_, _| {
            window_clone.close();
        });
    }
    application.add_action(&quit_action);

    // Wire buttons to unified actions
    open_button.set_action_name(Some("app.open"));
    save_button.set_action_name(Some("app.save"));

    // Setup application menu with unified actions
    let file_menu = gio::Menu::new();
    file_menu.append(Some("Open"), Some("app.open"));
    file_menu.append(Some("Save"), Some("app.save"));
    file_menu.append(Some("Quit"), Some("app.quit"));

    let help_menu = gio::Menu::new();
    help_menu.append(Some("About"), Some("app.about"));

    let main_menu = gio::Menu::new();
    main_menu.append_submenu(Some("File"), &file_menu);
    main_menu.append_submenu(Some("Help"), &help_menu);

    application.set_menubar(Some(&main_menu));

    window.show_all();
}

fn main() {
    let application = Application::new(
        Some("com.github.markdown-rs"),
        gio::ApplicationFlags::empty(),
    );

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run();
}
