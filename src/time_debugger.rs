use chrono;
use chrono::{DateTime, Duration, Local};


#[allow(dead_code)]
pub struct TimeDebugger {
    last_tick: DateTime<Local>
}


#[allow(dead_code)]
impl TimeDebugger {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            last_tick: chrono::offset::Local::now()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_current_time(&self) -> DateTime<Local> {
        chrono::offset::Local::now()
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.last_tick = chrono::offset::Local::now()
    }

    #[allow(dead_code)]
    pub fn get_elapsed_time(&self) -> Duration {
        chrono::offset::Local::now() - self.last_tick
    }

    #[allow(dead_code)]
    pub fn print_elapsed_time(&self) {
        println!("Elapsed time: {:?}", self.get_elapsed_time())
    }
}