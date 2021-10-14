use chrono::{DateTime, Utc};
use cron::Schedule;
use std;
use std::fmt;
use std::str::FromStr;

type CronError = Box<dyn std::error::Error>;
#[derive(Debug, Clone)]
pub struct CronEntryError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}
impl fmt::Display for CronEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}
impl std::error::Error for CronEntryError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub struct Cron<'a> {
    entries: Vec<CronEntry<'a>>,
}

impl<'a> Cron<'a> {
    pub fn new() -> Self {
        Cron { entries: Vec::new() }
    }

    pub fn add(&mut self, schedule_expr: &str, job: &'a mut dyn CronJob) -> Result<(), CronError> {
        let entry = CronEntry::new(schedule_expr, job)?;
        self.entries.push(entry);

        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), CronError> {
        for entry in &mut self.entries {
            entry.tick(Utc::now())?;
        }
        Ok(())
    }
}

struct CronEntry<'a> {
    job: &'a mut dyn CronJob,
    schedule: Schedule,
    next_run_at: DateTime<Utc>,
}

impl<'a> CronEntry<'a> {
    fn new(schedule_expr: &str, job: &'a mut dyn CronJob) -> Result<Self, CronError> {
        let schedule = Schedule::from_str(schedule_expr)?;
        let next_run_at = Self::upcoming(&schedule)?;

        Ok(CronEntry {
            job: job,
            schedule: schedule,
            next_run_at: next_run_at,
        })
    }

    fn upcoming(schedule: &Schedule) -> Result<DateTime<Utc>, CronEntryError>  {
        match schedule.upcoming(Utc).take(1).next() {
            Some(date) => Ok(date),
            None => Err(CronEntryError {
                message: format!("No upcomings from given schedule: {:?}", schedule),
                line: line!(),
                column: column!(),
            })
        }
    }

    fn tick(&mut self, current_time: DateTime<Utc>) -> Result<(), CronEntryError> {
        if current_time >= self.next_run_at {
            self.reset_next_run_at()?;
            // TODO: run in separated thread
            self.job.run();
        }
        Ok(())
    }

    fn reset_next_run_at(&mut self) -> Result<(), CronEntryError> {
        self.next_run_at = Self::upcoming(&self.schedule)?;
        Ok(())
    }
}

pub trait CronJob {
    fn run(&mut self);
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    struct TestJob {
        i: i8,
    }

    impl crate::CronJob for TestJob {
        fn run(&mut self) {
            self.i += 1;
        }
    }

    #[test]
    fn test_add() {
        let mut cron = crate::Cron::new();
        let mut job = TestJob { i: 1 };

        cron.add("0 30 9,12,15 1 Jan-Nov Mon 2021", &mut job).unwrap();

        assert_eq!(cron.entries.len(), 1);
    }

    #[test]
    fn test_cron_entry_tick() {
        let execute_at = Utc.ymd(2021, 10, 14).and_hms(16, 49, 31);
        let mut job = TestJob { i: 1 };

        let mut entry = crate::CronEntry::new(
            &execute_at.format("30 %M %H %d %b %a %Y").to_string(),
            &mut job,
        ).unwrap();
        entry.tick(execute_at).unwrap();

        assert_eq!(job.i, 2);
    }
}
