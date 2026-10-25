//! Text area with a placeholder: `cargo run -p mitsuami-extras --example text_area`.
//!
//! - A commit message: empty, so the placeholder shows; typing hides it,
//!   Clear brings it back. On Qt and WinUI it's the text area's own; on
//!   AppKit and GTK a hint over the first line, which a click goes through
//!   to the text (try clicking on it).
//! - The placeholder follows the app: edit it in the field under the area.
//! - A note that starts with text: its placeholder shows only once it's
//!   emptied.
//! - A read-only area: the placeholder shows while it's empty, and the
//!   text can't be edited until the switch says so.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mitsuami::prelude::*;
use mitsuami_extras::TextAreaWithPlaceholder;

fn heading(text: &str) -> impl View {
    Text::new(text.to_string()).text_style(TextStyle::Headline)
}

fn commit_message() -> impl View {
    let message = signal(String::new());
    let placeholder =
        signal(String::from("Enter commit message. Separate the subject from the description with an empty line."));
    let count = move || {
        let text = message.get();
        format!("{} lines, {} characters", text.lines().count(), text.chars().count())
    };
    Column::new().gap(Spacing::Md).children((
        heading("Commit message"),
        TextAreaWithPlaceholder::new(TextArea::new().a11y_label("Commit message").bind(message), message, placeholder)
            .column(|c| c.height(120)),
        Row::new().gap(Spacing::Md).align(Align::Center).children((
            Text::new(count).color(Color::SecondaryLabel).grow(1.0),
            Button::new("Clear")
                .enabled(move || !message.get().is_empty())
                .on_click(move || message.set(String::new())),
        )),
        Row::new().gap(Spacing::Md).align(Align::Center).children((
            Text::new("Placeholder:"),
            TextInput::new().a11y_label("Placeholder").bind(placeholder).grow(1.0),
        )),
    ))
}

fn note() -> impl View {
    let note = signal(String::from("Buy milk\nCall the bank"));
    Column::new().gap(Spacing::Md).children((
        heading("Note"),
        TextAreaWithPlaceholder::new(TextArea::new().a11y_label("Note").bind(note), note, "Nothing to remember")
            .column(|c| c.height(80)),
    ))
}

fn read_only() -> impl View {
    let text = signal(String::new());
    let editable = signal(false);
    Column::new().gap(Spacing::Md).children((
        heading("Read only"),
        TextAreaWithPlaceholder::new(
            TextArea::new().a11y_label("Read only").bind(text).read_only(move || !editable.get()),
            text,
            "Nothing here yet",
        )
        .column(|c| c.height(80)),
        // A switch's label is beside it, as the platforms lay them out.
        Row::new()
            .gap(Spacing::Md)
            .align(Align::Center)
            .children((Text::new("Editable"), Switch::new("Editable").bind(editable))),
    ))
}

fn page() -> impl View {
    Column::new().padding(Spacing::Xl).gap(Spacing::Xl).children((commit_message(), note(), read_only()))
}

fn main() {
    App::new().window("Text area with a placeholder", WindowSize::FitHeight(480.0), page).run();
}
