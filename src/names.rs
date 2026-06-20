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

/// Canonical ASCII (letters-only, lowercase) month keys for name parsing,
/// index 0 = month 1. Diacritics and separators from the display names are
/// stripped so `month_name` and these stay in sync conceptually.
const MONTH_KEYS: [&str; 12] = [
    "muharram", "safar", "rabialawwal", "rabialthani",
    "jumadaalula", "jumadaalakhira", "rajab", "shaban",
    "ramadan", "shawwal", "dhualqada", "dhualhijja",
];

/// Common alternate spellings / numbered forms users type, mapped to a 1-based
/// month. Checked before prefix matching.
const MONTH_ALIASES: [(&str, u8); 7] = [
    ("ramadhan", 9),
    ("rabi1", 3),
    ("rabi2", 4),
    ("rabialthani", 4),
    ("jumada1", 5),
    ("jumada2", 6),
    ("dhulhijja", 12),
];

/// Lowercase a string keeping only ASCII alphanumerics (so "Dhū al-Ḥijja",
/// "rabi-1", and "Ramadan" normalize to comparable keys).
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Parse a Hijri month name or abbreviation to a 1-based month number.
/// Case-insensitive, ignores non-alphanumerics. Matches an exact key, a known
/// alias, or a unique prefix (e.g. "muh" → 1). Ambiguous prefixes like "rabi"
/// (Rabīʿ al-Awwal vs al-Thānī) return None — disambiguate with "rabi1"/"rabi2".
pub fn month_number(name: &str) -> Option<u8> {
    let n = normalize(name);
    if n.is_empty() {
        return None;
    }
    if let Some(i) = MONTH_KEYS.iter().position(|k| *k == n) {
        return Some(i as u8 + 1);
    }
    if let Some((_, m)) = MONTH_ALIASES.iter().find(|(a, _)| *a == n) {
        return Some(*m);
    }
    let matches: Vec<u8> = MONTH_KEYS
        .iter()
        .enumerate()
        .filter(|(_, k)| k.starts_with(&n))
        .map(|(i, _)| i as u8 + 1)
        .collect();
    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
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

    #[test]
    fn month_number_exact_and_caseinsensitive() {
        assert_eq!(month_number("Muharram"), Some(1));
        assert_eq!(month_number("ramadan"), Some(9));
        assert_eq!(month_number("RAJAB"), Some(7));
    }

    #[test]
    fn month_number_unique_prefix() {
        assert_eq!(month_number("muh"), Some(1));
        assert_eq!(month_number("shaw"), Some(10));
    }

    #[test]
    fn month_number_aliases() {
        assert_eq!(month_number("ramadhan"), Some(9));
        assert_eq!(month_number("rabi2"), Some(4));
        assert_eq!(month_number("dhul-hijja"), Some(12));
    }

    #[test]
    fn month_number_ambiguous_or_unknown_is_none() {
        assert_eq!(month_number("rabi"), None); // ambiguous: al-Awwal vs al-Thani
        assert_eq!(month_number("xyz"), None);
        assert_eq!(month_number(""), None);
    }
}
