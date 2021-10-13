use cron_sched::{Cron, Job};

struct CrawlJob {}

impl Job for CrawlJob {
    fn run(&mut self) {
        println!("I'm running!")
    }
}

fn main() {
    let mut cron = Cron::<CrawlJob>::new();
    let mut job = CrawlJob {};

    cron.add("*/3 * * * * * *", &mut job).unwrap();

    let waiting = std::time::Duration::from_millis(500u64);
    loop {
        cron.tick().unwrap();
        std::thread::sleep(waiting);
    }
}
