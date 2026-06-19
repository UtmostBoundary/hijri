
use icu_calendar::cal::Hijri;
use icu_calendar::{Date, Iso};

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

/// Map ICU4X weekday to 0 = Sunday .. 6 = Saturday.
fn weekday_index(w: icu_calendar::types::Weekday) -> u8 {
    use icu_calendar::types::Weekday::*;
    match w {
        Sunday => 0,
        Monday => 1,
        Tuesday => 2,
        Wednesday => 3,
        Thursday => 4,
        Friday => 5,
        Saturday => 6,
    }
}

/// Convert an Umm al-Qura Hijri date to Gregorian (ISO), including weekday.
pub fn hijri_to_gregorian(year: i32, month: u8, day: u8) -> Result<GregorianDate, EngineError> {
    let h = Date::try_new_hijri_with_calendar(year, month, day, Hijri::new_umm_al_qura())
        .map_err(|_| EngineError::InvalidDate { year, month, day })?;
    let g = h.to_calendar(Iso);
    Ok(GregorianDate {
        year: g.era_year().year,
        month: g.month().ordinal,
        day: g.day_of_month().0,
        weekday: weekday_index(g.weekday()),
    })
}

/// Weekday index (0 = Sunday .. 6 = Saturday) for a Gregorian date.
pub fn gregorian_weekday(year: i32, month: u8, day: u8) -> Result<u8, EngineError> {
    let iso = Date::try_new_iso(year, month, day)
        .map_err(|_| EngineError::InvalidDate { year, month, day })?;
    Ok(weekday_index(iso.weekday()))
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

    #[test]
    fn converts_known_hijri_date() {
        // Verified against ICU4X 2.2: 1447-12-10 -> 2026-05-27
        let g = hijri_to_gregorian(1447, 12, 10).unwrap();
        assert_eq!((g.year, g.month, g.day), (2026, 5, 27));
    }

    #[test]
    fn computes_gregorian_weekday() {
        // 2026-06-18 is a Thursday (index 4).
        assert_eq!(gregorian_weekday(2026, 6, 18).unwrap(), 4);
    }

    #[test]
    fn far_future_date_is_computed_not_rejected() {
        // ICU4X extrapolates; must NOT error.
        assert!(gregorian_to_hijri(2200, 1, 1).is_ok());
    }
}
