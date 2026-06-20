use crate::engine::{GregorianDate, HijriDate};
use crate::names::{month_name, weekday_name, Lang};

/// Format the one-line "today / converted date" view, e.g.
/// `3 Muḥarram 1448 AH  (Thu, 2026-06-18)`.
pub fn date_line(h: &HijriDate, g: &GregorianDate, lang: Lang) -> String {
    let weekday_short = &weekday_name(g.weekday)[..3];
    format!(
        "{} {} {} AH  ({}, {:04}-{:02}-{:02})",
        h.day,
        month_name(h.month, lang),
        h.year,
        weekday_short,
        g.year,
        g.month,
        g.day,
    )
}

use crate::engine::{hijri_to_gregorian, EngineError};
use crate::events::events_in_month;
use crate::events::EVENTS;
use crate::names::WEEKDAY_ABBR;

/// Visual width of one month block (7 cells of 2 chars + 6 single-space gaps).
const COLW: usize = 20;
/// ANSI reverse-video on/off, used to highlight "today".
const REV: &str = "\x1b[7m";
const RST: &str = "\x1b[0m";

/// Number of days in a Hijri month. ICU4X rejects an out-of-range day, so a
/// successful round-trip of day 30 proves the month has 30 days; otherwise 29.
fn days_in_hijri_month(year: i32, month: u8) -> u8 {
    if hijri_to_gregorian(year, month, 30).is_ok() {
        30
    } else {
        29
    }
}

/// Weekday index (0 = Sunday) of the 1st of a Hijri month. Errors if the year
/// is outside the range ICU4X can convert.
fn first_weekday(year: i32, month: u8) -> Result<u8, EngineError> {
    Ok(hijri_to_gregorian(year, month, 1)?.weekday)
}

/// Display width of `s`, ignoring ANSI CSI escape sequences (so a reverse-video
/// cell still measures as its visible characters). Counts Unicode scalars,
/// which equals column width for the ASCII digits/spaces used in the grid.
fn display_width(s: &str) -> usize {
    let mut w = 0;
    let mut in_esc = false;
    for c in s.chars() {
        if in_esc {
            if c == 'm' {
                in_esc = false;
            }
        } else if c == '\x1b' {
            in_esc = true;
        } else {
            w += 1;
        }
    }
    w
}

