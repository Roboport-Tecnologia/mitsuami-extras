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

Each repository exports its own widgets over a C ABI: mitsuami its core and
built-in widgets, this one its widgets. Language bindings (C# first, in
`mitsuamia`) build on both.

## 5. Widgets

None yet.
