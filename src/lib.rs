use chrono::{DateTime, Utc};

pub struct Cron<'a, J> {
    entries: Vec<&'a CronEntry<'a, J>>,
}

impl<'a, J: Job> Cron<'a, J> {
    pub fn new() -> Self {
        Cron { entries: Vec::new() }
    }

    pub fn add(&mut self, entry: &'a CronEntry<J>) {
        // TODO: calculate a next_run_at field before add to the list
        self.entries.push(entry)
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }
}

pub struct CronEntry<'a, J> {
    job: &'a mut J,
    // TODO: add a schedule field (use cron crate)
    next_run_at: DateTime<Utc>,
}

impl<'a, J: Job> CronEntry<'a, J> {
    pub fn new(schedule_at: DateTime<Utc>, job: &'a mut J) -> Self {
        CronEntry {
            job: job,
            next_run_at: schedule_at,
        }
    }

    pub fn tick(&mut self, current_time: DateTime<Utc>) {
        if current_time >= self.next_run_at {
            self.job.run();

            // TODO: add schedule according which we'll change next_run_at
        }
    }
}

pub trait Job {
    fn run(&mut self);
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    struct TestJob {
        i: i8,
    }

    impl crate::Job for TestJob {
        fn run(&mut self) {
            self.i += 1;
        }
    }

    #[test]
    fn test_add() {
        let mut cron = crate::Cron::new();
        let schedule_at = Utc.ymd(2021, 10, 13).and_hms(16, 49, 30);
        let mut job = TestJob { i: 1 };
        let mut entry = crate::CronEntry::new(schedule_at, &mut job);

        cron.add(&mut entry);

        assert_eq!(cron.size(), 1);
    }

    #[test]
    fn test_cron_entry_tick() {
        let schedule_at = Utc.ymd(2021, 10, 13).and_hms(16, 49, 30);
        let execute_at = Utc.ymd(2021, 10, 13).and_hms(16, 49, 31);
        let mut job = TestJob { i: 1 };

        let mut entry = crate::CronEntry::new(schedule_at, &mut job);
        entry.tick(execute_at);

        assert_eq!(job.i, 2);
    }
}
