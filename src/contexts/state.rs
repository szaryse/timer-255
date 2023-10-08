#[derive(PartialEq, Clone)]
pub enum Activity {
    Break,
    Session,
}

#[derive(PartialEq, Clone)]
pub struct ActivityTime {
    pub activity_name: String,
    pub set_time: u32,
    pub activity_type: Activity,
}

#[derive(PartialEq, Clone)]
pub struct Timer {
    pub is_counting: bool,
    pub idx: usize,
    pub show_set_time: bool,
    pub is_pausing: bool,
}

pub enum TimerAction {
    DecreaseBreakTime,
    IncreaseBreakTime,
    DecreaseSessionTime,
    IncreaseSessionTime,
    StartTime,
    PauseTime,
    ResetTime,
    NextActivity,
}

#[derive(PartialEq, Clone)]
pub struct TimerState {
    pub activity_type: Activity,
    pub count: u32,
    pub is_counting: bool,
    pub is_pausing: bool,
    /* config */
    pub break_time: ActivityTime,
    pub session_time: ActivityTime,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            activity_type: Activity::Session,
            count: 25 * 60,
            is_counting: false,
            is_pausing: false,
            /* config */
            break_time: ActivityTime {
                activity_name: "Break".to_string(),
                set_time: 5,
                activity_type: Activity::Break,
            },
            session_time: ActivityTime {
                activity_name: "Session".to_string(),
                set_time: 25,
                activity_type: Activity::Session,
            },
        }
    }

    pub fn reduce(&mut self, action: TimerAction) {
        match action {
            TimerAction::DecreaseBreakTime => {
                if (2..=60).contains(&self.break_time.set_time) {
                    self.break_time.set_time -= 1;
                }
                if !self.is_counting {
                    self.count = self.break_time.set_time * 60;

                    if self.activity_type == Activity::Session {
                        self.activity_type = Activity::Break;
                    }
                }
            }
            TimerAction::IncreaseBreakTime => {
                if (0..=59).contains(&self.break_time.set_time) {
                    self.break_time.set_time += 1;
                }
                if !self.is_counting {
                    self.count = self.break_time.set_time * 60;

                    if self.activity_type == Activity::Session {
                        self.activity_type = Activity::Break;
                    }
                }
            }
            TimerAction::DecreaseSessionTime => {
                if (2..=60).contains(&self.session_time.set_time) {
                    self.session_time.set_time -= 1;
                }
                if !self.is_counting {
                    self.count = self.session_time.set_time * 60;

                    if self.activity_type == Activity::Break {
                        self.activity_type = Activity::Session;
                    }
                }
            }
            TimerAction::IncreaseSessionTime => {
                if (0..=59).contains(&self.session_time.set_time) {
                    self.session_time.set_time += 1;
                }
                if !self.is_counting {
                    self.count = self.session_time.set_time * 60;

                    if self.activity_type == Activity::Break {
                        self.activity_type = Activity::Session;
                    }
                }
            }
            TimerAction::StartTime => {
                if !self.is_counting {
                    self.is_counting = true;
                    self.is_pausing = false;
                }
            }
            TimerAction::PauseTime => {
                self.is_counting = false;
                self.is_pausing = true;
            }
            TimerAction::ResetTime => {
                self.is_counting = false;
                self.is_pausing = false;
                if self.activity_type == Activity::Session {
                    self.count = self.session_time.set_time * 60;
                }
                if self.activity_type == Activity::Break {
                    self.count = self.break_time.set_time * 60;
                }
            }
            TimerAction::NextActivity => match self.activity_type {
                Activity::Session => {
                    self.activity_type = Activity::Break;
                    self.count = self.break_time.set_time * 60;
                }
                Activity::Break => {
                    self.activity_type = Activity::Session;
                    self.count = self.session_time.set_time * 60;
                }
            },
        }
    }

    pub fn tick(&mut self) {
        self.count -= 1;
    }

    pub fn select_label(&self) -> String {
        match self.activity_type {
            Activity::Break => self.break_time.activity_name.clone(),
            Activity::Session => self.session_time.activity_name.clone(),
        }
    }

    pub fn select_time_value(&self, activity_type: Activity) -> u32 {
        match activity_type {
            Activity::Break => self.break_time.set_time,
            Activity::Session => self.session_time.set_time,
        }
    }
}
