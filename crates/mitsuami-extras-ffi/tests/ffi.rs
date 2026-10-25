//! mitsuami-extras' C ABI: its widgets are mitsuami nodes, built in the
//! same `Ui` as mitsuami's own C ABI drives, so mitsuami's functions size
//! them and hear their events.

use std::cell::RefCell;
use std::ffi::c_void;

use mitsuami::core::{NodeId, Prop, Ui, View};
use mitsuami::prelude::*;
use mitsuami_extras_ffi::*;
use mitsuami_ffi::*;
use mitsuami_test::prelude::*;

thread_local! {
    static HEARD: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

extern "C" fn record(_user: *mut c_void, _node: u32, event: *const MitsuamiEvent) {
    let event = unsafe { &*event };
    if event.kind == MitsuamiEventKind::Changed as u32 {
        let text = unsafe { std::slice::from_raw_parts(event.text, event.text_len) };
        HEARD.with(|h| h.borrow_mut().push(format!("changed {}", std::str::from_utf8(text).unwrap())));
    }
}

/// A node made through the C ABI, mounted as a view.
struct Adopt(u32);

impl View for Adopt {
    fn build(self, _: &Ui) -> NodeId {
        NodeId::from_raw(self.0)
    }
}

fn str(s: &str) -> MitsuamiStr {
    MitsuamiStr { ptr: s.as_ptr(), len: s.len() }
}

/// Whether the platform's own text area shows the placeholder.
fn native() -> bool {
    platform! {
        macos | gtk => false,
        _ => true,
    }
}

#[mitsuami_test::test]
async fn a_text_area_shows_its_placeholder_while_empty(app: TestApp) {
    attach(app.ui());
    let mut area = 0;
    let column = unsafe { mitsuami_extras_create_text_area(str("Commit message"), &mut area) };
    assert_ne!(column, 0);
    assert_ne!(area, 0);
    assert!(mitsuami_set_size(
        column,
        MitsuamiLength { unit: 1, value: 240.0 },
        MitsuamiLength { unit: 1, value: 120.0 }
    ));
    set_label(area, "Message");
    assert!(mitsuami_on_event(area, record, std::ptr::null_mut()));
    app.mount(move || Adopt(column));
    app.expect(by_label("Message")).to_be_visible().await;

    let hint = |text: &str| {
        if native() {
            assert!(app.ui().props(NodeId::from_raw(area)).contains(&Prop::Placeholder(text.into())));
        }
    };
    hint("Commit message");
    if !native() {
        app.expect(by_text("Commit message")).to_be_visible().await;
    }

    // The user's typing is the text area's, and hides the hint.
    app.get_by_label("Message").fill("Fix").await;
    assert_eq!(HEARD.with(|h| h.borrow().last().cloned()), Some("changed Fix".into()));
    if !native() {
        app.expect(by_text("Commit message")).to_be_hidden().await;
    }

    // The app's value: emptied, the hint is back.
    assert!(unsafe { mitsuami_extras_text_area_set_value(column, str("")) });
    app.settle().await;
    assert!(app.ui().props(NodeId::from_raw(area)).contains(&Prop::Value(String::new())));
    if !native() {
        app.expect(by_text("Commit message")).to_be_visible().await;
    }

    assert!(unsafe { mitsuami_extras_text_area_set_placeholder(column, str("Describe it")) });
    app.settle().await;
    hint("Describe it");
    if !native() {
        app.expect(by_text("Describe it")).to_be_visible().await;
    }

    // Destroyed with mitsuami's function, it's gone for extras' too.
    assert!(mitsuami_destroy(column));
    assert!(!unsafe { mitsuami_extras_text_area_set_value(column, str("x")) });
}

fn set_label(id: u32, text: &str) {
    assert!(unsafe { mitsuami_set_text(id, MitsuamiTextProp::A11yLabel as u32, text.as_ptr(), text.len()) });
}

mitsuami_test::main!();
