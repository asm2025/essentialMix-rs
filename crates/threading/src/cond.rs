use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct AutoResetCond {
    pair: Arc<(Mutex<bool>, Condvar)>,
}

impl AutoResetCond {
    /// Creates a new AutoResetCond with the specified initial state.
    /// If `initial_state` is true, the event is initially signaled.
    pub fn new(initial_state: bool) -> Self {
        Self {
            pair: Arc::new((Mutex::new(initial_state), Condvar::new())),
        }
    }

    /// Creates a new AutoResetCond in the unset state.
    pub fn new_unset() -> Self {
        Self::new(false)
    }

    /// Creates a new AutoResetCond in the set state.
    pub fn new_set() -> Self {
        Self::new(true)
    }

    /// Sets the event, releasing one waiting thread. The event automatically
    /// resets after one thread has been released.
    pub fn set(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        if !*guard {
            *guard = true;
            cvar.notify_one();
        }

        Ok(())
    }

    /// Resets the event to the non-signaled state.
    pub fn reset(&self) -> Result<()> {
        let (lock, _) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;
        *guard = false;
        Ok(())
    }

    /// Waits for the event to be set. When the event is set, this thread is
    /// released and the event is automatically reset.
    pub fn wait(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        // Wait until the event is set
        while !*guard {
            guard = Error::handle_poison_error(cvar.wait(guard))?;
        }

        // Auto-reset: consume the signal
        *guard = false;
        Ok(())
    }

    /// Waits for the event to be set, with a timeout.
    /// Returns Ok(true) if the event was set, Ok(false) if the timeout expired.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        // Check if already set before waiting
        if *guard {
            *guard = false; // Auto-reset
            return Ok(true);
        }

        let (mut new_guard, result) =
            Error::handle_poison_error(cvar.wait_timeout(guard, timeout))?;

        if result.timed_out() {
            return Ok(false);
        }

        // Auto-reset: consume the signal if it was set
        if *new_guard {
            *new_guard = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Waits for the event to be set, with a timeout in milliseconds.
    pub fn wait_timeout_ms(&self, timeout_ms: u64) -> Result<bool> {
        self.wait_timeout(Duration::from_millis(timeout_ms))
    }

    /// Gets whether the event is currently in the signaled state.
    pub fn is_set(&self) -> Result<bool> {
        let (lock, _) = &*self.pair;
        let guard = Error::handle_poison_error(lock.lock())?;
        Ok(*guard)
    }

    /// Waits while the condition is true. When the condition becomes false, returns.
    pub fn wait_while(&self, condition: impl Fn() -> bool) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        while condition() {
            guard = Error::handle_poison_error(cvar.wait(guard))?;
        }

        Ok(())
    }

    /// Waits while the condition is true, with a timeout.
    /// Returns Ok(true) if the condition became false, Ok(false) if the timeout expired.
    pub fn wait_timeout_while(
        &self,
        condition: impl Fn() -> bool,
        timeout: Duration,
    ) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;
        let start = std::time::Instant::now();

        while condition() {
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Ok(false);
            }
            let remaining = timeout - elapsed;
            let (new_guard, result) =
                Error::handle_poison_error(cvar.wait_timeout(guard, remaining))?;
            guard = new_guard;
            if result.timed_out() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[derive(Clone, Debug)]
pub struct ManualResetCond {
    pair: Arc<(Mutex<bool>, Condvar)>,
}

impl ManualResetCond {
    /// Creates a new ManualResetCond with the specified initial state.
    /// If `initial_state` is true, the event is initially signaled.
    pub fn new(initial_state: bool) -> Self {
        Self {
            pair: Arc::new((Mutex::new(initial_state), Condvar::new())),
        }
    }

    /// Creates a new ManualResetCond in the unset state.
    pub fn new_unset() -> Self {
        Self::new(false)
    }

    /// Creates a new ManualResetCond in the set state.
    pub fn new_set() -> Self {
        Self::new(true)
    }

    /// Sets the event, releasing all waiting threads. The event remains set
    /// until manually reset.
    pub fn set(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        if *guard {
            return Ok(()); // Already set
        }

        *guard = true;
        cvar.notify_all();
        Ok(())
    }

    /// Resets the event to the non-signaled state.
    pub fn reset(&self) -> Result<()> {
        let (lock, _) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;
        *guard = false;
        Ok(())
    }

    /// Waits for the event to be set. Unlike AutoResetCond, the event does
    /// not automatically reset after a thread is released.
    pub fn wait(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        // Wait until the event is set
        while !*guard {
            guard = Error::handle_poison_error(cvar.wait(guard))?;
        }

        // Don't reset - ManualResetEvent stays set
        Ok(())
    }

    /// Waits for the event to be set, with a timeout.
    /// Returns Ok(true) if the event was set, Ok(false) if the timeout expired.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let guard = Error::handle_poison_error(lock.lock())?;

        // Check if already set before waiting
        if *guard {
            return Ok(true);
        }

