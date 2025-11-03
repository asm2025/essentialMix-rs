#[cfg(test)]
mod tests {
    use emixthreading::Signal;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::thread;
    use std::time::Duration;

    // ============================================================================
    // Basic Signal Tests
    // ============================================================================

    #[test]
    fn test_signal_new() {
        let _signal = Signal::new();
        // New signal should be in unset state (wait would block)
    }

    #[test]
    fn test_signal_default() {
        let _signal = Signal::default();
        // Default signal should be in unset state
    }

    #[test]
    fn test_signal_set() {
        let signal = Arc::new(Signal::new());
        let signal_clone = signal.clone();

        // Set the signal in another thread
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            signal_clone.set();
        });

        // Wait should succeed once set
        signal.wait();
    }

    #[test]
    fn test_signal_set_idempotent() {
        let signal = Signal::new();

        // Setting multiple times should be fine
        signal.set();
        signal.set();
        signal.set();

        // Wait should succeed
        signal.wait();
    }

    #[test]
    fn test_signal_reset() {
        let signal = Signal::new();

        // Set it first
        signal.set();

        // Reset it
        signal.reset();

        // Now wait should block - test with timeout
        let result = signal.wait_timeout(Duration::from_millis(50));
        assert!(!result, "Should timeout after reset");
    }

    #[test]
    fn test_signal_set_reset_cycle() {
        let signal = Signal::new();

        for _ in 0..10 {
            signal.set();
            signal.reset();
        }

        // After reset, wait should timeout
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Should timeout after reset");
    }

    // ============================================================================
    // Wait Tests
    // ============================================================================

    #[test]
    fn test_signal_wait_blocks_until_set() {
        let signal = Arc::new(Signal::new());
        let signal_clone = signal.clone();
        let woke = Arc::new(AtomicUsize::new(0));
        let woke_clone = woke.clone();

        // Spawn thread that will set signal after delay
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            signal_clone.set();
            woke_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Wait should block until signal is set
        let start = std::time::Instant::now();
        signal.wait();
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(90),
            "Should have waited at least 90ms"
        );
        assert!(
            elapsed < Duration::from_millis(200),
            "Should not have waited too long"
        );
        assert_eq!(
            woke.load(Ordering::SeqCst),
            1,
            "Setter thread should have run"
        );
    }

    #[test]
    fn test_signal_wait_auto_resets() {
        let signal = Arc::new(Signal::new());

        // Set signal
        signal.set();

        // First wait should succeed immediately
        signal.wait();

        // Signal should be auto-reset after wait
        // Second wait should block - verify with timeout
        let result = signal.wait_timeout(Duration::from_millis(50));
        assert!(!result, "Should timeout after auto-reset");
    }

    #[test]
    fn test_signal_wait_when_already_set() {
        let signal = Signal::new();

        // Set it first
        signal.set();

        // Wait should return immediately
        let start = std::time::Instant::now();
        signal.wait();
        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_millis(10),
            "Should return immediately when already set"
        );

        // Should be reset after wait
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Should be reset after wait");
    }

    #[test]
    fn test_signal_wait_resets_after_wait() {
        let signal = Arc::new(Signal::new());
        let signal_clone = signal.clone();

        // Set in another thread
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            signal_clone.set();
        });

        // Wait for signal
        signal.wait();

        // Signal should be reset, so another wait should block
        let result = signal.wait_timeout(Duration::from_millis(50));
        assert!(!result, "Should timeout because signal was auto-reset");
    }

    // ============================================================================
    // Wait Timeout Tests
    // ============================================================================

    #[test]
    fn test_signal_wait_timeout_times_out() {
        let signal = Signal::new();

        // Wait with short timeout on unset signal
        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::from_millis(50));
        let elapsed = start.elapsed();

        assert!(!result, "Should timeout");
        assert!(
            elapsed >= Duration::from_millis(45),
            "Should have waited at least 45ms"
        );
        assert!(
            elapsed < Duration::from_millis(100),
            "Should not wait too long"
        );
    }

    #[test]
    fn test_signal_wait_timeout_zero_timeout() {
        let signal = Signal::new();

        // Zero timeout should behave like wait() (blocks until set)
        let signal_clone = signal.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            signal_clone.set();
        });

        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::ZERO);
        let elapsed = start.elapsed();

        assert!(result, "Should succeed when set");
        assert!(elapsed >= Duration::from_millis(45), "Should have waited");
    }

    #[test]
    fn test_signal_wait_timeout_when_already_set() {
        let signal = Signal::new();

        // Set it first
        signal.set();

        // Wait with timeout should return true immediately
        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::from_millis(100));
        let elapsed = start.elapsed();

        assert!(result, "Should return true when already set");
        assert!(
            elapsed < Duration::from_millis(10),
            "Should return immediately"
        );

        // Should be reset after wait
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Should be reset after wait");
    }

    #[test]
    fn test_signal_wait_timeout_succeeds_when_set() {
        let signal = Arc::new(Signal::new());
        let signal_clone = signal.clone();

        // Set signal after delay
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            signal_clone.set();
        });

        // Wait with timeout should succeed
        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::from_millis(200));
        let elapsed = start.elapsed();

        assert!(result, "Should succeed when set");
        assert!(
            elapsed >= Duration::from_millis(45),
            "Should have waited for signal"
        );
        assert!(
            elapsed < Duration::from_millis(100),
            "Should not wait full timeout"
        );
    }

    #[test]
    fn test_signal_wait_timeout_precise_timing() {
        let signal = Signal::new();

        // Very short timeout
        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::from_millis(1));
        let elapsed = start.elapsed();

        assert!(!result, "Should timeout");
        // Should return relatively quickly (within reason for OS scheduling)
        assert!(elapsed < Duration::from_millis(50), "Should return quickly");
    }

    // ============================================================================
    // Multiple Waiters Tests
    // ============================================================================

    #[test]
    fn test_signal_multiple_waiters_wake_with_multiple_sets() {
        let signal = Arc::new(Signal::new());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // Create multiple waiters
        for _ in 0..5 {
            let signal_clone = signal.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                signal_clone.wait();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Wait for all threads to be waiting
        thread::sleep(Duration::from_millis(100));

        // Verify no one has woken yet
        assert_eq!(
            woke_count.load(Ordering::SeqCst),
            0,
            "No waiters should have woken yet"
        );

        // Set signal multiple times - each set will wake one waiter (due to auto-reset)
        // Need to set N times for N waiters, and wait for each one to complete
        for _ in 0..5 {
            signal.set();
            // Wait to ensure the waiter has woken up and reset the signal
            // before we set it again
            thread::sleep(Duration::from_millis(100));
        }

        // All waiters should wake up - they should all complete since we set 5 times
        // Wait for all handles with a reasonable timeout expectation
        for handle in handles {
            handle.join().unwrap();
        }

        // All waiters should have woken (we set 5 times for 5 waiters)
        assert_eq!(
            woke_count.load(Ordering::SeqCst),
            5,
            "All waiters should have woken"
        );
    }

    #[test]
    fn test_signal_multiple_set_wake_sequential() {
        let signal = Arc::new(Signal::new());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // Create multiple waiters
        for _ in 0..5 {
            let signal_clone = signal.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                signal_clone.wait();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Wait for all threads to be waiting
        thread::sleep(Duration::from_millis(100));

        // Set signal multiple times - each should wake at least one waiter
        for _ in 0..5 {
            signal.set();
            thread::sleep(Duration::from_millis(50));
            // After each set, at least one more waiter should have woken
            // (though due to auto-reset, only one per set typically wakes)
        }

        // All waiters should eventually wake
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(
            woke_count.load(Ordering::SeqCst),
            5,
            "All waiters should have woken"
        );
    }

    // ============================================================================
    // Clone Tests
    // ============================================================================

    #[test]
    fn test_signal_clone_shares_state() {
        let signal = Signal::new();
        let cloned = signal.clone();

        // Set original
        signal.set();

        // Cloned should also see the signal (they share state via Arc)
        let result = cloned.wait_timeout(Duration::from_millis(10));
        assert!(result, "Clone should see the signal");

        // After wait, both should be reset
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Original should be reset after wait");
    }

    #[test]
    fn test_signal_clone_concurrent_access() {
        let signal = Arc::new(Signal::new());
        let cloned1 = signal.clone();
        let cloned2 = signal.clone();

        // Set from one clone
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cloned1.set();
        });

        // Wait from another clone
        let result = cloned2.wait_timeout(Duration::from_millis(200));
        assert!(result, "Should see signal set from another clone");
    }

    // ============================================================================
    // Concurrent Operations Tests
    // ============================================================================

    #[test]
    fn test_signal_concurrent_sets() {
        let signal = Arc::new(Signal::new());
        let mut handles = vec![];

        // Multiple threads setting concurrently
        for _ in 0..10 {
            let signal_clone = signal.clone();
            handles.push(thread::spawn(move || {
                signal_clone.set();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Signal should be set after all sets complete
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(result, "Should be set after concurrent sets");
    }

    #[test]
    fn test_signal_concurrent_set_reset() {
        let signal = Arc::new(Signal::new());
        let mut handles = vec![];

        // Some threads set
        for _ in 0..5 {
            let signal_clone = signal.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    signal_clone.set();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }

        // Some threads reset
        for _ in 0..5 {
            let signal_clone = signal.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    signal_clone.reset();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Operation should complete without panicking
        // The final state depends on race conditions, but operations should be safe
    }

    #[test]
    fn test_signal_concurrent_wait_and_set() {
        let signal = Arc::new(Signal::new());
        let mut wait_handles = vec![];
        let woke_count = Arc::new(AtomicUsize::new(0));

        // Multiple threads waiting
        for _ in 0..5 {
            let signal_clone = signal.clone();
            let woke_clone = woke_count.clone();
            wait_handles.push(thread::spawn(move || {
                signal_clone.wait();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Wait for threads to start waiting
        thread::sleep(Duration::from_millis(100));

        // Set signal multiple times sequentially to wake all waiters
        // Due to auto-reset behavior, we need to set once per waiter
        // and wait between sets to ensure each waiter gets a chance
        for i in 0..6 {
            signal.set();

            // Wait between sets to allow a waiter to wake and reset
            // Stagger the waits slightly
            thread::sleep(Duration::from_millis(50 + (i * 10)));

            // If all waiters have woken, we can stop setting
            if woke_count.load(Ordering::SeqCst) == 5 {
                break;
            }
        }

        // Wait for all wait threads to complete
        // They should all complete since we set at least 5 times (one per waiter)
        for handle in wait_handles {
            handle.join().unwrap();
        }

        // All waiters should have woken (we set at least 5 times)
        assert_eq!(
            woke_count.load(Ordering::SeqCst),
            5,
            "All waiters should have woken"
        );
    }

    // ============================================================================
    // Edge Cases and Complex Scenarios
    // ============================================================================

    #[test]
    fn test_signal_multiple_sequential_waits() {
        let signal = Arc::new(Signal::new());

        // Test multiple wait/set cycles
        for i in 0..5 {
            let signal_clone = signal.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis((i * 50) as u64 + 50));
                signal_clone.set();
            });

            let result = signal.wait_timeout(Duration::from_millis(500));
            assert!(result, "Should be set in cycle {}", i);
            // Signal should be reset after wait
            let result = signal.wait_timeout(Duration::from_millis(10));
            assert!(!result, "Should be reset after wait in cycle {}", i);
        }
    }

    #[test]
    fn test_signal_reset_then_set_then_wait() {
        let signal = Signal::new();

        // Set it
        signal.set();

        // Reset it
        signal.reset();

        // Set it again
        signal.set();

        // Wait should succeed
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(result, "Should succeed after set");

        // Should be reset after wait
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Should be reset after wait");
    }

    #[test]
    fn test_signal_wait_timeout_with_set_during_wait() {
        let signal = Arc::new(Signal::new());
        let signal_clone = signal.clone();

        // Set in another thread after a delay
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            signal_clone.set();
        });

        // Wait with timeout - should succeed when set
        let start = std::time::Instant::now();
        let result = signal.wait_timeout(Duration::from_millis(200));
        let elapsed = start.elapsed();

        assert!(result, "Should succeed when set");
        assert!(
            elapsed >= Duration::from_millis(45),
            "Should have waited for signal"
        );
        assert!(
            elapsed < Duration::from_millis(100),
            "Should not wait full timeout"
        );

        // Should be reset after wait
        let result = signal.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Should be reset after wait");
    }

    #[test]
    fn test_signal_debug_trait() {
        let signal = Signal::new();
        let debug_str = format!("{:?}", signal);
        // Should not panic and should contain some debug info
        assert!(!debug_str.is_empty(), "Debug string should not be empty");
    }

    #[test]
    fn test_signal_default_clone() {
        let signal1 = Signal::default();
        let signal2 = Signal::default();

        // They should be independent
        signal1.set();
        let result = signal2.wait_timeout(Duration::from_millis(10));
        assert!(!result, "Default signals should be independent");

        // signal1 should be set
        let result = signal1.wait_timeout(Duration::from_millis(10));
        assert!(result, "signal1 should be set");
    }
}
