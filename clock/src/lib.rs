use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut tmp_hours = hours;
        let mut tmp_minutes = minutes;

        if minutes < 0 {
            tmp_minutes = 60 + (minutes % 60);
            tmp_hours = tmp_hours + ((minutes / 60) - 1);
        } else if minutes / 60 > 0 {
            tmp_minutes -= (minutes / 60) * 60;
            tmp_hours += minutes / 60;
        }

        if tmp_hours < 0 {
            tmp_hours = 24 + (hours % 24);
        } else if tmp_hours / 24 > 0 {
            tmp_hours -= (tmp_hours / 24) * 24;
        }
        
        Clock { hours: tmp_hours, minutes: tmp_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut total_minutes = self.minutes + minutes;
        if total_minutes >= 60 {
            total_minutes -= 60;
            hours += 1;
        }
        if hours >= 24 {
            hours -= 24;
        }
        Clock { hours: hours, minutes: total_minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)   
    }
}
