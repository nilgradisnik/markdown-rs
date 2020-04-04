# Markdown Rust

Fast, simple, distraction free Markdown editor.

![screenshot](./screenshot.png)

## Motivation

There seem to be no simple, fast Markdown editors on Linux. 

> Inspired by [uberwriter](http://uberwriter.wolfvollprecht.de/).

## Build from source

This project depends on [gtk-rs](http://gtk-rs.org/docs-src/requirements.html), [sourceview](https://github.com/gtk-rs/sourceview) and [webkit2gtk](https://github.com/gtk-rs/webkit2gtk-rs).

- Ubuntu

        sudo apt install libgtk-3-dev libgtksourceview-3.0-dev libwebkit2gtk-4.0-dev


- Fedora

        sudo dnf install gtk3-devel glib2-devel gtksourceview3-devel webkitgtk4-devel


Build and run

    cargo run

## Packaging status

Fedora/CentOS [COPR](https://copr.fedorainfracloud.org/coprs/atim/markdown-rs/): `sudo dnf copr enable atim/markdown-rs -y && sudo dnf install markdown-rs`
