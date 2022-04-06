use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    total_minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours_converted = (self.total_minutes / 60) % 24;
        let minutes_converted = self.total_minutes % 60;
        let hours_print = if hours_converted < 10 {
            format!("0{}", hours_converted)
        } else {
            format!("{}", hours_converted)
        };
        let minutes_print = if minutes_converted < 10 {
            format!("0{}", minutes_converted)
        } else {
            format!("{}", minutes_converted)
        };
        write!(f, "{}:{}", hours_print, minutes_print)
    }
}

impl Clock {
    fn convert_hours_to_minutes(hours: i32) -> i32 {
        // ensure it will not overflow the max hours and convert to minutes
        if hours < 0 {
            let mut helper_hours = hours;
            while helper_hours < 0 {
                helper_hours += 24;
            }
            ((helper_hours + 24) % 24) * 60
        } else {
            (hours % 24) * 60
        }
    }

    fn minutes_converted(minutes: i32) -> i32 {
        // ensure it will not overflow the max minutes
        if minutes < 0 {
            let mut helper_minutes = minutes;
            while helper_minutes < 0 {
                helper_minutes += 1440;
            }
            helper_minutes
        } else if minutes > 1440 {
            let mut helper_minutes = minutes;
            while helper_minutes > 1440 {
                helper_minutes -= 1440;
            }
            helper_minutes
        } else {
            minutes
        }
    }
    fn convert_to_minutes(hours: i32, minutes: i32) -> i32 {
        let hours_in_minutes = Clock::convert_hours_to_minutes(hours);
        let minutes_converted = Clock::minutes_converted(minutes);
        let total = hours_in_minutes + minutes_converted;
        let total = Clock::minutes_converted(total);
        return total;
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = Clock::convert_to_minutes(hours, minutes);
        Clock { total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.total_minutes + minutes;
        let minutes_converted = Clock::minutes_converted(total_minutes);
        Clock {
            total_minutes: minutes_converted,
        }
    }
}
