#[cfg(test)]
mod tests {
    use rrule::{RRuleSet};
    use chrono::{DateTime};

    const RRULE: &str = "DTSTART;TZID=Europe/London:20000101T000000\nRRULE:FREQ=DAILY;BYHOUR=0,1,2,3,4;BYMINUTE=0,30";

    const MARCH_31_UTC: i64 = 1711843200000;
    const OCT_27_UTC: i64 = 1729987200000;

    const FOUR_HOURS_MS: i64 = 14400000;

    #[test]
    fn clocks_go_forward() {
        let rrule: RRuleSet = RRULE.parse().unwrap();

        let result = between(rrule, MARCH_31_UTC, MARCH_31_UTC + FOUR_HOURS_MS, true);

        assert_eq!(result, [
            1711843200000, // 00:00:00 GMT+0000 (Greenwich Mean Time) 2024-03-31T00:00:00.000Z
            1711845000000, // 00:30:00 GMT+0000 (Greenwich Mean Time) 2024-03-31T00:30:00.000Z
            1711846800000, // 02:00:00 GMT+0100 (British Summer Time) 2024-03-31T01:00:00.000Z
            1711848600000, // 02:30:00 GMT+0100 (British Summer Time) 2024-03-31T01:30:00.000Z

            // 1711846800000, // [DUPLICATE] 02:00:00 GMT+0100 (British Summer Time) 2024-03-31T01:00:00.000Z
            // 1711848600000, // [DUPLICATE] 02:30:00 GMT+0100 (British Summer Time) 2024-03-31T01:30:00.000Z

            1711850400000, // 03:00:00 GMT+0100 (British Summer Time) 2024-03-31T02:00:00.000Z
            1711852200000, // 03:30:00 GMT+0100 (British Summer Time) 2024-03-31T02:30:00.000Z
            1711854000000, // 04:00:00 GMT+0100 (British Summer Time) 2024-03-31T03:00:00.000Z
            1711855800000, // 04:30:00 GMT+0100 (British Summer Time) 2024-03-31T03:30:00.000Z
            1711857600000, // [MISSING] 05:00:00 GMT+0100 (British Summer Time) 2024-03-31T04:00:00.000Z
        ]);
    }

    #[test]
    fn clocks_go_back() {
        let rrule: RRuleSet = RRULE.parse().unwrap();

        let result = between(rrule, OCT_27_UTC, OCT_27_UTC + FOUR_HOURS_MS, true);

        assert_eq!(result, [
            1729987200000, // 01:00:00 GMT+0100 (British Summer Time) 2024-10-27T00:00:00.000Z
            1729989000000, // 01:30:00 GMT+0100 (British Summer Time) 2024-10-27T00:30:00.000Z

            1729990800000, // [MISSING] 01:00:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T01:00:00.000Z
            1729992600000, // [MISSING] 01:30:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T01:30:00.000Z

            1729994400000, // 02:00:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T02:00:00.000Z
            1729996200000, // 02:30:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T02:30:00.000Z
            1729998000000, // 03:00:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T03:00:00.000Z
            1729999800000, // 03:30:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T03:30:00.000Z
            1730001600000, // 04:00:00 GMT+0000 (Greenwich Mean Time) 2024-10-27T04:00:00.000Z
        ]);
    }

    pub fn between(
        rrule_set: RRuleSet,
        after: i64,
        before: i64,
        inclusive: bool,
    ) -> Vec<i64> {
        let mut vec = Vec::new();

        for date in rrule_set.into_iter() {
            let date_timestamp = date.timestamp_millis();
            let is_after = is_after(date_timestamp, after, inclusive);
            let is_before = is_before(date_timestamp, before, inclusive);

            if is_after && is_before {
                let datetime: i64 = DateTime::from(date).timestamp_millis();
                vec.push(datetime);
            } else if !is_before {
                break;
            }
        }

        return vec;
    }

    fn is_after(timestamp: i64, after_timestamp: i64, inclusive: bool) -> bool {
        if inclusive && timestamp < after_timestamp {
            return false;
        } else if !inclusive && timestamp <= after_timestamp {
            return false;
        }

        true
    }

    fn is_before(timestamp: i64, before_timestamp: i64, inclusive: bool) -> bool {
        if inclusive && timestamp > before_timestamp {
            return false;
        } else if !inclusive && timestamp >= before_timestamp {
            return false;
        }

        true
    }
}
