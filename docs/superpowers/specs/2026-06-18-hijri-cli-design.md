# `hijri` — a friendly Hijri calendar CLI

**Status:** Design / spec
**Date:** 2026-06-18
**Repo:** `github.com/UtmostBoundary/hijri`

## 1. Purpose & positioning

A modern, `cal`-like command-line tool for Hijri (Islamic) dates, installable from
Homebrew (and GitHub release artifacts, with apt as a fast-follow). Users should be
able to type `hijri`, `hijri cal`, or `hijri convert 2026-06-18` and get an accurate,
well-formatted answer.

It improves on the existing `itools`/`idate` tools on:

- **Ergonomics** — clean subcommands and ISO `YYYY-MM-DD` input instead of `yyyymmdd`.
- **Accuracy** — Umm al-Qura (the official Saudi calculated calendar) as the single,
  trustworthy conversion method.
- **Scriptability** — first-class `--json` output for piping into other tools.
- **Event awareness** — major Islamic events highlighted in the calendar grid.
- **Honesty** — a clear, documented stance on what the dates mean and their limits
  (see §4, Accuracy & methodology).

## 2. Accuracy & methodology (read this first)

There are effectively **three different "Hijri calendars,"** and conflating them is the
most common source of confusion. This tool is deliberate about which one it implements.

1. **Observational (real moon sighting).** The religious calendar: a month begins when
   the crescent moon is *actually sighted*. It is **retrospective** (announced ~1 day
   ahead), **regional** (Saudi Arabia, Morocco, Pakistan, Indonesia may differ for the
   same lunation), and therefore **impossible for any offline tool to compute**.

2. **Umm al-Qura.** A **calculated** calendar (not a record of sightings). Saudi Arabia's
   KACST computes it from an astronomical rule and publishes it years in advance. It is
   the **official civil calendar** of Saudi Arabia — used for government, printed
   calendars, and most software — so it is what most people mean by "the Hijri date." It
   is still an approximation of real sighting and can be **±1 day** off the announced
   religious date.

3. **Tabular/arithmetic.** A pure 30-year cyclic formula with no astronomy. Works for any
   date but is the least faithful to reality. **Intentionally not supported by this tool.**

**What this tool does:** it computes **Umm al-Qura** (via ICU4X's embedded tables). This
is the correct, reproducible, offline answer for a `cal`-like utility, and it matches the
printed Saudi calendar and most apps.

**The honesty rule, baked into the product:** the tool reports the *calculated civil*
date. For religiously-observed events (Ramadan start, the two Eids, Ashura), the **actual
announced date in your country may differ by ±1 day**, because that depends on local moon
sighting, which no offline tool can predict. This is surfaced in `--help`, the README, and
a one-line note on `hijri events` output.

A future, explicitly out-of-scope option (`--source online`) could fetch announced dates
from a published feed; that would be a fundamentally different, network-dependent mode.

## 3. Architecture

A single Rust binary, internally split into focused, independently testable modules:

- **`engine`** — wraps ICU4X (`icu_calendar`, `IslamicUmmalQura`). Converts
  Gregorian↔Hijri. Pure functions, no I/O. The single place all date math lives.
- **`events`** — a small embedded table of fixed-Hijri-date events. Because these events
  are fixed in the *Hijri* calendar, highlighting them in a Hijri grid is exactly correct.
- **`render`** — formats human output: the `cal`-style month grid, the single-date line,
  and the events list. Owns color, Arabic script, and transliteration concerns.
- **`format`** — output mode: human (default) vs `--json` (a stable, documented schema).
- **`cli`** — `clap`-based argument parsing, subcommand dispatch, global flags.

**Engine choice:** built on **ICU4X**, maintained by the Unicode org, rather than a
hand-rolled implementation or a smaller unmaintained crate. Correctness of dates is the
entire job of this tool, so it stands on Unicode's tested implementation. Trade-off:
a few MB of dependencies / binary size, still shipping as a clean static binary.

## 4. Command surface (v1)

```
hijri                      # today's Hijri date (human line)
hijri today                # alias of the above
hijri cal                  # current Hijri month grid, today highlighted, events marked
hijri cal <month> <year>   # arbitrary Hijri month, e.g. hijri cal 9 1447
hijri convert 2026-06-18   # Gregorian -> Hijri
hijri convert 1447-12-10   # Hijri -> Gregorian (auto-detected; --from/--to to force)
hijri events [<year>]      # list major events for the (Hijri) year, with Gregorian dates
```

**Global flags:**

- `--method <umm-al-qura>` — default and currently the only value. Reserved for a future
  online/observational source. (Tabular is intentionally excluded.)
- `--json` — machine-readable output.
- `--lang <en|ar>` — `en` = transliteration (Muharram, Safar …), `ar` = Arabic script.
- `--no-color` — disable ANSI color (also auto-disabled when stdout is not a TTY).
- `--help`, `--version`.

## 5. Conversion & accuracy rules

- **Umm al-Qura only.** No fallback method.
- **Out-of-range handling:** dates outside Umm al-Qura's supported range (~1924–2077 CE /
  the corresponding Hijri years) produce a **clear error on stderr** stating the supported
  range, and a **non-zero exit code**. There is no silent approximation.
