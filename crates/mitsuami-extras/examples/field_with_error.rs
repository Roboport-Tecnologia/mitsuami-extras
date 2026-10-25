//! Fields with their error messages: `cargo run -p mitsuami-extras --example field_with_error`.
//!
//! - An "Add remote" form, checked as it's typed, as a Git client checks
//!   one. Type a name with a space, or `origin` (taken): the message shows
//!   under the field, and the field is marked as the platform marks one
//!   (red on GNOME; nothing on macOS, Windows and KDE, whose fields have no
//!   error state). Fix it and the message goes.
//! - The URL must start with `https://` or `git@`; its field is in a row
//!   with a button, and the message goes under the whole row.
//! - Add stays off until both are right, and a field says nothing until
//!   it's been typed in: empty it again and it says it needs one.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mitsuami::prelude::*;
use mitsuami_extras::FieldWithError;

fn name_error(name: &str) -> Option<String> {
    if name.is_empty() {
        Some("A remote needs a name".into())
    } else if name.contains(char::is_whitespace) {
        Some("A remote's name can't have spaces".into())
    } else if name == "origin" {
        Some("There's a remote called origin already".into())
    } else {
        None
    }
}

fn url_error(url: &str) -> Option<String> {
    if url.is_empty() {
        Some("A remote needs a URL".into())
    } else if !url.starts_with("https://") && !url.starts_with("git@") {
        Some("A URL starts with https:// or git@".into())
    } else {
        None
    }
}

fn label(text: &str) -> impl View {
    Text::new(text.to_string()).width(120)
}

fn page() -> impl View {
    let name = signal(String::new());
    let url = signal(String::new());
    // Nothing to say about a field before it's been typed in.
    let (name_typed, url_typed) = (signal(false), signal(false));
    mitsuami::reactive::effect(move || {
        if !name.get().is_empty() {
            name_typed.set(true);
        }
        if !url.get().is_empty() {
            url_typed.set(true);
        }
    });
    let name_err = move || name_typed.get().then(|| name_error(&name.get())).flatten();
    let url_err = move || url_typed.get().then(|| url_error(&url.get())).flatten();
    let added = signal(String::new());

    Column::new().padding(Spacing::Xl).gap(Spacing::Md).children((
        Text::new("Add remote").text_style(TextStyle::Headline),
        Row::new().gap(Spacing::Md).align(Align::Start).children((
            label("Name:"),
            FieldWithError::new(
                TextInput::new().a11y_label("Name").placeholder("origin").bind(name).error(name_err),
                name_err,
            )
            .column(|c| c.grow(1.0)),
        )),
        Row::new().gap(Spacing::Md).align(Align::Start).children((
            label("URL:"),
            FieldWithError::new(
                Row::new().gap(Spacing::Sm).children((
                    TextInput::new()
                        .a11y_label("URL")
                        .placeholder("https://example.com/repo.git")
                        .bind(url)
                        .error(url_err)
                        .grow(1.0),
                    Button::new("Paste Example").on_click(move || {
                        url.set("https://example.com/repo.git".into());
                        url_typed.set(true);
                    }),
                )),
                url_err,
            )
            .column(|c| c.grow(1.0)),
        )),
        Row::new().gap(Spacing::Md).align(Align::Center).children((
            Text::new(move || added.get()).color(Color::SecondaryLabel).grow(1.0),
            Button::new("Add")
                .role(ButtonRole::Default)
                .enabled(move || name_error(&name.get()).is_none() && url_error(&url.get()).is_none())
                .on_click(move || added.set(format!("Added {}", name.get()))),
        )),
    ))
}

fn main() {
    App::new().window("Fields with their errors", WindowSize::FitHeight(480.0), page).run();
}
