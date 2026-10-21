# mitsuami-extras

Widgets for mitsuami that some platforms have no native control for. The
main repository (`../rust-ui-toolkit`, published as `mitsuami`) only shares
widgets every platform has natively; these are the rest.

- `docs/ARCHITECTURE.md` says what belongs here and how each platform's
  render is chosen (native, ad hoc, composed, drawn), with a section per
  widget saying what each platform shows and why.
- Each widget is a mitsuami custom widget (mitsuami's `docs/ARCHITECTURE.md`
  §7.3). Follow mitsuami's `CLAUDE.md` for conventions, testing, and
  type-checking the other platforms from macOS.
- Depends on the published `mitsuami`. If a widget needs something mitsuami
  lacks, add it there first.
