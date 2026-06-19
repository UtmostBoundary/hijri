#![allow(dead_code)]

use icu_calendar::cal::Hijri;
use icu_calendar::Date;

/// A date in the Umm al-Qura Hijri calendar. `month` and `day` are 1-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HijriDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// A proleptic Gregorian (ISO) date plus its weekday index (0 = Sunday .. 6 = Saturday).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GregorianDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub weekday: u8,
}

/// Errors the engine can surface.
#[derive(Debug, PartialEq, Eq)]
pub enum EngineError {
    /// The supplied (year, month, day) is not a real date in its calendar.
    InvalidDate { year: i32, month: u8, day: u8 },
}

/// Convert a Gregorian (ISO) date to Umm al-Qura Hijri.
pub fn gregorian_to_hijri(year: i32, month: u8, day: u8) -> Result<HijriDate, EngineError> {
    let iso = Date::try_new_iso(year, month, day)
        .map_err(|_| EngineError::InvalidDate { year, month, day })?;
    let h = iso.to_calendar(Hijri::new_umm_al_qura());
    Ok(HijriDate {
        year: h.era_year().year,
        month: h.month().ordinal,
        day: h.day_of_month().0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_known_gregorian_date() {
        // Verified against ICU4X 2.2: 2026-06-18 -> 1448-01-03
        let h = gregorian_to_hijri(2026, 6, 18).unwrap();
        assert_eq!(h, HijriDate { year: 1448, month: 1, day: 3 });
    }

    #[test]
    fn rejects_invalid_gregorian_date() {
        let err = gregorian_to_hijri(2026, 13, 1).unwrap_err();
        assert_eq!(err, EngineError::InvalidDate { year: 2026, month: 13, day: 1 });
    }
}
