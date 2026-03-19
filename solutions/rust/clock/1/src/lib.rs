#[derive(Debug, PartialEq, Eq)]
pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = (hours + minutes / 60) % 24;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        if hours < 0 {
            hours += 24;
        }
        Clock(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.0, self.1 + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.0, self.1)
    }
}
