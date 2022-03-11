#[cfg(all(feature = "localtime"))]
use chrono::{DateTime, Duration, Local, Utc};
#[cfg(not(feature = "localtime"))]
use chrono::{DateTime, Duration, Utc};
use std::{collections::HashMap, fmt};

const MARK: &'static str = "maxi_el_amor_de_mi_vida";

#[derive(Debug)]
pub enum TicTocError {
    TimerAlreadyExists,
    TimerNotExists,
    TimerUpdateError,
    TimerResultError,
}

impl fmt::Display for TicTocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Self::TimerAlreadyExists => write!(f, "A timer with default key mark already exists."),
            &Self::TimerNotExists => write!(f, "Timer does not exists."),
            &Self::TimerUpdateError => write!(f, "Error updating timer."),
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
    #[cfg(any(feature = "localtime"))]
    local_start: DateTime<Local>,
    #[cfg(any(feature = "localtime"))]
    local_end: Option<DateTime<Local>>,
    #[cfg(any(feature = "localtime"))]
    local_diff: Option<Duration>,
}

impl Timer {
    #[cfg(not(feature = "localtime"))]
    fn new() -> Self {
        Self {
            start: Utc::now(),
            end: None,
            diff: None,
        }
    }

    #[cfg(any(feature = "localtime"))]
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

    #[cfg(not(feature = "localtime"))]
    fn _finish(&self) -> Self {
        Self {
            end: Some(Utc::now()),
            ..*self
        }
    }

    #[cfg(any(feature = "localtime"))]
    fn _finish(&self) -> Self {
        Self {
            end: Some(Utc::now()),
            local_end: Some(Local::now()),
            ..*self
        }
    }

    #[cfg(not(feature = "localtime"))]
    fn _diff(&self) -> Self {
        Self {
            diff: Some(self.start.time() - self.end.unwrap().time()),
            ..*self
        }
    }

    #[cfg(any(feature = "localtime"))]
    fn _diff(&self) -> Self {
        Self {
            diff: Some(self.start.time() - self.end.unwrap().time()),
            local_diff: Some(self.local_start.time() - self.local_end.unwrap().time()),
            ..*self
        }
    }

    #[cfg(not(feature = "localtime"))]
    fn finish(self) -> Self {
        self._finish()._diff()
    }

    #[cfg(any(feature = "localtime"))]
    fn finish(self) -> Self {
        self._finish()._diff()
    }

    fn get_duration(&self) -> Duration {
        self.diff.unwrap()
    }
    #[cfg(any(feature = "localtime"))]
    fn get_local_duration(&self) -> Duration {
        self.local_diff.unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct TicToc<'a> {
    timers: HashMap<&'a str, Timer>,
}

impl<'a> TicToc<'a> {
    pub fn new() -> Self {
        Self {
            timers: HashMap::default(),
        }
    }

    fn _get_timer(&self, key: Option<&'a str>) -> Result<Timer, TicTocError> {
        let key = key.unwrap_or(MARK);
        match self.timers.get(key) {
            Some(t) => Ok(*t),
            None => Err(TicTocError::TimerNotExists),
        }
    }

    fn _update_timer(&mut self, key: Option<&'a str>, timer: Timer) -> Result<(), TicTocError> {
        let key = key.unwrap_or(MARK);
        match self.timers.insert(key, timer) {
            Some(_) => Ok(()),
            None => Err(TicTocError::TimerUpdateError),
        }
    }

    #[cfg(not(feature = "localtime"))]
    pub fn tic(&mut self, mark: Option<&'a str>) -> Result<DateTime<Utc>, TicTocError> {
        let mark: &str = match mark.is_some() {
            true => match self.timers.contains_key(mark.unwrap()) {
                false => mark.unwrap(),
                true => return Err(TicTocError::TimerAlreadyExists),
            },
            false => match self.timers.contains_key(MARK) {
                false => MARK,
                true => return Err(TicTocError::TimerAlreadyExists),
            },
        };
        let timer = Timer::new();
        let now = &timer.start;
        self.timers.insert(mark, timer);
        Ok(*now)
    }

    #[cfg(any(feature = "localtime"))]
    pub fn tic(&mut self, mark: Option<&'a str>) -> Result<DateTime<Local>, TicTocError> {
        let mark: &str = match mark.is_some() {
            true => match self.timers.contains_key(mark.unwrap()) {
                false => mark.unwrap(),
                true => return Err(TicTocError::TimerAlreadyExists),
            },
            false => match self.timers.contains_key(MARK) {
                false => MARK,
                true => return Err(TicTocError::TimerAlreadyExists),
            },
        };
        let timer = Timer::new();
        let now = &timer.local_start;
        self.timers.insert(mark, timer);
        Ok(*now)
    }