/// Right-pad `s` with spaces to `width` visible columns (ANSI-aware).
fn pad_to(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

/// Build the lines of one month block: a centered title, the weekday header,
/// and the week rows — each padded to `COLW` visible columns. "Today" is
/// reverse-video when `color` is true. No event legend (callers add that).
fn month_block(
    year: i32,
    month: u8,
    today: Option<(i32, u8, u8)>,
    lang: Lang,
    color: bool,
) -> Result<Vec<String>, EngineError> {
    let lead = first_weekday(year, month)? as usize;
    let total = days_in_hijri_month(year, month);

    let mut lines = Vec::new();

    let title = format!("{} {}", month_name(month, lang), year);
    let pad = COLW.saturating_sub(title.chars().count()) / 2;
    lines.push(pad_to(&format!("{}{}", " ".repeat(pad), title), COLW));
    lines.push(WEEKDAY_ABBR.join(" "));

    let mut week: Vec<String> = vec!["  ".to_string(); lead];
    for day in 1..=total {
        let is_today = today == Some((year, month, day));
        let cell = if is_today && color {
            format!("{}{:>2}{}", REV, day, RST)
        } else {
            format!("{:>2}", day)
        };
        week.push(cell);
        if week.len() == 7 {
            lines.push(pad_to(&week.join(" "), COLW));
            week.clear();
        }
    }
    if !week.is_empty() {
        while week.len() < 7 {
            week.push("  ".to_string());
        }
        lines.push(pad_to(&week.join(" "), COLW));
    }

    Ok(lines)
}

/// Render a `cal`-style grid for a single Hijri month, followed by the event
/// legend for that month. `today` highlights the current day (reverse-video
/// when `color`). Errors if the month/year is outside the convertible range.
pub fn month_grid(
    year: i32,
    month: u8,
    today: Option<(i32, u8, u8)>,
    lang: Lang,
    color: bool,
) -> Result<String, EngineError> {
    let block = month_block(year, month, today, lang, color)?;
    let mut out: String = block
        .iter()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    out.push('\n');

    let evs = events_in_month(month);
    for e in evs {
        out.push_str(&format!("  {} {} — {}\n", e.day, month_name(month, lang), e.name));
    }
    Ok(out)
}

/// Render a full Hijri year as a `cal`-style 12-month grid, 3 months across and
/// 4 rows down, with a centered year header. `today` highlights the current day
/// across all months (reverse-video when `color`). No event legend, to keep the
/// overview compact like `cal`.
pub fn year_grid(
    year: i32,
    today: Option<(i32, u8, u8)>,
    lang: Lang,
    color: bool,
) -> Result<String, EngineError> {
    const GAP: &str = "  ";
    let total_w = COLW * 3 + GAP.len() * 2;

    let mut out = String::new();
    let header = format!("{}", year);
    let pad = total_w.saturating_sub(header.len()) / 2;
    out.push_str(&" ".repeat(pad));
    out.push_str(&header);
    out.push_str("\n\n");

    for row in 0..4 {
        let blocks: Vec<Vec<String>> = (0..3)
            .map(|c| {
                let month = (row * 3 + c + 1) as u8;
                month_block(year, month, today, lang, color)
            })
            .collect::<Result<_, _>>()?;

        let max_lines = blocks.iter().map(Vec::len).max().unwrap_or(0);
        for i in 0..max_lines {
            let blank = " ".repeat(COLW);
            let parts: Vec<String> = blocks
                .iter()
                .map(|b| pad_to(b.get(i).unwrap_or(&blank), COLW))
                .collect();
            out.push_str(parts.join(GAP).trim_end());
            out.push('\n');
        }
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn grid_has_title_and_weekday_header() {
        let g = month_grid(1448, 1, None, Lang::En, false).unwrap();
        assert!(g.contains("Muḥarram 1448"));
        assert!(g.contains("Su Mo Tu We Th Fr Sa"));
    }

    #[test]
    fn grid_lists_events_for_muharram() {
        let g = month_grid(1448, 1, None, Lang::En, false).unwrap();
        assert!(g.contains("Islamic New Year"));
        assert!(g.contains("Ashura"));
    }

    #[test]
    fn grid_highlights_today_with_ansi_when_color_on() {
        let g = month_grid(1448, 1, Some((1448, 1, 3)), Lang::En, true).unwrap();
        assert!(g.contains("\x1b[7m 3\x1b[0m"));
    }

    #[test]
    fn grid_no_ansi_when_color_off() {
        let g = month_grid(1448, 1, Some((1448, 1, 3)), Lang::En, false).unwrap();
        assert!(!g.contains('\x1b'));
    }

    #[test]
    fn grid_week_rows_stay_aligned_with_highlight() {
        // The reverse-video "today" cell must not change the visible width of
        // its week row, so the calendar columns stay aligned.
        let block = month_block(1448, 1, Some((1448, 1, 3)), Lang::En, true).unwrap();
        for line in &block {
            assert_eq!(display_width(line), COLW, "line not {COLW} wide: {line:?}");
        }
    }

    #[test]
    fn grid_errors_on_out_of_range_year_instead_of_panicking() {
        assert!(month_grid(100000, 1, None, Lang::En, false).is_err());
    }

    #[test]
    fn year_grid_shows_all_twelve_months_and_header() {
        let y = year_grid(1448, None, Lang::En, false).unwrap();
        assert!(y.contains("1448"));
        for m in 1..=12u8 {
            assert!(
                y.contains(month_name(m, Lang::En)),
                "missing month {}",
                month_name(m, Lang::En)
            );
        }
    }

    #[test]
    fn year_grid_errors_on_out_of_range_year() {
        assert!(year_grid(100000, None, Lang::En, false).is_err());
    }
}

/// Render the events list for a Hijri year, each with its Gregorian date.
/// Ends with the observance caveat line. Errors if the year is outside the
/// convertible range.
pub fn events_list(year: i32, lang: Lang) -> Result<String, EngineError> {
    let mut out = String::new();
    out.push_str(&format!("Major events — {} AH\n", year));
    for e in EVENTS.iter() {
        let g = hijri_to_gregorian(year, e.month, e.day)?;
        out.push_str(&format!(
            "  {:<18} {:>2} {:<16} {:04}-{:02}-{:02}\n",
            e.name,
            e.day,
            month_name(e.month, lang),
            g.year,
            g.month,
            g.day,
        ));
    }
    out.push_str("\nNote: dates are Umm al-Qura calculated; local religious observance may differ by ±1 day.\n");
    Ok(out)
}

#[cfg(test)]
mod events_list_tests {
    use super::*;

    #[test]
    fn lists_all_events_with_caveat() {
        let s = events_list(1448, Lang::En).unwrap();
        assert!(s.contains("Eid al-Adha"));
        assert!(s.contains("Ramadan (start)"));
        assert!(s.contains("may differ by ±1 day"));
    }

    #[test]
    fn errors_on_out_of_range_year_instead_of_panicking() {
        assert!(events_list(100000, Lang::En).is_err());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_date_line() {
        let h = HijriDate { year: 1448, month: 1, day: 3 };
        let g = GregorianDate { year: 2026, month: 6, day: 18, weekday: 4 };
        assert_eq!(date_line(&h, &g, Lang::En), "3 Muḥarram 1448 AH  (Thu, 2026-06-18)");
    }
}
