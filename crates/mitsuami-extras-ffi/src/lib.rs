//! mitsuami-extras over a C ABI, in one library with mitsuami's
//! (`mitsuami-ffi`, whose functions it exports too): two copies of mitsuami
//! in one process would be two cores. Bindings load this library in place
//! of mitsuami's. Its nodes are mitsuami's, so mitsuami's functions place,
//! size and destroy them, and hear their events.

use mitsuami::core::{NodeId, Prop, Ui, View};
use mitsuami::prelude::*;
use mitsuami::reactive::{Owner, on_cleanup};
use mitsuami_extras::{FieldWithError, TextAreaWithPlaceholder};
use mitsuami_ffi::MitsuamiStr;
use mitsuami_ffi::extend::{guard, keep, string, ui};

pub use mitsuami_ffi;

/// A text area's value and placeholder, which the hint follows.
struct TextAreaState {
    value: Signal<String>,
    placeholder: Signal<String>,
}

thread_local! {
    static TEXT_AREAS: std::cell::RefCell<std::collections::HashMap<u32, TextAreaState>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// A text area with a placeholder on every platform
/// (`TextAreaWithPlaceholder`): a column holding the platform's text area,
/// which grows to fill it. Returns the column, which is sized and placed
/// as any node; `*area` gets the text area, for its other props (its
/// accessible name, lines, read-only) and its events (`Changed` as the
/// user types). Set its value with `mitsuami_extras_text_area_set_value`,
/// not on the text area, so the hint follows it. 0 on failure.
///
/// # Safety
/// `placeholder` is a string as mitsuami's functions take them; `area` is
/// null or points to a `u32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mitsuami_extras_create_text_area(placeholder: MitsuamiStr, area: *mut u32) -> u32 {
    guard(|| {
        let ui = ui()?;
        let placeholder = unsafe { string(placeholder)? };
        let owner = Owner::new_root();
        let (id, inner, state) = owner.with(|| {
            let value = signal(String::new());
            let hint = signal(placeholder);
            let inner = node_ref();
            let id =
                TextAreaWithPlaceholder::new(TextArea::new().bind(value).node_ref(inner), value, move || hint.get())
                    .build(&ui);
            (id, inner, TextAreaState { value, placeholder: hint })
        });
        let inner = inner.get_untracked().ok_or("the text area wasn't built")?;
        if !area.is_null() {
            unsafe { *area = inner.raw() };
        }
        let raw = id.raw();
        TEXT_AREAS.with(|t| t.borrow_mut().insert(raw, state));
        owner.with(|| on_cleanup(move || _ = TEXT_AREAS.with(|t| t.borrow_mut().remove(&raw))));
        keep(raw, owner);
        Ok(id.raw())
    })
    .unwrap_or(0)
}

fn text_area<T>(id: u32, f: impl FnOnce(&TextAreaState) -> T) -> Result<T, String> {
    TEXT_AREAS.with(|t| t.borrow().get(&id).map(f)).ok_or_else(|| format!("{id} isn't an extras text area"))
}

/// Sets the text area's text, which the user then edits: the hint shows
/// while it's empty. `id` is the column the create function returned.
///
/// # Safety
/// `value` is a string as mitsuami's functions take them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mitsuami_extras_text_area_set_value(id: u32, value: MitsuamiStr) -> bool {
    guard(|| {
        let value = unsafe { string(value)? };
        text_area(id, |t| t.value.set(value))
    })
    .is_some()
}

/// Sets the placeholder. `id` is the column the create function returned.
///
/// # Safety
/// `placeholder` is a string as mitsuami's functions take them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mitsuami_extras_text_area_set_placeholder(id: u32, placeholder: MitsuamiStr) -> bool {
    guard(|| {
        let placeholder = unsafe { string(placeholder)? };
        text_area(id, |t| t.placeholder.set(placeholder))
    })
    .is_some()
}

/// A node the caller made, in one of extras' views.
struct Adopt(u32);

impl View for Adopt {
    fn build(self, _: &Ui) -> NodeId {
        NodeId::from_raw(self.0)
    }
}

/// A field's error, which its message and the field follow.
struct FieldState {
    error: Signal<Option<String>>,
}

thread_local! {
    static FIELDS: std::cell::RefCell<std::collections::HashMap<u32, FieldState>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// A field with its error message under it (`FieldWithError`): a column
/// holding `content` (the field, or a row with it and its buttons) and the
/// message, shown while there's an error. `field` is the text field the
/// error marks (`Prop::Error`): `content` itself, or a node in it. Returns
/// the column, which is sized and placed as any node. Set the error with
/// `mitsuami_extras_field_set_error`. 0 on failure.
#[unsafe(no_mangle)]
pub extern "C" fn mitsuami_extras_create_field_with_error(content: u32, field: u32) -> u32 {
    guard(|| {
        let ui = ui()?;
        let owner = Owner::new_root();
        let (id, error) = owner.with(|| {
            let error = signal(None::<String>);
            let marked = ui.clone();
            mitsuami::reactive::effect(move || {
                marked.set_prop(NodeId::from_raw(field), Prop::Error(error.get()));
            });
            (FieldWithError::new(Adopt(content), error).build(&ui), error)
        });
        let raw = id.raw();
        FIELDS.with(|f| f.borrow_mut().insert(raw, FieldState { error }));
        owner.with(|| on_cleanup(move || _ = FIELDS.with(|f| f.borrow_mut().remove(&raw))));
        keep(raw, owner);
        Ok(raw)
    })
    .unwrap_or(0)
}

/// Sets what's wrong with the field; empty: nothing. `id` is the column
/// the create function returned.
///
/// # Safety
/// `error` is a string as mitsuami's functions take them.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mitsuami_extras_field_set_error(id: u32, error: MitsuamiStr) -> bool {
    guard(|| {
        let error = unsafe { string(error)? };
        FIELDS
            .with(|f| f.borrow().get(&id).map(|f| f.error.set((!error.is_empty()).then_some(error))))
            .ok_or_else(|| format!("{id} isn't an extras field"))
    })
    .is_some()
}
