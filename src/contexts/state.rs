#[derive(Clone)]
pub struct ActivityTime {
    pub activity_name: String,
    // default_time: u32,
    pub set_time: u32,
}

impl ActivityTime {
    pub fn decrease(&mut self) {
        if (1..=60).contains(&self.set_time) {
            self.set_time -= 1;
        }
    }
    pub fn increase(&mut self) {
        if (0..=59).contains(&self.set_time) {
            self.set_time += 1;
        }
    }
}

pub fn create_initial_times() -> Vec<ActivityTime> {
    let break_time = ActivityTime {
        activity_name: "Break".to_string(),
        set_time: 5,
    };
    let session_time = ActivityTime {
        activity_name: "Session".to_string(),
        set_time: 25,
    };

    vec![break_time, session_time]
}

#[derive(PartialEq, Clone)]
pub struct Timer {
    pub is_counting: bool,
    pub idx: usize,
    pub show_set_time: bool,
    pub is_pausing: bool,
}
