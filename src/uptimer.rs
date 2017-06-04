use time::{ Tm, now_utc };
use num_integer::Integer;

pub struct Uptimer {
    started_at: Tm
}

impl Uptimer {
    pub fn new() -> Uptimer {
        Uptimer {
            started_at: now_utc()
        }
    }
    pub fn uptime_string(&self) -> String {
        let seconds = (now_utc() - self.started_at).num_seconds();
        let (minutes, seconds) = seconds.div_rem(&60);
        let (hours, minutes) = minutes.div_rem(&60);
        let (days, hours) = hours.div_rem(&24); 
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    }
}