use crate::engine::{gregorian_to_hijri, hijri_to_gregorian, EngineError, GregorianDate, HijriDate};
use crate::names::{self, Lang};
use crate::{json, render};
use clap::{Parser, Subcommand};
use std::io::IsTerminal;
use time::OffsetDateTime;

#[derive(Parser)]
#[command(
    name = "hijri",
    version,
    about = "Hijri (Islamic) calendar in your terminal",
    args_conflicts_with_subcommands = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Shortcut: a bare Hijri year (e.g. `hijri 1448`) shows that whole year;
    /// a bare month name (e.g. `hijri ramadan`) shows that month of this year.
    pub target: Option<String>,

    /// Conversion method (only umm-al-qura is supported).
    #[arg(long, global = true, default_value = "umm-al-qura")]
    pub method: String,

    /// Output as JSON.
    #[arg(long, global = true)]
    pub json: bool,

    /// Name language: en (transliteration) or ar (Arabic script).
    #[arg(long, global = true, default_value = "en")]
    pub lang: String,

    /// Disable colored output (also auto-disabled when not writing to a terminal).
    #[arg(long, global = true)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show today's Hijri date (default when no subcommand).
    Today,
    /// Show a Hijri calendar. No args: current month. One arg: a year (whole-year
    /// grid) or a month name (that month this year). Two args: <month> <year>.
    Cal {
        first: Option<String>,
        second: Option<String>,
    },
    /// Convert a date (Gregorian <-> Hijri, auto-detected).
    Convert {
        date: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    /// List major Islamic events for a Hijri year.
    Events { year: Option<i32> },
}

/// Parse "YYYY-MM-DD" into (year, month, day).
pub fn parse_ymd(s: &str) -> Result<(i32, u8, u8), String> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(format!("expected YYYY-MM-DD, got '{}'", s));
    }
    let y = parts[0].parse::<i32>().map_err(|_| format!("bad year in '{}'", s))?;
    let m = parts[1].parse::<u8>().map_err(|_| format!("bad month in '{}'", s))?;
    let d = parts[2].parse::<u8>().map_err(|_| format!("bad day in '{}'", s))?;
    Ok((y, m, d))
}

fn parse_lang(s: &str) -> Lang {
    match s {
        "ar" => Lang::Ar,
        _ => Lang::En,
    }
}

/// Today's Gregorian date in local time (falls back to UTC if the local
/// offset can't be determined).
fn today_gregorian() -> (i32, u8, u8) {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    (now.year(), now.month() as u8, now.day())
}

/// Resolve a Hijri date + matching Gregorian date for output.
fn both_from_gregorian(y: i32, m: u8, d: u8) -> Result<(HijriDate, GregorianDate), EngineError> {
    let h = gregorian_to_hijri(y, m, d)?;
    let weekday = crate::engine::gregorian_weekday(y, m, d)?;
    Ok((h, GregorianDate { year: y, month: m, day: d, weekday }))
}

fn both_from_hijri(y: i32, m: u8, d: u8) -> Result<(HijriDate, GregorianDate), EngineError> {
    let g = hijri_to_gregorian(y, m, d)?;
    Ok((HijriDate { year: y, month: m, day: d }, g))
}

fn print_date(h: &HijriDate, g: &GregorianDate, lang: Lang, as_json: bool) {
    if as_json {
        println!("{}", json::date_json(h, g, lang));
    } else {
        println!("{}", render::date_line(h, g, lang));
    }
}

/// Parse a month given as a name (e.g. "ramadan", "muh") or a 1..=12 number.
fn parse_month(s: &str) -> Result<u8, String> {
    if let Some(m) = names::month_number(s) {
        return Ok(m);
    }
    match s.parse::<u8>() {
        Ok(m) if (1..=12).contains(&m) => Ok(m),
        _ => Err(format!("'{}' is not a month name or number (1-12)", s)),
    }
}

/// Render a calendar, mirroring `cal`'s argument rules:
/// - no args        → current Hijri month
/// - one number     → whole-year grid for that Hijri year
/// - one month name → that month of the current Hijri year
/// - two args       → <month (name or number)> <year>
fn show_calendar(
    first: Option<&str>,
    second: Option<&str>,
    lang: Lang,
    color: bool,
) -> Result<(), String> {
    let (ty, tm, td) = today_gregorian();
    let today_h = gregorian_to_hijri(ty, tm, td).map_err(fmt_engine_err)?;
    let today_tuple = Some((today_h.year, today_h.month, today_h.day));

    let output = match (first, second) {
        (None, _) => render::month_grid(today_h.year, today_h.month, today_tuple, lang, color),
        (Some(a), None) => {
            if let Some(m) = names::month_number(a) {
                render::month_grid(today_h.year, m, today_tuple, lang, color)
            } else if let Ok(y) = a.parse::<i32>() {
                render::year_grid(y, today_tuple, lang, color)
            } else {
                return Err(format!("'{}' is not a month name or year", a));
            }
        }
        (Some(a), Some(b)) => {
            let m = parse_month(a)?;
            let y = b.parse::<i32>().map_err(|_| format!("invalid year '{}'", b))?;
            render::month_grid(y, m, today_tuple, lang, color)
        }
    };
    print!("{}", output.map_err(fmt_engine_err)?);
    Ok(())
}

