use crate::engine::{GregorianDate, HijriDate};
use crate::names::{month_name, weekday_name, Lang};
use serde::Serialize;

#[derive(Serialize)]
struct HijriOut {
    year: i32,
    month: u8,
    day: u8,
    month_name: String,
}

#[derive(Serialize)]
struct GregorianOut {
    year: i32,
    month: u8,
    day: u8,
    weekday: String,
}

#[derive(Serialize)]
struct DateOut {
    hijri: HijriOut,
    gregorian: GregorianOut,
    method: &'static str,
}

/// Serialize a single converted/looked-up date to pretty JSON.
pub fn date_json(h: &HijriDate, g: &GregorianDate, lang: Lang) -> String {
    let out = DateOut {
        hijri: HijriOut {
            year: h.year,
            month: h.month,
            day: h.day,
            month_name: month_name(h.month, lang).to_string(),
        },
        gregorian: GregorianOut {
            year: g.year,
            month: g.month,
            day: g.day,
            weekday: weekday_name(g.weekday).to_string(),
        },
        method: "umm-al-qura",
    };
    serde_json::to_string_pretty(&out).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_expected_fields() {
        let h = HijriDate { year: 1448, month: 1, day: 3 };
        let g = GregorianDate { year: 2026, month: 6, day: 18, weekday: 4 };
        let s = date_json(&h, &g, Lang::En);
        assert!(s.contains("\"method\": \"umm-al-qura\""));
        assert!(s.contains("\"year\": 1448"));
        assert!(s.contains("\"weekday\": \"Thursday\""));
    }
}
