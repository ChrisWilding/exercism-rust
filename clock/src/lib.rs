use std::fmt;

const MINUTES_IN_DAY: i32 = 1440;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(0).add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self((self.0 + minutes).rem_euclid(MINUTES_IN_DAY))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.0 / 60;
        let minutes = self.0 % 60;
        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}