/// Run the CLI. Returns Err(message) for any user-facing failure.
pub fn run(cli: Cli) -> Result<(), String> {
    if cli.method != "umm-al-qura" {
        return Err(format!("unsupported method '{}' (only 'umm-al-qura')", cli.method));
    }
    let lang = parse_lang(&cli.lang);
    let color = !cli.no_color && std::io::stdout().is_terminal();

    // A bare positional (`hijri 1448` / `hijri ramadan`) is shorthand for `cal`.
    let command = match (cli.command, cli.target) {
        (Some(cmd), _) => cmd,
        (None, Some(t)) => Command::Cal { first: Some(t), second: None },
        (None, None) => Command::Today,
    };

    match command {
        Command::Today => {
            let (y, m, d) = today_gregorian();
            let (h, g) = both_from_gregorian(y, m, d).map_err(fmt_engine_err)?;
            print_date(&h, &g, lang, cli.json);
        }
        Command::Cal { first, second } => {
            show_calendar(first.as_deref(), second.as_deref(), lang, color)?;
        }
        Command::Convert { date, from, to } => {
            let (y, m, d) = parse_ymd(&date)?;
            let direction = resolve_direction(y, from.as_deref(), to.as_deref())?;
            let (h, g) = match direction {
                Direction::FromGregorian => both_from_gregorian(y, m, d),
                Direction::FromHijri => both_from_hijri(y, m, d),
            }
            .map_err(fmt_engine_err)?;
            print_date(&h, &g, lang, cli.json);
        }
        Command::Events { year } => {
            let y = match year {
                Some(y) => y,
                None => {
                    let (ty, tm, td) = today_gregorian();
                    gregorian_to_hijri(ty, tm, td).map_err(fmt_engine_err)?.year
                }
            };
            print!("{}", render::events_list(y, lang).map_err(fmt_engine_err)?);
        }
    }
    Ok(())
}

enum Direction { FromGregorian, FromHijri }

/// Validate a `--from`/`--to` calendar name, returning an error on anything
/// other than "gregorian" or "hijri".
fn parse_calendar(which: &str, value: &str) -> Result<bool, String> {
    match value {
        "hijri" => Ok(true),
        "gregorian" => Ok(false),
        other => Err(format!(
            "invalid --{} value '{}' (expected 'gregorian' or 'hijri')",
            which, other
        )),
    }
}

/// Decide conversion direction. Explicit --from/--to win (and are validated);
/// otherwise a year >= 1700 is treated as Gregorian, else Hijri.
fn resolve_direction(year: i32, from: Option<&str>, to: Option<&str>) -> Result<Direction, String> {
    if let Some(f) = from {
        return Ok(if parse_calendar("from", f)? { Direction::FromHijri } else { Direction::FromGregorian });
    }
    if let Some(t) = to {
        return Ok(if parse_calendar("to", t)? { Direction::FromGregorian } else { Direction::FromHijri });
    }
    Ok(if year >= 1700 { Direction::FromGregorian } else { Direction::FromHijri })
}

fn fmt_engine_err(e: EngineError) -> String {
    match e {
        EngineError::InvalidDate { year, month, day } => {
            format!("invalid date {:04}-{:02}-{:02}", year, month, day)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_iso_date() {
        assert_eq!(parse_ymd("2026-06-18").unwrap(), (2026, 6, 18));
    }

    #[test]
    fn rejects_garbage_date() {
        assert!(parse_ymd("hello").is_err());
        assert!(parse_ymd("2026/06/18").is_err());
    }

    #[test]
    fn direction_defaults_by_year() {
        assert!(matches!(resolve_direction(2026, None, None).unwrap(), Direction::FromGregorian));
        assert!(matches!(resolve_direction(1447, None, None).unwrap(), Direction::FromHijri));
        assert!(matches!(resolve_direction(1447, Some("gregorian"), None).unwrap(), Direction::FromGregorian));
    }

    #[test]
    fn direction_rejects_unknown_calendar() {
        assert!(resolve_direction(1447, Some("hijriii"), None).is_err());
        assert!(resolve_direction(2026, None, Some("nonsense")).is_err());
    }
}
