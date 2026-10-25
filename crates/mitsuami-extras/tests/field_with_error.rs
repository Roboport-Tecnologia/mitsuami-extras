//! `FieldWithError`: a field with its error message under it, shown while
//! there is one. The field carries the error too (`Prop::Error`), which
//! marks it as the platform does and makes assistive technology read it.

use mitsuami::core::Prop;
use mitsuami::prelude::*;
use mitsuami_extras::FieldWithError;
use mitsuami_test::prelude::*;

const MESSAGE: &str = "Remote name is required";

#[mitsuami_test::test]
async fn shows_the_message_under_the_field_while_there_is_one(app: TestApp) {
    let error = signal(Some(MESSAGE.to_owned()));
    app.mount(move || {
        Column::new().padding(16).child(
            FieldWithError::new(TextInput::new().a11y_label("Name").error(error).test_id("name"), error)
                .column(|c| c.width(240)),
        )
    });
    app.expect(by_label("Name")).to_be_visible().await;

    app.expect(by_text(MESSAGE)).to_be_visible().await;
    let field = app.get_by_test_id("name").frame();
    let message = app.get(by_text(MESSAGE)).frame();
    assert!(message.y() >= field.y() + field.height(), "{message:?} under {field:?}");
    assert!(app.get_by_test_id("name").native_state().props.contains(&Prop::Error(Some(MESSAGE.into()))));

    error.set(None);
    app.settle().await;
    app.expect(by_text(MESSAGE)).to_be_hidden().await;
    assert!(app.get_by_test_id("name").native_state().props.contains(&Prop::Error(None)));
}

mitsuami_test::main!();
