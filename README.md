<h1 align="center">Cron-sched</h1>

[![Rust](https://github.com/isqad/cron-sched/actions/workflows/rust.yml/badge.svg)](https://github.com/isqad/cron-sched/actions/workflows/rust.yml)

Miniature crond

⚠️ Please do not use in your projects

## Usage

Sample code:

```rust
use cron_sched::{Cron, Job};

struct GreetingJob {}

impl Job for GreetingJob {
    fn run(&mut self) {
        // Some job...
        println!("I'm running!")
    }
}

fn main() {
    let mut cron = Cron::<GreetingJob>::new();
    let mut job = GreetingJob {};

    // Run every 3 seconds
    cron.add("*/3 * * * * * *", &mut job).unwrap();

    // Do not make it less than 500 ms
    let waiting = std::time::Duration::from_millis(500u64);
    loop {
        cron.tick().unwrap();
        std::thread::sleep(waiting);
    }
}
```

See more [examples](examples) directory.
