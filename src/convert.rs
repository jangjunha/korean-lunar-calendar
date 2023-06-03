use chrono::{Days, NaiveDate};

use crate::dataset::{BASE, MONTHS};
use crate::date::LunarDate;
use crate::month::M;

pub fn lunar_to_gregorian(lunar_date: &LunarDate) -> Option<NaiveDate> {
    let LunarDate {
        year: ty,
        month: (tm, tl),
        day: td,
    } = lunar_date;

    let mut days_sum: u64 = 0;
    for M { y, m, l, d } in MONTHS.iter() {
        if *ty == *y && *tm == *m && *tl == *l {
            return Some(*BASE + Days::new(days_sum + (*td as u64) - 1));
        }
        days_sum += *d as u64
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lunar_to_gregorian() {
        assert_eq!(
            lunar_to_gregorian(&LunarDate {
                year: 1583,
                month: (1, false),
                day: 1
            }),
            NaiveDate::from_ymd_opt(1583, 1, 24)
        );

        assert_eq!(
            lunar_to_gregorian(&LunarDate {
                year: 1993,
                month: (3, true),
                day: 25
            }),
            NaiveDate::from_ymd_opt(1993, 5, 16)
        );

        assert_eq!(
            lunar_to_gregorian(&LunarDate {
                year: 1582,
                month: (12, false),
                day: 30
            }),
            None,
        );
    }
}
