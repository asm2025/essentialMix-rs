mod cond;
pub use crate::cond::*;
pub mod constants;
pub mod consumer;
mod signal;
pub use self::signal::*;
mod spinner;
pub use self::spinner::*;

use futures::Future;
use std::{fmt, pin::Pin, sync::Arc, thread};
use tokio::{
    sync::Notify,
    time::{self, Duration},
};

pub use emixcore::{Error, Result};

#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum TaskResult {
    #[default]
    None,
    Cancelled,
    TimedOut,
    Error(String),
    Success,
}

impl fmt::Display for TaskResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskResult::Cancelled => write!(f, "Cancelled"),
            TaskResult::TimedOut => write!(f, "Timedout"),
            TaskResult::Error(e) => write!(f, "Error: {}", e),
            TaskResult::Success => write!(f, "Success"),
            _ => Ok(()),
        }
    }
}

#[derive(Default, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QueueBehavior {
    #[default]
    FIFO,
    LIFO,
}

impl fmt::Display for QueueBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueueBehavior::FIFO => write!(f, "FIFO"),
            QueueBehavior::LIFO => write!(f, "LIFO"),
        }
    }
}

pub trait TaskItem: Clone + Send + Sync + fmt::Debug {}
impl<T: Clone + Send + Sync + fmt::Debug> TaskItem for T {}

pub trait StaticTaskItem: TaskItem + 'static {}
impl<T: TaskItem + 'static> StaticTaskItem for T {}

pub trait TaskDelegation<TPC: AwaitableConsumer<T>, T: StaticTaskItem>: StaticTaskItem {
    fn on_started(&self, pc: &TPC);
    fn process(&self, pc: &TPC, item: &T) -> Result<TaskResult>;
    fn on_completed(&self, pc: &TPC, item: &T, result: &TaskResult) -> bool;
    fn on_cancelled(&self, pc: &TPC);
    fn on_finished(&self, pc: &TPC);
}

pub trait AwaitableConsumer<T: TaskItem>: StaticTaskItem {
    fn is_cancelled(&self) -> bool;
    fn is_finished(&self) -> bool;
}

pub fn wait<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    finished: &Arc<ManualResetCond>,
) -> Result<()> {
    match finished.wait_while(|| !this.is_cancelled() && !this.is_finished()) {
        Ok(_) => {
            if this.is_cancelled() {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn wait_async<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    finished: &Arc<Notify>,
) -> Result<()> {
    let mut notified = false;

    while !notified && !this.is_finished() && !this.is_cancelled() {
        thread::sleep(Duration::ZERO);
        finished.notified().await;
        notified = true;
    }

    if this.is_cancelled() {
        return Err(Error::Canceled);
    }

    Ok(())
}

pub fn wait_until<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    finished: &Arc<ManualResetCond>,
    cond: impl Fn(&TPC) -> bool,
) -> Result<()> {
    match finished.wait_while(|| !this.is_cancelled() && !this.is_finished() && !cond(this)) {
        Ok(_) => {
            if this.is_cancelled() {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn wait_until_async<
    TPC: AwaitableConsumer<T>,
    T: StaticTaskItem,
    F: Fn(&TPC) -> Pin<Box<dyn Future<Output = bool> + Send>>,
>(
    this: &TPC,
    finished: &Arc<Notify>,
    cond: F,
) -> Result<()> {
    let mut notified = false;

    while !cond(this).await && !notified && !this.is_cancelled() && !this.is_finished() {
        finished.notified().await;
        notified = true;
    }

    if this.is_cancelled() {
        return Err(Error::Canceled);
    }

    Ok(())
}

pub fn wait_for<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    timeout: Duration,
    finished: &Arc<ManualResetCond>,
) -> Result<()> {
    if timeout.is_zero() {
        return Err(Error::Timeout);
    }

    match finished.wait_timeout_while(|| !this.is_cancelled() && !this.is_finished(), timeout) {
        Ok(true) => {
            if this.is_cancelled() {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        }
        Ok(false) => Err(Error::Timeout),
        Err(e) => {
            // Preserve Poisoned errors, but treat other errors as timeout
            match e {
                Error::Poisoned(_) => Err(e),
                _ => Err(Error::Timeout),
            }
        }
    }
}

pub async fn wait_for_async<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    timeout: Duration,
    finished: &Arc<Notify>,
) -> Result<()> {
    if timeout.is_zero() {
        return Err(Error::Timeout);
    }

    let result = time::timeout(timeout, finished.notified()).await;
    match result {
        Ok(_) => {
            if this.is_cancelled() {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        }
        Err(_) => Err(Error::Timeout),
    }
}

pub fn wait_for_until<TPC: AwaitableConsumer<T>, T: StaticTaskItem>(
    this: &TPC,
    timeout: Duration,
    finished: &Arc<ManualResetCond>,
    cond: impl Fn(&TPC) -> bool,
) -> Result<()> {
    if timeout.is_zero() {
        return Err(Error::Timeout);
    }
    match finished.wait_timeout_while(
        || !this.is_cancelled() && !this.is_finished() && !cond(this),
        timeout,
    ) {
        Ok(true) => {
            if this.is_cancelled() {
                Err(Error::Canceled)
            } else {
                Ok(())
            }
        }
        Ok(false) => Err(Error::Timeout),
        Err(e) => {
            // Preserve Poisoned errors, but treat other errors as timeout
            match e {
                Error::Poisoned(_) => Err(e),
                _ => Err(Error::Timeout),
            }
        }
    }
}

pub async fn wait_for_until_async<
    TPC: AwaitableConsumer<T>,
    T: StaticTaskItem,
    F: Fn(&TPC) -> Pin<Box<dyn Future<Output = bool> + Send>>,
>(
    this: &TPC,
    timeout: Duration,
    finished: &Arc<Notify>,
    cond: F,
) -> Result<()> {
    if timeout.is_zero() {
        return Err(Error::Timeout);
    }

    let start = time::Instant::now();

    while !cond(this).await {
        if this.is_cancelled() {
            return Err(Error::Canceled);
        }

        if time::Instant::now().duration_since(start) > timeout {
            return Err(Error::Timeout);
        }

        match time::timeout(constants::PEEK_TIMEOUT_DEF, finished.notified()).await {
            Ok(_) => {
                if this.is_cancelled() {
                    return Err(Error::Canceled);
                }

                return Ok(());
            }
            Err(_) => {
                if time::Instant::now().duration_since(start) > timeout {
                    return Err(Error::Timeout);
                }
            }
        }
    }

    Ok(())
}
