# Cron Scheduler ⏰

Distributed cron scheduler with leader election.

## Features

- **Cron Expressions**: Full POSIX syntax
- **Leader Election**: Prevent duplicate runs
- **Persistence**: WAL-based state
- **Distributed Lock**: Redis/etcd backend

## Quick Start

```rust
let scheduler = Scheduler::new();
scheduler.add_job("0 */5 * * * *", || {
    println!("Running every 5 minutes");
});
scheduler.start().await;
```

## License

MIT