fn main() {
    let mut date = Date::new();
    let mut count = 0;

    while !date.is_dec_31_2000() {
        if date.is_sunday_first_of_month() && date.year >= 1901 {
            count += 1;
        }
        date.turn()
    }
    println!("{} sundays first of month", count);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum DayInWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayInWeek {
    fn next(&self) -> Self {
        match self {
            DayInWeek::Monday => DayInWeek::Tuesday,
            DayInWeek::Tuesday => DayInWeek::Wednesday,
            DayInWeek::Wednesday => DayInWeek::Thursday,
            DayInWeek::Thursday => DayInWeek::Friday,
            DayInWeek::Friday => DayInWeek::Saturday,
            DayInWeek::Saturday => DayInWeek::Sunday,
            DayInWeek::Sunday => DayInWeek::Monday
        }
    }
    fn turn(&mut self) {
        *self = self.next();
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MonthOfYear {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl MonthOfYear {
    fn number_of_days(&self, is_leap_year: bool) -> u8 {
        match self {
            MonthOfYear::January => 31,
            MonthOfYear::February => if is_leap_year { 29 } else { 28 },
            MonthOfYear::March => 31,
            MonthOfYear::April => 30,
            MonthOfYear::May => 31,
            MonthOfYear::June => 30,
            MonthOfYear::July => 31,
            MonthOfYear::August => 31,
            MonthOfYear::September => 30,
            MonthOfYear::October => 31,
            MonthOfYear::November => 30,
            MonthOfYear::December => 31,
        }
    }

    fn is_last_month_of_year(&self) -> bool {
        match self {
            MonthOfYear::December => true,
            _ => false
        }
    }

    fn is_first_month_of_year(&self) -> bool {
        match self {
            MonthOfYear::January => true,
            _ => false
        }
    }

    fn next(&self) -> Self {
        match self {
            MonthOfYear::January => MonthOfYear::February,
            MonthOfYear::February => MonthOfYear::March,
            MonthOfYear::March => MonthOfYear::April,
            MonthOfYear::April => MonthOfYear::May,
            MonthOfYear::May => MonthOfYear::June,
            MonthOfYear::June => MonthOfYear::July,
            MonthOfYear::July => MonthOfYear::August,
            MonthOfYear::August => MonthOfYear::September,
            MonthOfYear::September => MonthOfYear::October,
            MonthOfYear::October => MonthOfYear::November,
            MonthOfYear::November => MonthOfYear::December,
            MonthOfYear::December => MonthOfYear::January,
        }
    }

    fn turn(&mut self) {
        *self = self.next();
    }
}

fn is_leap_year(year: u64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Date {
    day_in_week: DayInWeek,
    month_of_year: MonthOfYear,
    day_in_month: u8,
    year: u64,
}

impl Date {
    /// Starts from Monday Jan 1 1900
    fn new() -> Self {
        Self {
            day_in_week: DayInWeek::Monday,
            month_of_year: MonthOfYear::January,
            day_in_month: 1,
            year: 1900,
        }
    }

    fn days_in_month(&self) -> u8 {
        self.month_of_year.number_of_days(is_leap_year(self.year))
    }

    fn next(&self) -> Self {
        let mut new = self.clone();
        new.day_in_week.turn();
        new.day_in_month += 1;
        if new.day_in_month > new.days_in_month() {
            new.day_in_month = 1;
            new.month_of_year.turn();
            if new.month_of_year.is_first_month_of_year() {
                new.year += 1;
            }
        }

        new
    }

    fn turn(&mut self) {
        *self = self.next();
    }

    fn is_sunday_first_of_month(&self) -> bool {
        self.day_in_week == DayInWeek::Sunday && self.day_in_month == 1
    }

    fn is_dec_31_2000(&self) -> bool {
        self.day_in_month == 31 && self.month_of_year == MonthOfYear::December && self.year == 2000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_in_week_turn() {
        let mut day_in_week = DayInWeek::Monday;
        day_in_week.turn();
        day_in_week.turn();
        assert_eq!(day_in_week, DayInWeek::Wednesday);
        day_in_week.turn();
        assert_eq!(day_in_week, DayInWeek::Thursday);
        day_in_week.turn();
        day_in_week.turn();
        day_in_week.turn();
        assert_eq!(day_in_week, DayInWeek::Sunday);
        day_in_week.turn();
        assert_eq!(day_in_week, DayInWeek::Monday);
    }

    #[test]
    fn test_months_of_year_turn() {
        let mut month_of_year = MonthOfYear::January;
        month_of_year.turn();
        month_of_year.turn();
        month_of_year.turn();
        assert_eq!(month_of_year, MonthOfYear::April);
        month_of_year.turn();
        month_of_year.turn();
        assert_eq!(month_of_year, MonthOfYear::June);
        month_of_year.turn();
        month_of_year.turn();
        month_of_year.turn();
        month_of_year.turn();
        month_of_year.turn();
        month_of_year.turn();
        assert_eq!(month_of_year, MonthOfYear::December);
        month_of_year.turn();
        assert_eq!(month_of_year, MonthOfYear::January);
    }

    #[test]
    fn test_first_last_month_of_year() {
        assert_eq!(MonthOfYear::January.is_first_month_of_year(), true);
        assert_eq!(MonthOfYear::May.is_first_month_of_year(), false);
        assert_eq!(MonthOfYear::December.is_first_month_of_year(), false);

        assert_eq!(MonthOfYear::January.is_last_month_of_year(), false);
        assert_eq!(MonthOfYear::May.is_last_month_of_year(), false);
        assert_eq!(MonthOfYear::December.is_last_month_of_year(), true);
    }

    #[test]
    fn test_date() {
        let mut date = Date::new();

        for _ in 0..37_398 {
            date.turn();
        }

        assert_eq!(date, Date { day_in_week: DayInWeek::Friday, month_of_year: MonthOfYear::May, day_in_month: 24, year: 2002 });
        assert_eq!(date.is_sunday_first_of_month(), false);
        assert_eq!(date.is_dec_31_2000(), false);
    }
}