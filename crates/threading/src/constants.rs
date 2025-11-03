use std::time::Duration;

use crate::QueueBehavior;

pub const CAPACITY_DEF: usize = 0;
pub const THREADS_DEF: usize = 1;
pub const THREADS_MIN: usize = 1;
pub const THREADS_MAX: usize = 255;
pub const QUEUE_BEHAVIOR_DEF: QueueBehavior = QueueBehavior::FIFO;
pub const THRESHOLD_DEF: Duration = Duration::ZERO;
pub const SLEEP_AFTER_SEND_DEF: Duration = Duration::ZERO;
pub const PEEK_TIMEOUT_DEF: Duration = Duration::from_millis(50);
pub const PEEK_TIMEOUT_MIN: Duration = Duration::from_millis(10);
pub const PEEK_TIMEOUT_MAX: Duration = Duration::from_secs(5);
pub const PAUSE_TIMEOUT_DEF: Duration = Duration::from_millis(50);
pub const PAUSE_TIMEOUT_MIN: Duration = Duration::from_millis(10);
pub const PAUSE_TIMEOUT_MAX: Duration = Duration::from_secs(5);
pub const INTERVAL: u64 = 100;
