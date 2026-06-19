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