- **`convert` direction auto-detection:** a year ≥ 1700 is treated as Gregorian; otherwise
  Hijri. `--from <gregorian|hijri>` / `--to <gregorian|hijri>` override the guess.
- **Input format:** ISO `YYYY-MM-DD`.

## 6. Output

**Human (default):**

```
$ hijri
10 Dhū al-Ḥijja 1447 AH  (Thu, 2026-06-18)

$ hijri cal
        Dhū al-Ḥijja 1447
 Su Mo Tu We Th Fr Sa
              1  2  3  4
  5  6  7  8  9 10* 11      * = Eid al-Adha
 12 13 14 15 16 17 18
 ...
```

**JSON (`--json`):** each command emits a stable schema, e.g.:

```json
{
  "hijri":     { "year": 1447, "month": 12, "day": 10, "month_name": "Dhū al-Ḥijja" },
  "gregorian": { "year": 2026, "month": 6,  "day": 18, "weekday": "Thursday" },
  "method": "umm-al-qura",
  "events": [ { "name": "Eid al-Adha", "hijri": "1447-12-10" } ]
}
```

## 7. Events (v1)

A small embedded table of major, fixed-Hijri-date events, marked in `cal` output and
listed by `hijri events`:

| Event           | Hijri date          |
|-----------------|---------------------|
| Islamic New Year| 1 Muharram          |
| Ashura          | 10 Muharram         |
| Mawlid          | 12 Rabī' al-Awwal   |
| Ramadan (start) | 1 Ramadan           |
| Eid al-Fitr     | 1 Shawwal           |
| Eid al-Adha     | 10 Dhū al-Ḥijja     |

`hijri events` output carries a one-line note: *"Dates are Umm al-Qura calculated; local
religious observance may differ by ±1 day."*

## 8. Error handling

- Invalid dates, out-of-range input, and unparseable arguments produce a clear message on
  **stderr** and a **non-zero exit code** (good Unix citizen).
- Under `--json`, errors are emitted as a JSON error object (still non-zero exit).
- Informational notes (e.g. the events caveat) go to **stderr** so stdout stays clean for
  piping.

## 9. Testing

- **`engine` unit tests** against known reference conversions: published Umm al-Qura dates,
  the calendar epoch, and the supported-range boundaries (including the out-of-range error).
- **`events` tests:** fixed Hijri dates map to the expected Gregorian dates in a sample year.
- **CLI integration tests** (`assert_cmd`): each subcommand's stdout, exit codes, `--json`
  shape, and `--lang` / `--no-color` behavior.

## 10. Distribution

- **`cargo-dist`** produces cross-platform binaries plus `.tar.gz` / `.deb` artifacts on
  GitHub Releases.
- A Homebrew **tap** repo named `UtmostBoundary/homebrew-tap`, with an auto-updated formula
  (generated by cargo-dist), giving:

  ```
  brew install UtmostBoundary/tap/hijri
  ```

- **README** documents `brew install`, the GitHub-release install, and `cargo install hijri`.

**Explicitly out of v1 scope (documented fast-follows):**

- A self-hosted APT repository (so `apt install hijri` works directly).
- Submission to `homebrew-core` (would make it `brew install hijri`).
- An `--source online` observational/announced-dates mode.

## 11. Non-goals (v1)

- Live moon-sighting / announced religious dates (requires network; see §2).
- Prayer times, qibla, age-in-Hijri calculators, or other almanac features.
- The tabular/arithmetic method.
