// https://github.com/Stebalien/horrorshow-rs
use horrorshow::Raw;
use horrorshow::helper::doctype;

// https://github.com/kivikakk/comrak
use comrak::{markdown_to_html, ComrakOptions};

pub fn string_to_html(markdown: &str) -> String {
    let options = ComrakOptions {
        hardbreaks: true,
        ext_table: true,
        ext_strikethrough: true,
        ..ComrakOptions::default()
    };

    markdown_to_html(markdown, &options)
}

pub fn render(markdown: &str) -> String {
    format!(
        "{}",
        html!(
            : doctype::HTML;
            html {
                head {
                    link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/github.min.css") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                    script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                    script {
                        : Raw("hljs.initHighlightingOnLoad()")
                    }
                    style {
                        : "body { width: 90%; margin: 0 auto; }";
                        : "img { max-width: 90% }"
                    }
                }
                body {
                    : Raw(&string_to_html(markdown));
                }
            }
        )
    )
}
