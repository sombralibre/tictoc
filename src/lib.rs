#[cfg(all(feature = "localtime", feature = "full"))]
use chrono::{DateTime, Duration, Local, Utc};
use chrono::{DateTime, Duration, Utc};
use std::{collections::HashMap, fmt};

const MARK: &'static str = "maxi_el_amor_de_mi_vida";
pub enum TicTocError {
    TimerTaken,
    TimerNotExists,
    TimerResultError,
}

impl fmt::Display for TicTocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::TimerTaken => write!(f, "A timer with default key mark already exists."),
            &Self::TimerNotExists => write!(f, "Timer does not exists."),
            &Self::TimerResultError => write!(f, "Cannot resolve Timer result."),
        }
    }
}

pub enum TimeUnits {
    NanoSeconds,
    MicroSeconds,
    MilliSeconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

#[derive(Debug, Clone, Copy)]
struct Timer {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    diff: Option<Duration>,
    #[cfg(any(feature = "localtime", feature = "full"))]
    local_start: DateTime<Local>,
    #[cfg(any(feature = "localtime", feature = "full"))]
    local_end: Option<DateTime<Local>>,
    #[cfg(any(feature = "localtime", feature = "full"))]
    local_diff: Option<Duration>,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: Utc::now(),
            end: None,
            diff: None,
        }
    }

    #[cfg(any(feature = "localtime", feature = "full"))]
    fn new() -> Self {
        Self {
            start: Utc::now(),
            end: None,
            diff: None,
            local_start: Local::now(),
            local_end: None,
            local_diff: None,
        }
    }

    fn _finish(&self) -> Self {
        Self {
            end: Some(Utc::now()),
            ..*self
        }
    }

    #[cfg(any(feature = "localtime", feature = "full"))]
    fn _finish(&self) -> Self {
        Self {
            end: Some(Utc::now()),
            local_end: Some(Utc::now()),
            ..*self
        }
    }

    fn _diff(&self) -> Self {
        Self {
            diff: Some(self.start.time() - self.end.unwrap().time()),
            ..*self
        }
    }

    #[cfg(any(feature = "localtime", feature = "full"))]
    fn _diff(&self) -> Self {
        Self {
            diff: Some(self.start.time() - self.end.unwrap().time()),
            local_diff: Some(self.local_start.time() - self.local_end.unwrap().time()),
            ..*self
        }
    }

    fn finish(self) -> Self {
        self._finish()._diff()
    }

    #[cfg(any(feature = "localtime", feature = "full"))]
    fn finish(self) -> Self {
        Self {
            end: Some(Utc::now()),
            local_end: Some(Local::now())..self,
        }
    }

    fn get_duration(&self) -> Duration {
        self.diff.unwrap()
    }
    #[cfg(any(feature = "localtime", feature = "full"))]
    fn get_local_duration(&self) -> Duration {
        self.local_diff.unwrap()
    }
}

#[derive(Debug)]
pub struct TicToc<'a> {
    timers: HashMap<&'a str, Timer>,
}

impl<'a> TicToc<'a> {
    pub fn new() -> Self {
        Self {
            timers: HashMap::default(),
        }
    }

    fn _get_timer(&'a mut self, key: Option<&'a str>) -> Result<&'a Timer, TicTocError> {
        let key = key.unwrap_or(MARK);
        match self.timers.get_mut(key) {
            Some(t) => Ok(t),
            None => Err(TicTocError::TimerNotExists),
        }
    }

    pub fn tic(&mut self, mark: Option<&'a str>) -> Result<(), TicTocError> {
        let mark: &str = match mark.is_some() {
            true => mark.unwrap(),
            false => match self.timers.contains_key(MARK) {
                false => MARK,
                true => return Err(TicTocError::TimerTaken),
            },
        };
        self.timers.insert(mark, Timer::new());
        Ok(())
    }

    pub fn toc(&'a mut self, mark: Option<&'a str>) -> Result<(), TicTocError> {
        self._get_timer(mark)?.finish();
        Ok(())
    }

    pub fn time(
        &'a mut self,
        mark: Option<&'a str>,
        unit: Option<TimeUnits>,
    ) -> Result<i64, TicTocError> {
        let unit = unit.unwrap_or(TimeUnits::MilliSeconds);
        let duration = self._get_timer(mark)?.get_duration();
        let result: Option<i64> = match unit {
            TimeUnits::NanoSeconds => duration.num_nanoseconds(),
            TimeUnits::MicroSeconds => duration.num_microseconds(),
            TimeUnits::MilliSeconds => Some(duration.num_milliseconds()),
            TimeUnits::Seconds => Some(duration.num_seconds()),
            TimeUnits::Minutes => Some(duration.num_minutes()),
            TimeUnits::Hours => Some(duration.num_hours()),
            TimeUnits::Days => Some(duration.num_days()),
            TimeUnits::Weeks => Some(duration.num_weeks()),
        };
        match result {
            Some(r) => Ok(r),
            None => Err(TicTocError::TimerResultError),
        }
    }

    #[cfg(any(feature = "localtime", feature = "full"))]
    pub fn local_time(
        &'a mut self,
        mark: Option<&'a str>,
        unit: Option<TimeUnits>,
    ) -> Result<i64, TicTocError> {
        let unit = unit.unwrap_or(TimeUnits::MilliSeconds);
        let duration = self._get_timer(mark)?.get_local_duration();
        let result: Option<i64> = match unit {
            TimeUnits::NanoSeconds => duration.num_nanoseconds(),
            TimeUnits::MicroSeconds => duration.num_microseconds(),
            TimeUnits::MilliSeconds => Some(duration.num_milliseconds()),
            TimeUnits::Seconds => Some(duration.num_seconds()),
            TimeUnits::Minutes => Some(duration.num_minutes()),
            TimeUnits::Hours => Some(duration.num_hours()),
            TimeUnits::Days => Some(duration.num_days()),
            TimeUnits::Weeks => Some(duration.num_weeks()),
        };
        match result {
            Some(r) => Ok(r),
            None => Err(TicTocError::TimerResultError),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
