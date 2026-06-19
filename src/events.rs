/// A major Islamic event that falls on a fixed date in the Hijri calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub name: &'static str,
    pub month: u8,
    pub day: u8,
}

/// Major fixed-Hijri-date events, in calendar order.
pub const EVENTS: [Event; 6] = [
    Event { name: "Islamic New Year", month: 1,  day: 1 },
    Event { name: "Ashura",           month: 1,  day: 10 },
    Event { name: "Mawlid",           month: 3,  day: 12 },
    Event { name: "Ramadan (start)",  month: 9,  day: 1 },
    Event { name: "Eid al-Fitr",      month: 10, day: 1 },
    Event { name: "Eid al-Adha",      month: 12, day: 10 },
];

/// The event on a given Hijri month/day, if any.
pub fn event_on(month: u8, day: u8) -> Option<&'static Event> {
    EVENTS.iter().find(|e| e.month == month && e.day == day)
}

/// All events in a given Hijri month, in day order.
pub fn events_in_month(month: u8) -> Vec<&'static Event> {
    EVENTS.iter().filter(|e| e.month == month).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eid_al_adha_is_dhul_hijja_10() {
        let e = event_on(12, 10).unwrap();
        assert_eq!(e.name, "Eid al-Adha");
    }

    #[test]
    fn no_event_on_ordinary_day() {
        assert!(event_on(2, 15).is_none());
    }

    #[test]
    fn muharram_has_two_events() {
        assert_eq!(events_in_month(1).len(), 2);
    }
}
