# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-06-20

### Added
- Single-month calendars now mark Islamic event days with a `*` in the gap
  after the day (e.g. `10*11`), so the marker is width-preserving and the
  columns stay aligned. The full event list still appears below the grid.

### Changed
- In the full-year view, each month cell now shows just the month name (the
  year is already in the page header), matching `cal`. This also fixes a layout
  bug where long names such as "Jumādā al-Ākhira 1448" overflowed the column
  and broke alignment.

## [0.2.0] - 2026-06-20

### Added
- `hijri cal <year>` shows the whole Hijri year as a 12-month grid (3 across),
  matching the argument rules of the classic `cal` command.
- Month names accepted wherever a month is expected, e.g.
  `hijri cal ramadan 1448` (also abbreviations like `muh`, and numbered forms
  like `rabi2`), via unambiguous-prefix matching.
- Bare shortcuts: `hijri 1448` for a whole year and `hijri ramadan` for that
  month of the current Hijri year.
- `--no-color` flag; color is also auto-disabled when output is not a terminal.

### Changed
- Today is now highlighted with width-preserving ANSI reverse-video instead of
  `[3]` brackets, fixing the column misalignment the brackets caused.
- `hijri cal` argument semantics now follow `cal`: one number is a year (year
  grid), two numbers are `<month> <year>`. `hijri cal 9 1448` still works.

## [0.1.0] - 2026-06-20

Initial release.

### Added
- `hijri` / `hijri today` — today's Hijri (Umm al-Qura) date.
- `hijri cal [<month> <year>]` — Hijri month grid with today highlighted and a
  legend of the month's Islamic events.
- `hijri convert <YYYY-MM-DD>` — bidirectional Gregorian↔Hijri conversion with
  auto-detected direction and `--from`/`--to` overrides.
- `hijri events [<year>]` — major Islamic events for a Hijri year with their
  Gregorian dates and an observance caveat.
- `--json` output and `--lang <en|ar>` for English transliteration or Arabic
  script month names.
- Built on the ICU4X (`icu_calendar`) Umm al-Qura calendar.
- Distribution via a Homebrew tap and cargo-dist GitHub releases (prebuilt
  binaries, shell installer, `.tar.xz`/`.deb` artifacts).

[Unreleased]: https://github.com/UtmostBoundary/hijri/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/UtmostBoundary/hijri/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/UtmostBoundary/hijri/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/UtmostBoundary/hijri/releases/tag/v0.1.0
