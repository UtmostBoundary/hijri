# hijri

[![Crates.io](https://img.shields.io/crates/v/hijri.svg)](https://crates.io/crates/hijri)
[![Release](https://img.shields.io/github/v/release/UtmostBoundary/hijri)](https://github.com/UtmostBoundary/hijri/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A friendly, `cal`-like command-line tool for Hijri (Islamic) dates. Convert between Gregorian and Hijri, browse month calendars, and list major Islamic events — all from your terminal.

---

## Install

### Homebrew

```
brew install UtmostBoundary/tap/hijri
```

> Note: available once a release is published. Until then, build from source (see below).

### Cargo

Once published to crates.io:

```
cargo install hijri
```

Or install directly from the repository at any time:

```
cargo install --git https://github.com/UtmostBoundary/hijri
```

### GitHub Releases

Prebuilt binaries (`.tar.gz`, `.deb`) will be available on the [Releases page](https://github.com/UtmostBoundary/hijri/releases) once a release is tagged.

### Building from source

```
git clone https://github.com/UtmostBoundary/hijri.git
cd hijri
cargo build --release
# Binary is at target/release/hijri
```

---

## Usage

### Today's date

Running `hijri` with no arguments, or `hijri today`, prints today's Hijri date alongside the Gregorian date.

```
$ hijri
3 Muḥarram 1448 AH  (Thu, 2026-06-18)

$ hijri today
3 Muḥarram 1448 AH  (Thu, 2026-06-18)
```

### Calendar

`hijri cal` follows the same argument rules as the classic `cal` command:

| Command | Shows |
|---------|-------|
| `hijri cal` | the current Hijri month |
| `hijri cal 1448` | the whole Hijri **year** 1448 (12-month grid) |
| `hijri cal 9 1448` | a single month — `<month> <year>` |
| `hijri cal ramadan 1448` | a month by **name** (also `muharram`, `rajab`, `muh`, …) |
| `hijri ramadan` | that month of the current Hijri year (bare shorthand) |
| `hijri 1448` | the whole year (bare shorthand) |

Today's date is highlighted (reverse-video) when writing to a terminal; use `--no-color` (or pipe the output) to disable. In a single month, Islamic event days are marked with a `*` and listed in full below the grid.

```
$ hijri cal 9 1448
    Ramaḍān 1448
Su Mo Tu We Th Fr Sa
    1* 2  3  4  5  6
 7  8  9 10 11 12 13
14 15 16 17 18 19 20
21 22 23 24 25 26 27
28 29
  1 Ramaḍān — Ramadan (start)
```

```
$ hijri cal 1448
                              1448

   Muḥarram 1448           Ṣafar 1448       Rabīʿ al-Awwal 1448
Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa
       1  2  3  4  5            1  2  3  4                  1  2
 6  7  8  9 10 11 12   5  6  7  8  9 10 11   3  4  5  6  7  8  9
...
```

### Date conversion

`hijri convert <YYYY-MM-DD>` converts between Gregorian and Hijri calendars. The direction is auto-detected: a year ≥ 1700 is treated as Gregorian, otherwise as Hijri. You can override this with `--from <gregorian|hijri>` and `--to <gregorian|hijri>`.

**Gregorian → Hijri:**

```
$ hijri convert 2026-06-18
3 Muḥarram 1448 AH  (Thu, 2026-06-18)
```

**Hijri → Gregorian:**

```
$ hijri convert 1447-12-10
10 Dhū al-Ḥijja 1447 AH  (Wed, 2026-05-27)
```

### Events

`hijri events [<hijri-year>]` lists major Islamic events for a Hijri year with their corresponding Gregorian dates. Without an argument it uses the current Hijri year.

```
$ hijri events 1448
Major events — 1448 AH
  Islamic New Year    1 Muḥarram         2026-06-16
  Ashura             10 Muḥarram         2026-06-25
  ...

Note: dates are Umm al-Qura calculated; local religious observance may differ by ±1 day.
```

### Global flags

| Flag | Description |
|------|-------------|
| `--method umm-al-qura` | Calculation method (only supported value; reserved for a future online source). |
| `--json` | Output structured JSON. |
| `--lang <en\|ar>` | Month names in English or Arabic. |
| `--no-color` | Disable colored output (also auto-disabled when not writing to a terminal). |

**Arabic month names:**

```
$ hijri convert 2026-06-18 --lang ar
3 محرم 1448 AH  (Thu, 2026-06-18)
```

**JSON output:**

```
$ hijri convert 2026-06-18 --json
{
  "hijri": {
    "year": 1448,
    "month": 1,
    "day": 3,
    "month_name": "Muḥarram"
  },
  "gregorian": {
    "year": 2026,
    "month": 6,
    "day": 18,
    "weekday": "Thursday"
  },
  "method": "umm-al-qura"
}
```

---

## Accuracy & methodology

There are three fundamentally different things people call "the Hijri calendar," and they do not always agree.

### Observational (real moon sighting)

The Hijri calendar is a lunar calendar whose months traditionally begin when a new crescent moon is sighted by a human observer. This is the *religious* calendar. It is retrospective by nature — the start of a new month is announced at most a day or so in advance, and it is regional: Saudi Arabia, Morocco, Pakistan, Indonesia, and other countries often differ by a day because they use different horizons and criteria. **No offline tool can compute this calendar** — it depends on actual observations that have not happened yet.

### Umm al-Qura

The Umm al-Qura calendar is a *calculated* (not observational) calendar maintained by Saudi Arabia's King Abdulaziz City for Science and Technology (KACST). Months begin based on an astronomical criterion (the moon must set after the sun on the evening of conjunction). KACST publishes the Umm al-Qura calendar years in advance; it is the official civil and administrative calendar of Saudi Arabia and is what most software (and most people outside of a strictly religious context) mean when they say "the Hijri date."

**This tool computes the Umm al-Qura calculated civil date.**

Because Umm al-Qura is a computed approximation of moon sighting, it can be ±1 day off from the date actually announced for religiously-observed events (Ramadan start, Eid al-Fitr, Eid al-Adha) in any given country. No offline tool can predict what moon-sighting authorities will announce. The ±1 day caveat shown in `hijri events` output reflects this.

### Tabular/arithmetic

A third system uses a fixed cyclic formula to approximate the lunar calendar purely mathematically, with no reference to astronomy at all. This tool intentionally does not support the tabular calendar; it uses Umm al-Qura exclusively.

### Dates far from the modern era

The tool always returns a computed value and does not hard-error on dates outside the historical range of the published Umm al-Qura tables. The underlying ICU4X library will extrapolate, but dates far from the modern era (roughly before 1300 AH / 1882 CE, or far into the future) are increasingly approximate. Use them as rough estimates only.

---

## How it works

`hijri` is written in Rust and built on the [ICU4X](https://github.com/unicode-org/icu4x) `icu_calendar` crate, which provides a well-tested implementation of the Umm al-Qura calendar. Date arithmetic, month-length calculations, and conversions are all delegated to ICU4X.

---

## License

MIT — see [LICENSE](LICENSE).
