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
