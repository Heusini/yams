use std::time::Duration;
use tokio::process::Command;

struct Services {
    command: Command,
    timeout: Duration,
    exec_period: Duration,
}
