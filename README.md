<h1 align="center">Cron-sched</h1>

[![Rust](https://github.com/isqad/cron-sched/actions/workflows/rust.yml/badge.svg)](https://github.com/isqad/cron-sched/actions/workflows/rust.yml)

Miniature crond

⚠️ Please do not use in your projects

## Usage

Sample code:

```rust
use cron_sched::{Cron, CronJob};

struct GreetingJob {}

impl CronJob for GreetingJob {
    fn run(&mut self) {
        println!("GreetingJob: Hello, world!")
    }
}

struct CounterJob {
    i: u64
}

impl CronJob for CounterJob {
    fn run(&mut self) {
        self.i += 1;
        println!("CounterJob: i = {}", self.i);
    }
}

fn main() {
    let mut cron = Cron::new();
    let mut greeting_job = GreetingJob {};
    let mut counter_job = CounterJob { i: 0 };

    cron.add("*/3 * * * * * *", &mut greeting_job).unwrap();
    cron.add("*/1 * * * * * *", &mut counter_job).unwrap();

    let waiting = std::time::Duration::from_millis(500u64);
    loop {
        cron.tick().unwrap();
        std::thread::sleep(waiting);
    }
}
```

See more [examples](examples) directory.

## Roadmap

- [x] Common features
- [ ] Run method instead custom loop
- [ ] Run cron jobs in separated threads
- [ ] Add logging
- [ ] make it compatible with embedded systems (stm32)
- [ ] tokio runtime
