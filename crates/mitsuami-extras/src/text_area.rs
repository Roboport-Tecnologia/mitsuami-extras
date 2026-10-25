//! A text area with a placeholder on every platform: the platform's own
//! where its text area has one (Qt's `TextArea`, XAML's `TextBox`), and
//! elsewhere a hint laid over the first line in the placeholder colour,
//! which clicks pass through, as AppKit and GTK apps add one (macOS's text
//! view has none in its public API, GTK's text view none at all).

use mitsuami::core::{NodeId, Ui, View};
use mitsuami::prelude::*;

/// A [`TextArea`] with a placeholder it shows while it's empty, on every
/// platform. The area is the app's, bound to `value`: the hint follows it.
///
/// ```ignore
/// let message = signal(String::new());
/// TextAreaWithPlaceholder::new(TextArea::new().bind(message), message, "Commit message")
/// ```
///
/// It's a column holding the area, which grows to fill it, so its size is
/// set on it as on any container.
pub struct TextAreaWithPlaceholder {
    area: TextArea,
    value: Signal<String>,
    placeholder: Value<String>,
    column: Container,
}

impl TextAreaWithPlaceholder {
    pub fn new(area: TextArea, value: Signal<String>, placeholder: impl IntoValue<String>) -> TextAreaWithPlaceholder {
        TextAreaWithPlaceholder { area, value, placeholder: placeholder.into_value(), column: Column::new() }
    }

    /// The column it's in: its size, its place in its parent.
    pub fn column(mut self, f: impl FnOnce(Container) -> Container) -> TextAreaWithPlaceholder {
        self.column = f(self.column);
        self
    }
}

/// Whether the platform's text area shows a placeholder itself.
fn native() -> bool {
    platform! {
        macos | gtk => false,
        _ => true,
    }
}

impl View for TextAreaWithPlaceholder {
    fn build(self, ui: &Ui) -> NodeId {
        let TextAreaWithPlaceholder { area, value, placeholder, column } = self;
        if native() {
            return column.child(area.placeholder(placeholder).grow(1.0)).build(ui);
        }
        // The hint starts where the area's first line does, and leaves its
        // trailing edge alone, so a long one wraps inside it.
        let insets = ui.metrics().text_area_insets;
        let hint = Column::new()
            .absolute()
            .top(insets.top)
            .start(insets.left)
            .end(insets.right)
            .click_through(true)
            .hidden(move || !value.get().is_empty())
            .child(Text::new(placeholder).color(Color::Placeholder));
        column.children((area.grow(1.0), hint)).build(ui)
    }
}