        let (new_guard, result) = Error::handle_poison_error(cvar.wait_timeout(guard, timeout))?;

        if result.timed_out() {
            return Ok(false);
        }

        // Check if event was set
        Ok(*new_guard)
    }

    /// Waits for the event to be set, with a timeout in milliseconds.
    pub fn wait_timeout_ms(&self, timeout_ms: u64) -> Result<bool> {
        self.wait_timeout(Duration::from_millis(timeout_ms))
    }

    /// Gets whether the event is currently in the signaled state.
    pub fn is_set(&self) -> Result<bool> {
        let (lock, _) = &*self.pair;
        let guard = Error::handle_poison_error(lock.lock())?;
        Ok(*guard)
    }

    /// Waits while the condition is true. When the condition becomes false, returns.
    pub fn wait_while(&self, condition: impl Fn() -> bool) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        while condition() {
            guard = Error::handle_poison_error(cvar.wait(guard))?;
        }

        Ok(())
    }

    /// Waits while the condition is true, with a timeout.
    /// Returns Ok(true) if the condition became false, Ok(false) if the timeout expired.
    pub fn wait_timeout_while(
        &self,
        condition: impl Fn() -> bool,
        timeout: Duration,
    ) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;
        let start = std::time::Instant::now();

        while condition() {
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Ok(false);
            }
            let remaining = timeout - elapsed;
            let (new_guard, result) =
                Error::handle_poison_error(cvar.wait_timeout(guard, remaining))?;
            guard = new_guard;
            if result.timed_out() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

#[derive(Clone, Debug)]
pub struct CountdownCond {
    pair: Arc<(Mutex<CountdownInner>, Condvar)>,
    initial_count: usize,
}

#[derive(Debug)]
struct CountdownInner {
    count: usize,
}

impl CountdownCond {
    /// Creates a new CountdownCond with the specified initial count.
    /// The event becomes signaled when the count reaches zero.
    pub fn new(initial_count: usize) -> Self {
        Self {
            initial_count,
            pair: Arc::new((
                Mutex::new(CountdownInner {
                    count: initial_count,
                }),
                Condvar::new(),
            )),
        }
    }

    /// Gets the current remaining count.
    pub fn current_count(&self) -> Result<usize> {
        let (lock, _) = &*self.pair;
        let guard = Error::handle_poison_error(lock.lock())?;
        Ok(guard.count)
    }

    /// Gets the initial count that was specified when the CountdownCond was created.
    pub fn initial_count(&self) -> usize {
        self.initial_count
    }

    /// Signals the event, decrementing the count by one.
    /// Returns the new count value.
    pub fn signal(&self) -> Result<usize> {
        self.signal_n(1)
    }

    /// Signals the event, decrementing the count by the specified amount.
    /// Returns the new count value.
    pub fn signal_n(&self, signal_count: usize) -> Result<usize> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        if guard.count == 0 {
            return Ok(0); // Already at zero, can't go negative
        }

        if signal_count >= guard.count {
            guard.count = 0;
        } else {
            guard.count -= signal_count;
        }

        // If count reached zero, notify all waiting threads
        if guard.count == 0 {
            cvar.notify_all();
        }

        Ok(guard.count)
    }

    /// Increments the current count by the specified amount.
    pub fn add_count(&self, increment: usize) -> Result<usize> {
        let (lock, _) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;
        guard.count += increment;
        Ok(guard.count)
    }

    /// Attempts to increment the current count by the specified amount.
    /// Returns Ok(Some(new_count)) if successful, Ok(None) if the count is already zero.
    pub fn try_add_count(&self, increment: usize) -> Result<Option<usize>> {
        let (lock, _) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        if guard.count == 0 {
            return Ok(None); // Can't add to zero count
        }

        guard.count += increment;
        Ok(Some(guard.count))
    }

    /// Waits until the count reaches zero.
    pub fn wait(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = Error::handle_poison_error(lock.lock())?;

        // Wait until count reaches zero
        while guard.count > 0 {
            guard = Error::handle_poison_error(cvar.wait(guard))?;
        }

        Ok(())
    }

    /// Waits until the count reaches zero, with a timeout.
    /// Returns Ok(true) if the count reached zero, Ok(false) if the timeout expired.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let guard = Error::handle_poison_error(lock.lock())?;

        // Check if already at zero before waiting
        if guard.count == 0 {
            return Ok(true);
        }

        let (new_guard, result) = Error::handle_poison_error(cvar.wait_timeout(guard, timeout))?;

        if result.timed_out() {
            return Ok(false);
        }

        // Check if count reached zero
        Ok(new_guard.count == 0)
    }

    /// Waits until the count reaches zero, with a timeout in milliseconds.
    pub fn wait_timeout_ms(&self, timeout_ms: u64) -> Result<bool> {
        self.wait_timeout(Duration::from_millis(timeout_ms))
    }

    /// Gets whether the count has reached zero (event is signaled).
    pub fn is_set(&self) -> Result<bool> {
        Ok(self.current_count()? == 0)
    }
}
