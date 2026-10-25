//! Widgets for mitsuami that some platforms have no native control for.
//!
//! Each one is the platform's own control where it has one, and elsewhere
//! built the way that platform's apps build it. `docs/ARCHITECTURE.md` has,
//! per widget, what each platform shows and why.

mod text_area;

pub use text_area::TextAreaWithPlaceholder;
