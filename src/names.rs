/// Output language for names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    /// English transliteration (e.g. "Muḥarram").
    En,
    /// Arabic script (e.g. "مُحَرَّم").
    Ar,
}

/// Hijri month names, index 0 = month 1 (Muharram).
const MONTHS_EN: [&str; 12] = [
    "Muḥarram", "Ṣafar", "Rabīʿ al-Awwal", "Rabīʿ al-Thānī",
    "Jumādā al-Ūlā", "Jumādā al-Ākhira", "Rajab", "Shaʿbān",
    "Ramaḍān", "Shawwāl", "Dhū al-Qaʿda", "Dhū al-Ḥijja",
];

const MONTHS_AR: [&str; 12] = [
    "محرم", "صفر", "ربيع الأول", "ربيع الآخر",
    "جمادى الأولى", "جمادى الآخرة", "رجب", "شعبان",
    "رمضان", "شوال", "ذو القعدة", "ذو الحجة",
];

/// Weekday names, index 0 = Sunday .. 6 = Saturday.
const WEEKDAYS_EN: [&str; 7] =
    ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

/// Two-letter weekday headers for the calendar grid, index 0 = Sunday.
pub const WEEKDAY_ABBR: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// Name of a Hijri month. `month` is 1-based; panics outside 1..=12.
pub fn month_name(month: u8, lang: Lang) -> &'static str {
    let i = (month - 1) as usize;
    match lang {
        Lang::En => MONTHS_EN[i],
        Lang::Ar => MONTHS_AR[i],
    }
}

/// Name of a weekday. `index` is 0 = Sunday .. 6 = Saturday.
pub fn weekday_name(index: u8) -> &'static str {
    WEEKDAYS_EN[index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_one_is_muharram() {
        assert_eq!(month_name(1, Lang::En), "Muḥarram");
        assert_eq!(month_name(1, Lang::Ar), "محرم");
    }

    #[test]
    fn month_twelve_is_dhul_hijja() {
        assert_eq!(month_name(12, Lang::En), "Dhū al-Ḥijja");
    }

    #[test]
    fn weekday_four_is_thursday() {
        assert_eq!(weekday_name(4), "Thursday");
    }
}
