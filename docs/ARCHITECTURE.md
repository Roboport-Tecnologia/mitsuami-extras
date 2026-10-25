# Architecture

## 1. What goes here

mitsuami shares a widget only if every platform has a native control for it
(mitsuami's `docs/ARCHITECTURE.md` §1). Apps still need the others: a date
picker, a calendar, an expander, a code view. This repository is where
they're defined, and where we decide what each platform shows.

- **A widget native everywhere belongs in mitsuami,** not here. Check all
  four platforms (AppKit, WinUI 3, GTK 4, Qt Quick and Kirigami) first.
- **Each widget is a mitsuami custom widget** (mitsuami §7.3): one shared
  definition (props, events, semantics, accessibility actions) and one render
  per platform.

## 2. Choosing each platform's render

For each platform, use the first that exists:

1. **Native:** the platform's own control (`native::<W>()`).
2. **Ad hoc:** built from the platform's widgets the way its own apps build
   it, e.g. GNOME Software's and Discover's star ratings (`ad_hoc::<W>()`).
   Name the apps it follows.
3. **Composed** from mitsuami's widgets, so it still uses native controls.
4. **Drawn,** with mitsuami's `Canvas` in semantic colours, only for what
   native controls can't build.

Native always wins, as in mitsuami: what a platform shows follows that
platform, not the other platforms' look. Each widget's section below says,
per platform, which render it uses and why.

## 3. Testing

As in mitsuami: no unit tests, integration tests in
`crates/mitsuami-extras/tests` written with `mitsuami-test`, run headless and
natively (`MITSUAMI_NATIVE=1`). Tests assert what every platform does, and say
where platforms differ.

## 4. C ABI

`crates/mitsuami-extras-ffi` exports this repository's widgets over a C ABI,
in one library with mitsuami's (`mitsuami-ffi`, whose functions it exports
too): two copies of mitsuami in one process would be two cores, so the two
ABIs can't be separate libraries. Language bindings (C# first, in
`mitsuamia`) load this library in place of mitsuami's. An extras widget is
built with mitsuami's builder API in a scope `mitsuami-ffi` disposes with
the node (`mitsuami_ffi::extend`); its nodes are mitsuami's, so mitsuami's
functions size, place and destroy them and hear their events. Until
mitsuami's next release, both build against the sibling checkout
(`../mitsuami`); `mitsuami-ffi` isn't published at all.

## 5. Widgets

### 5.1 TextAreaWithPlaceholder

A text area with a placeholder on every platform, as SourceGit's commit
message has one.

| Platform | Render | Why |
|---|---|---|
| WinUI | native: `TextBox.PlaceholderText` | XAML's text box has one. |
| Kirigami | native: `QQC2.TextArea.placeholderText` | Qt's text area has one. |
| AppKit | composed: a hint over the first line | `NSTextView` has no placeholder in its public API (a private `placeholderString`, not used); AppKit apps draw the hint over the text view. |
| GTK | composed: a hint over the first line | `GtkTextView` has none; GTK apps lay a label over it in an overlay, with `can-target` off. |

- **The hint** is a `Text` in `Color::Placeholder` at the text area's
  `text_area_insets`, in a container clicks pass through
  (`click_through`), hidden while the area has text. It wraps inside the
  area's width.
- **The area is the app's** `TextArea`, bound to the signal the widget
  takes, so the hint follows it; it grows to fill the column the widget
  is, which the app sizes.
- **C ABI:** `mitsuami_extras_create_text_area` returns the column and the
  text area; the value and the placeholder are set through extras'
  functions (so the hint follows them), the rest on the text area with
  mitsuami's.
- **Example:** `cargo run -p mitsuami-extras --example text_area`: an empty
  commit message whose placeholder the app can change and clear back to, a
  note that starts with text, and a read-only area.
- **Where it has run:** headless and AppKit (`tests/text_area.rs`: the hint
  shows and goes, a click on it reaches the area, and it starts where typed
  text does, from captures; `mitsuami-extras-ffi`'s `tests/ffi.rs`). A click
  on the hint reaching the text ran by hand in the example on AppKit (the
  maintainer, 2026-10-25). GTK,
  Kirigami and WinUI only type-check: a native run must confirm GTK's hint
  lines up and passes clicks, and Qt's and XAML's placeholders show.
