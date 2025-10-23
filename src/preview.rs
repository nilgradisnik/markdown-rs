extern crate comrak;
extern crate horrorshow;

use self::comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use self::horrorshow::helper::doctype;
use self::horrorshow::{html, Raw};

const GITHUB_CSS: &str = include_str!("github-markdown.css");
const HIGHLIGHT_CSS: &str = include_str!("highlight.css");

#[derive(Clone, Debug)]
pub struct Preview<'a> {
    comrak_options: ComrakOptions<'a>,
}

impl<'a> Preview<'a> {
    pub fn new() -> Self {
        let mut comrak_render_options = ComrakRenderOptions::default();
        comrak_render_options.hardbreaks = true;

        let mut comrak_extension_options = ComrakExtensionOptions::default();
        comrak_extension_options.table = true;
        comrak_extension_options.strikethrough = true;

        let comrak_options = ComrakOptions {
            render: comrak_render_options,
            extension: comrak_extension_options,
            ..ComrakOptions::default()
        };

        Preview { comrak_options }
    }

    pub fn render(&self, markdown: &str) -> String {
        format!(
            "{}",
            html! {
                : doctype::HTML;
                html {
                    head {
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                        script {
                            : Raw("hljs.initHighlightingOnLoad()")
                        }
                        style {
                            : GITHUB_CSS;
                            : HIGHLIGHT_CSS;
                            : "body { width: 90%; margin: 0 auto; } img { max-width: 90% }";
                        }
                    }
                    body {
                        article(class="markdown-body") {
                            : Raw(markdown_to_html(markdown, &self.comrak_options));
                        }
                    }
                }
            }
        )
    }
}
