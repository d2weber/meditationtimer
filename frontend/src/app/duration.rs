type Seconds = u32;
type Minutes = u32;

#[derive(Clone, Copy)]
pub struct Duration {
    pub seconds: Seconds,
}

impl Duration {
    pub fn new() -> Duration {
        Duration { seconds: 0 }
    }

    pub fn from_parts(mins: Minutes, secs: Seconds) -> Duration {
        Duration {
            seconds: mins * 60 + secs,
        }
    }

    pub fn from_secs(seconds: Seconds) -> Duration {
        Duration { seconds }
    }

    pub fn mins_part(&self) -> Minutes {
        self.seconds / 60
    }

    pub fn secs_part(&self) -> Seconds {
        self.seconds % 60
    }

    pub fn mins_secs(&self) -> (Minutes, Seconds) {
        (self.mins_part(), self.secs_part())
    }

    pub fn update_mins_part(&mut self, new_mins: Minutes) {
        *self = Duration::from_parts(new_mins, self.secs_part())
    }

    pub fn update_secs_part(&mut self, new_secs: Seconds) {
        *self = Duration::from_parts(self.mins_part(), new_secs);
    }
}
