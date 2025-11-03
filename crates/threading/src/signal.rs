use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug, Default, Clone)]
pub struct Signal {
    inner: Arc<(Mutex<bool>, Condvar)>,
}

impl Signal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&self) {
        let (lock, cvar) = &*self.inner;
        let mut signaled = lock.lock().unwrap();
        *signaled = true;
        cvar.notify_all();
    }

    pub fn reset(&self) {
        let (lock, _) = &*self.inner;
        let mut signaled = lock.lock().unwrap();
        *signaled = false;
    }

    pub fn wait(&self) {
        let (lock, cvar) = &*self.inner;
        let mut signaled = lock.lock().unwrap();

        while !*signaled {
            signaled = cvar.wait(signaled).unwrap();
        }

        // Reset for the next use
        *signaled = false;
    }

    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        if timeout.is_zero() {
            self.wait();
            return true;
        }

        let (lock, cvar) = &*self.inner;
        let mut signaled = lock.lock().unwrap();

        let start = Instant::now();

        while !*signaled {
            let elapsed = start.elapsed();
            let Some(remaining) = timeout.checked_sub(elapsed) else {
                return false;
            };
            signaled = cvar.wait_timeout(signaled, remaining).unwrap().0;
        }

        *signaled = false;
        true
    }
}
