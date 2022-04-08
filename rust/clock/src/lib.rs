use core::fmt;

const MINUTES_PER_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours_converted = self.minutes / MINUTES_PER_HOUR;
        let minutes_converted = self.minutes % MINUTES_PER_HOUR;
        write!(f, "{:02}:{:02}", hours_converted, minutes_converted)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_in_minutes = hours * 60;
        let minutes = (hours_in_minutes + minutes).rem_euclid(MINUTES_PER_DAY);
        Clock { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        let minutes_converted = (total_minutes).rem_euclid(MINUTES_PER_DAY);
        Clock {
            minutes: minutes_converted,
        }
    }
}
