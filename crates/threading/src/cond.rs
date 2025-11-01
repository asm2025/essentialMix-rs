use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct Mutcond {
    pair: Arc<(Mutex<bool>, Condvar)>,
}

impl Mutcond {
    pub fn new() -> Self {
        Self {
            pair: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn is_signaled(&self) -> Result<bool> {
        let (lock, _) = &*self.pair;
        let guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        Ok(*guard)
    }

    pub fn notify_one(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        *guard = true;
        cvar.notify_one();
        Ok(())
    }

    pub fn notify_all(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        *guard = true;
        cvar.notify_all();
        Ok(())
    }

    pub fn wait(&self) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };

        while !*guard {
            match cvar.wait(guard) {
                Ok(g) => guard = g,
                Err(e) => {
                    let error_msg = format!("{}", e);
                    let _poisoned_guard = e.into_inner();
                    return Err(Error::Poisoned(if error_msg.is_empty() {
                        "Poison error".to_string()
                    } else {
                        error_msg
                    }));
                }
            }
        }

        *guard = false;
        Ok(())
    }

    pub fn wait_timeout(&self, timeout: Duration) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        let (new_guard, result) = match cvar.wait_timeout(guard, timeout) {
            Ok(tup) => tup,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        guard = new_guard;

        if result.timed_out() {
            return Ok(false);
        }

        *guard = false;
        Ok(true)
    }

    pub fn wait_timeout_ms(&self, timeout: u64) -> Result<bool> {
        self.wait_timeout(Duration::from_millis(timeout))
    }

    pub fn wait_while(&self, condition: impl Fn() -> bool) -> Result<()> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };

        while condition() {
            match cvar.wait(guard) {
                Ok(g) => guard = g,
                Err(e) => {
                    let error_msg = format!("{}", e);
                    let _poisoned_guard = e.into_inner();
                    return Err(Error::Poisoned(if error_msg.is_empty() {
                        "Poison error".to_string()
                    } else {
                        error_msg
                    }));
                }
            }
        }

        Ok(())
    }

    pub fn wait_timeout_while(
        &self,
        condition: impl Fn() -> bool,
        timeout: Duration,
    ) -> Result<bool> {
        let (lock, cvar) = &*self.pair;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner();
                return Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }));
            }
        };
        let start = Instant::now();

        while condition() {
            let remaining = timeout.checked_sub(start.elapsed());
            match remaining {
                Some(time) => match cvar.wait_timeout(guard, time) {
                    Ok((g, result)) => {
                        guard = g;
                        if result.timed_out() {
                            return Ok(false);
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("{}", e);
                        let _poisoned_guard = e.into_inner();
                        return Err(Error::Poisoned(if error_msg.is_empty() {
                            "Poison error".to_string()
                        } else {
                            error_msg
                        }));
                    }
                },
                None => return Ok(false),
            }
        }

        Ok(true)
    }
}
