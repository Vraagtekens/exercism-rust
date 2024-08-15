#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        let additional_hours = minutes.div_euclid(60);
        minutes = minutes.rem_euclid(60);
        hours += additional_hours;
        hours = hours.rem_euclid(24);

        hours = match hours {
            24 => 0,
            0..=23 => hours,
            _ => hours % 24,
        };

        Clock{
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {

        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.hours, self.minutes)
//     }
// }