    #[cfg(not(feature = "localtime"))]
    pub fn toc(&mut self, mark: Option<&'a str>) -> Result<DateTime<Utc>, TicTocError> {
        let timer = self._get_timer(mark)?.finish();
        let now = &timer.end;
        self._update_timer(mark, timer)?;
        match now {
            Some(n) => Ok(n.to_owned()),
            None => Err(TicTocError::TimerUpdateError),
        }
    }

    #[cfg(any(feature = "localtime"))]
    pub fn toc(&mut self, mark: Option<&'a str>) -> Result<DateTime<Local>, TicTocError> {
        let timer = self._get_timer(mark)?.finish();
        let now = &timer.local_end;
        self._update_timer(mark, timer)?;
        match now {
            Some(n) => Ok(n.to_owned()),
            None => Err(TicTocError::TimerUpdateError),
        }
    }

    pub fn time(
        &mut self,
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

    #[cfg(any(feature = "localtime"))]
    pub fn local_time(
        &mut self,
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
    use super::*;
    #[cfg(all(feature = "localtime"))]
    use chrono::{DateTime, Local};
    #[cfg(not(feature = "localtime"))]
    use chrono::{DateTime, Utc};

    #[test]
    fn default_timer_works() {
        let mut tictoc = TicToc::new();
        assert!(tictoc.tic(None).is_ok());
    }

    #[test]
    fn named_timer_works() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        assert!(tictoc.tic(mark).is_ok());
    }

    #[test]
    fn multiple_named_timers_works() {
        let mark = Some("test1");
        let mark2 = Some("test2");
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(mark).unwrap();
        assert!(tictoc.tic(mark2).is_ok());
    }

    #[test]
    fn timer_does_exists_works() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(mark).unwrap();
        assert!(tictoc.toc(mark).is_ok());
    }

    #[test]
    fn multiple_timer_with_default_and_named_works() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(None).unwrap();
        assert!(tictoc.tic(mark).is_ok());
    }

    #[test]
    fn result_comparison_works() {
        let mut tictoc = TicToc::new();
        let tic = tictoc.tic(None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc = tictoc.toc(None).unwrap();
        let diff = tic - toc;
        assert_eq!(diff.num_milliseconds(), tictoc.time(None, None).unwrap());
    }

    #[test]
    fn multiple_result_comparison_works() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        let tic = tictoc.tic(None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc = tictoc.toc(None).unwrap();
        let tic1 = tictoc.tic(mark).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc1 = tictoc.toc(mark).unwrap();
        let diff = tic - toc;
        let diff1 = tic1 - toc1;
        let result = tictoc.time(None, None).unwrap();
        let result1 = tictoc.time(mark, None).unwrap();
        assert_eq!(
            diff.num_milliseconds().eq(&result),
            diff1.num_milliseconds().eq(&result1)
        );
    }

    #[test]
    fn multiple_result_comparison_fail() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();

        let tic = tictoc.tic(None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc = tictoc.toc(None).unwrap();
        let diff = tic - toc;

        let _ = tictoc.tic(mark).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        #[cfg(not(feature = "localtime"))]
        let tic1: DateTime<Utc> = Utc::now();
        #[cfg(all(feature = "localtime"))]
        let tic1: DateTime<Local> = Local::now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc1 = tictoc.toc(mark).unwrap();
        let diff1 = tic1 - toc1;

        let result = tictoc.time(None, None).unwrap();
        let result1 = tictoc.time(mark, None).unwrap();
        assert_ne!(
            diff.num_milliseconds().eq(&result),
            diff1.num_milliseconds().eq(&result1)
        );
    }

    #[test]
    fn result_comparison_fail() {
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(None).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        #[cfg(not(feature = "localtime"))]
        let tic: DateTime<Utc> = Utc::now();
        #[cfg(all(feature = "localtime"))]
        let tic: DateTime<Local> = Local::now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let toc = tictoc.toc(None).unwrap();
        let diff = tic - toc;
        assert_ne!(diff.num_milliseconds(), tictoc.time(None, None).unwrap());
    }

    #[test]
    fn timer_doesnt_exists_fail() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        assert!(tictoc.time(mark, None).is_err());
    }

    #[test]
    fn multiple_default_timers_fail() {
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(None).unwrap();
        assert!(tictoc.tic(None).is_err());
    }
    #[test]
    fn timer_already_exists_fails() {
        let mark = Some("test1");
        let mut tictoc = TicToc::new();
        let _ = tictoc.tic(mark).unwrap();
        assert!(tictoc.tic(mark).is_err());
    }
}
