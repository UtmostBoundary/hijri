use assert_cmd::Command;
use predicates::str::contains;

fn hijri() -> Command {
    Command::cargo_bin("hijri").unwrap()
}

#[test]
fn convert_gregorian_to_hijri() {
    hijri()
        .args(["convert", "2026-06-18"])
        .assert()
        .success()
        .stdout(contains("3 Muḥarram 1448 AH"))
        .stdout(contains("Thu, 2026-06-18"));
}

#[test]
fn convert_hijri_to_gregorian() {
    hijri()
        .args(["convert", "1447-12-10"])
        .assert()
        .success()
        .stdout(contains("2026-05-27"));
}

#[test]
fn json_output_has_method() {
    hijri()
        .args(["convert", "2026-06-18", "--json"])
        .assert()
        .success()
        .stdout(contains("\"method\": \"umm-al-qura\""));
}

#[test]
fn cal_shows_events() {
    hijri()
        .args(["cal", "1", "1448"])
        .assert()
        .success()
        .stdout(contains("Islamic New Year"))
        .stdout(contains("Ashura"));
}

#[test]
fn events_list_has_caveat() {
    hijri()
        .args(["events", "1448"])
        .assert()
        .success()
        .stdout(contains("Eid al-Adha"))
        .stdout(contains("may differ by ±1 day"));
}

#[test]
fn invalid_date_errors() {
    hijri()
        .args(["convert", "2026-13-99"])
        .assert()
        .failure()
        .stderr(contains("invalid date"));
}

#[test]
fn unsupported_method_errors() {
    hijri()
        .args(["convert", "2026-06-18", "--method", "tabular"])
        .assert()
        .failure()
        .stderr(contains("unsupported method"));
}

#[test]
fn arabic_month_name() {
    hijri()
        .args(["convert", "2026-06-18", "--lang", "ar"])
        .assert()
        .success()
        .stdout(contains("محرم"));
}

#[test]
fn cal_out_of_range_year_errors_without_panicking() {
    hijri()
        .args(["cal", "1", "100000"])
        .assert()
        .failure()
        .stderr(contains("invalid date"));
}

#[test]
fn events_out_of_range_year_errors_without_panicking() {
    hijri()
        .args(["events", "100000"])
        .assert()
        .failure()
        .stderr(contains("invalid date"));
}

#[test]
fn convert_rejects_unknown_from_value() {
    hijri()
        .args(["convert", "1447-12-10", "--from", "hijriii"])
        .assert()
        .failure()
        .stderr(contains("invalid --from value"));
}

#[test]
fn cal_single_year_shows_whole_year() {
    // One numeric arg = whole-year grid (like `cal 2026`): header + all months.
    hijri()
        .args(["cal", "1448"])
        .assert()
        .success()
        .stdout(contains("1448"))
        .stdout(contains("Muḥarram"))
        .stdout(contains("Ramaḍān"))
        .stdout(contains("Dhū al-Ḥijja"));
}

#[test]
fn bare_year_is_shorthand_for_cal_year() {
    hijri()
        .args(["1448"])
        .assert()
        .success()
        .stdout(contains("Muḥarram"))
        .stdout(contains("Ramaḍān"));
}

#[test]
fn cal_accepts_month_name_with_year() {
    hijri()
        .args(["cal", "ramadan", "1448"])
        .assert()
        .success()
        .stdout(contains("Ramaḍān 1448"))
        .stdout(contains("Su Mo Tu We Th Fr Sa"));
}

#[test]
fn bare_month_name_shows_that_month() {
    hijri()
        .args(["rajab"])
        .assert()
        .success()
        .stdout(contains("Rajab"));
}

#[test]
fn cal_month_number_and_year_still_works() {
    // Backward compatibility: two numbers = <month> <year>.
    hijri()
        .args(["cal", "9", "1448"])
        .assert()
        .success()
        .stdout(contains("Ramaḍān 1448"));
}

#[test]
fn cal_ambiguous_month_name_errors() {
    hijri()
        .args(["cal", "rabi", "1448"])
        .assert()
        .failure()
        .stderr(contains("not a month name or number"));
}
