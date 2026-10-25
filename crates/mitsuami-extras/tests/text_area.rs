//! `TextAreaWithPlaceholder`: a text area's placeholder on every platform.
//! Qt's and XAML's text areas show their own; on AppKit and GTK, whose
//! text views have none, it's a hint over the first line that clicks pass
//! through, shown while the area is empty.

use mitsuami::core::{Point, Prop};
use mitsuami::prelude::*;
use mitsuami_extras::TextAreaWithPlaceholder;
use mitsuami_test::prelude::*;

const HINT: &str = "Write a message";

fn note(text: Signal<String>) -> impl View {
    Column::new().padding(16).child(
        TextAreaWithPlaceholder::new(TextArea::new().a11y_label("Message").bind(text).test_id("area"), text, HINT)
            .column(|c| c.width(240).height(120)),
    )
}

/// Whether the platform's own text area shows it.
fn native() -> bool {
    platform! {
        macos | gtk => false,
        _ => true,
    }
}

#[mitsuami_test::test]
async fn shows_the_placeholder_while_empty(app: TestApp) {
    let text = signal(String::new());
    app.mount(move || note(text));
    app.expect(by_label("Message")).to_be_visible().await;

    if native() {
        let area = app.get_by_test_id("area").native_state();
        assert!(area.props.contains(&Prop::Placeholder(HINT.into())));
        return;
    }
    app.expect(by_text(HINT)).to_be_visible().await;
    text.set("Fix the build".into());
    app.settle().await;
    app.expect(by_text(HINT)).to_be_hidden().await;
    text.set(String::new());
    app.settle().await;
    app.expect(by_text(HINT)).to_be_visible().await;
}

/// Typing hides it, as the platform's own placeholder goes.
#[mitsuami_test::test]
async fn typing_hides_it(app: TestApp) {
    if native() {
        return;
    }
    let text = signal(String::new());
    app.mount(move || note(text));
    app.expect(by_label("Message")).to_be_visible().await;
    app.get_by_test_id("area").fill("W").await;
    app.expect(by_text(HINT)).to_be_hidden().await;
}

/// A click on the hint reaches the text area under it.
#[mitsuami_test::test]
async fn a_click_on_it_reaches_the_area(app: TestApp) {
    if native() {
        return;
    }
    let text = signal(String::new());
    app.mount(move || note(text));
    app.expect(by_text(HINT)).to_be_visible().await;
    let hint = app.get_by_text(HINT).frame();
    match app.hit(Point::new(hint.x() + hint.width() / 2.0, hint.y() + hint.height() / 2.0)) {
        Err(_) => assert!(!app.is_headless(), "headless hit-tests"),
        Ok(hit) => assert_eq!(hit, Some(app.get_by_test_id("area").id())),
    }
}

/// The hint starts where typed text does: the dark pixels of the hint
/// over an empty area and of the same words typed in it begin at the same
/// place.
#[mitsuami_test::test]
async fn lines_up_with_typed_text(app: TestApp) {
    if native() {
        return;
    }
    let text = signal(String::new());
    app.mount(move || note(text));
    app.expect(by_text(HINT)).to_be_visible().await;

    // Where the text's pixels start inside the area: the first column and
    // row that differ from the area's background.
    let start = async || {
        let image = match app.ui().capture(app.window()).await {
            Err(mitsuami::core::backend::CaptureError::Unsupported) => return None,
            Err(e) => panic!("capture failed: {e:?}"),
            Ok(image) => image,
        };
        let area = app.get_by_test_id("area").frame();
        let scale = image.scale_factor;
        let px = |x: f32, y: f32| {
            let i = ((y * scale) as usize * image.width as usize + (x * scale) as usize) * 4;
            image.rgba[i..i + 3].to_vec()
        };
        // Inside the border, away from the corners.
        let background = px(area.max_x() - 12.0, area.max_y() - 12.0);
        let (mut left, mut top) = (f32::MAX, f32::MAX);
        let mut y = area.y() + 3.0;
        while y < area.y() + 40.0 {
            let mut x = area.x() + 3.0;
            while x < area.x() + 120.0 {
                if px(x, y) != background {
                    left = left.min(x);
                    top = top.min(y);
                }
                x += 0.5;
            }
            y += 0.5;
        }
        Some((left, top))
    };
    let Some(hinted) = start().await else {
        return assert!(app.is_headless(), "only headless may lack capture");
    };
    text.set(HINT.into());
    app.settle().await;
    let typed = start().await.unwrap();
    assert!(
        (hinted.0 - typed.0).abs() <= 1.0 && (hinted.1 - typed.1).abs() <= 1.0,
        "hint starts at {hinted:?}, typed text at {typed:?}"
    );
}

mitsuami_test::main!();
