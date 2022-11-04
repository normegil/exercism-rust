use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }.normalize_minutes().normalize_hours()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn normalize_minutes(&self) -> Self {
        let mut change_hours = 0;
        let mut final_minutes = self.minutes;
        if final_minutes / 60 != 0 {
            final_minutes = if final_minutes > 0 { final_minutes % 60 } else { 60 + (final_minutes % 60) };
            change_hours = self.minutes / 60;
            if final_minutes == 60 {
                final_minutes = 0;
            }
            if self.minutes < 0 && self.minutes % 60 != 0{
                change_hours -= 1;
            }
        } else if final_minutes < 0 {
            final_minutes = 60 + final_minutes;
            change_hours -= 1;
        }
        Clock { hours: self.hours + change_hours, minutes: final_minutes }
    }

    fn normalize_hours(&self) -> Self {
        let mut final_hours = self.hours;
        if final_hours / 24 != 0 {
            final_hours = if final_hours > 0 { final_hours % 24 } else { 24 + (final_hours % 24) };
            if final_hours == 24 {
                final_hours = 0;
            }
        } else if final_hours < 0 {
            final_hours = 24 + final_hours;
        }
        Clock { hours: final_hours, minutes: self.minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)   
    }
}
