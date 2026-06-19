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

/// Render a `cal`-style month grid for a Hijri month. `today` is an optional
/// (year, month, day) to highlight with brackets. Errors if the month/year is
/// out of the convertible range.
pub fn month_grid(
    year: i32,
    month: u8,
    today: Option<(i32, u8, u8)>,
    lang: Lang,
) -> Result<String, EngineError> {
    let lead = first_weekday(year, month)? as usize;
    let total = days_in_hijri_month(year, month);

    let mut out = String::new();
    let title = format!("{} {}", month_name(month, lang), year);
    // Center the title over a 20-char week row.
    let pad = (20usize.saturating_sub(title.chars().count())) / 2;
    out.push_str(&" ".repeat(pad));
    out.push_str(&title);
    out.push('\n');
    out.push_str(&WEEKDAY_ABBR.join(" "));
    out.push('\n');

    let mut col = 0;
    out.push_str(&"   ".repeat(lead));
    col += lead;

    for day in 1..=total {
        let is_today = today == Some((year, month, day));
        // Today is bracketed (`[3]`); other days are right-aligned in 2 columns.
        if is_today {
            out.push_str(&format!("[{}]", day));
        } else {
            out.push_str(&format!("{:>2}", day));
        }
        col += 1;
        if col % 7 == 0 {
            out.push('\n');
        } else {
            out.push(' ');
        }
    }
    if col % 7 != 0 {
        out.push('\n');
    }

    // Event legend.
    let evs = events_in_month(month);
    if !evs.is_empty() {
        for e in evs {
            out.push_str(&format!("  {} {} — {}\n", e.day, month_name(month, lang), e.name));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn grid_has_title_and_weekday_header() {
        let g = month_grid(1448, 1, None, Lang::En).unwrap();
        assert!(g.contains("Muḥarram 1448"));
        assert!(g.contains("Su Mo Tu We Th Fr Sa"));
    }

    #[test]
    fn grid_lists_events_for_muharram() {
        let g = month_grid(1448, 1, None, Lang::En).unwrap();
        assert!(g.contains("Islamic New Year"));
        assert!(g.contains("Ashura"));
    }

    #[test]
    fn grid_brackets_today() {
        let g = month_grid(1448, 1, Some((1448, 1, 3)), Lang::En).unwrap();
        assert!(g.contains("[3]"));
    }

    #[test]
    fn grid_errors_on_out_of_range_year_instead_of_panicking() {
        assert!(month_grid(100000, 1, None, Lang::En).is_err());
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
