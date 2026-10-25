//! A field with its error message under it. No platform has a control
//! for it: GTK marks the entry red, the others' fields have no error state
//! (mitsuami's `Prop::Error` gives each what it has, and assistive
//! technology the message), and every platform's apps put the message by
//! the field themselves, a line of text under it in the error colour, as
//! GNOME's and macOS's forms do. So it's composed: the field, then the
//! message, shown while there is one.

use mitsuami::core::{NodeId, TextStyle, Ui, View};
use mitsuami::prelude::*;

/// A field (or a row with one, a path field and its button) with the
/// message of what's wrong with it under it, shown while `error` has one.
/// Give the field the same `error` (`TextInput::error`), which marks it
/// as the platform does.
///
/// ```ignore
/// let error = signal(None::<String>);
/// FieldWithError::new(TextInput::new().bind(name).error(error), error)
/// ```
///
/// It's a column holding the field, then the message, so its size and
/// place are set on it as on any container.
pub struct FieldWithError<V> {
    field: V,
    error: Value<Option<String>>,
    column: Container,
}

impl<V: View> FieldWithError<V> {
    pub fn new(field: V, error: impl IntoValue<Option<String>>) -> FieldWithError<V> {
        FieldWithError { field, error: error.into_value(), column: Column::new() }
    }

    /// The column it's in: its size, its place in its parent.
    pub fn column(mut self, f: impl FnOnce(Container) -> Container) -> FieldWithError<V> {
        self.column = f(self.column);
        self
    }
}

impl<V: View> View for FieldWithError<V> {
    fn build(self, ui: &Ui) -> NodeId {
        let FieldWithError { field, error, column } = self;
        let shown = error.clone();
        let message = Text::new(move || error.get().unwrap_or_default())
            .text_style(TextStyle::Caption)
            .color(Color::Error)
            .hidden(move || shown.get().is_none());
        column.align(Align::Stretch).gap(2).children((field, message)).build(ui)
    }
}
