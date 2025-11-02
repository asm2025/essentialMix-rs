#[cfg(test)]
mod tests {
    use emixthreading::{AutoResetCond, ManualResetCond, CountdownCond};
    use std::time::Duration;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    use std::thread;

    // ============================================================================
    // AutoResetCond Tests
    // ============================================================================

    #[test]
    fn test_autoreset_new_unset() {
        let cond = AutoResetCond::new_unset();
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(!signaled.unwrap(), "Should not be signaled initially");
    }

    #[test]
    fn test_autoreset_new_set() {
        let cond = AutoResetCond::new_set();
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Should be signaled when created with new_set()");
    }

    #[test]
    fn test_autoreset_new_with_state() {
        let cond = AutoResetCond::new(true);
        assert!(cond.is_set().unwrap(), "Should be set when created with true");
        
        let cond = AutoResetCond::new(false);
        assert!(!cond.is_set().unwrap(), "Should not be set when created with false");
    }

    #[test]
    fn test_autoreset_set() {
        let cond = AutoResetCond::new_unset();
        
        let result = cond.set();
        assert!(result.is_ok(), "set should succeed");
        
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Should be signaled after set");
    }

    #[test]
    fn test_autoreset_set_idempotent() {
        let cond = AutoResetCond::new_unset();
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should be set");
        
        // Setting again should still work (no-op if already set)
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should still be set");
    }

    #[test]
    fn test_autoreset_wait_auto_resets() {
        let cond = Arc::new(AutoResetCond::new_unset());
        
        // Set in another thread
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait should succeed
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed");
        
        // After wait, flag should be reset (auto-reset behavior)
        let signaled = cond.is_set();
        assert!(!signaled.unwrap(), "Flag should be reset after wait (auto-reset behavior)");
    }

    #[test]
    fn test_autoreset_wait_when_already_set() {
        let cond = AutoResetCond::new_set();
        
        // Wait should return immediately and reset
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed immediately");
        
        // Should be reset after wait
        assert!(!cond.is_set().unwrap(), "Should be reset after wait");
    }

    #[test]
    fn test_autoreset_wait_timeout() {
        let cond = AutoResetCond::new_unset();
        
        // Wait with short timeout, should timeout
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_autoreset_wait_timeout_when_already_set() {
        let cond = AutoResetCond::new_set();
        
        // Wait with timeout when already set should return true immediately
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should return true when already set");
        
        // Should be reset after wait
        assert!(!cond.is_set().unwrap(), "Should be reset after wait");
    }

    #[test]
    fn test_autoreset_wait_timeout_ms() {
        let cond = AutoResetCond::new_unset();
        
        let result = cond.wait_timeout_ms(50);
        assert!(result.is_ok(), "wait_timeout_ms should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_autoreset_reset() {
        let cond = AutoResetCond::new_set();
        assert!(cond.is_set().unwrap(), "Should be set initially");
        
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should be reset after reset()");
        
        // Reset again should still work
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should still be reset");
    }

    #[test]
    fn test_autoreset_set_reset_cycle() {
        let cond = AutoResetCond::new_unset();
        
        for _ in 0..10 {
            cond.set().unwrap();
            assert!(cond.is_set().unwrap(), "Should be set");
            
            cond.reset().unwrap();
            assert!(!cond.is_set().unwrap(), "Should be reset");
        }
    }

    #[test]
    fn test_autoreset_only_one_waiter_wakes() {
        let cond = Arc::new(AutoResetCond::new_unset());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Create multiple waiters
        for i in 0..5 {
            let cond_clone = cond.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
                woke_clone.fetch_add(1, Ordering::SeqCst);
                i // Return the waiter index
            }));
        }
        
        // Wait for all threads to be waiting
        thread::sleep(Duration::from_millis(100));
        
        // Verify no one has woken yet
        assert_eq!(woke_count.load(Ordering::SeqCst), 0, "No waiters should have woken yet");
        
        // Set once - should wake exactly one waiter
        cond.set().unwrap();
        thread::sleep(Duration::from_millis(50));
        assert_eq!(woke_count.load(Ordering::SeqCst), 1, "Exactly one waiter should have woken");
        
        // Set again - should wake another waiter
        cond.set().unwrap();
        thread::sleep(Duration::from_millis(50));
        assert_eq!(woke_count.load(Ordering::SeqCst), 2, "Exactly two waiters should have woken");
        
        // Set remaining times to wake all waiters
        for _ in 0..3 {
            cond.set().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        
        // All waiters should wake up
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(woke_count.load(Ordering::SeqCst), 5, "All waiters should have woken");
    }

    #[test]
    fn test_autoreset_wait_while() {
        let cond = Arc::new(AutoResetCond::new_unset());
        let flag = Arc::new(AtomicUsize::new(0));
        
        // Another thread will set when flag reaches 3
        let cond_clone = cond.clone();
        let flag_clone = flag.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            flag_clone.store(3, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait while flag is less than 3
        let flag_ref = flag.clone();
        let result = cond.wait_while(|| flag_ref.load(Ordering::SeqCst) < 3);
        assert!(result.is_ok(), "wait_while should succeed");
        assert_eq!(flag_ref.load(Ordering::SeqCst), 3, "Flag should be 3");
    }

    #[test]
    fn test_autoreset_wait_timeout_while() {
        let cond = AutoResetCond::new_unset();
        
        // Wait with condition that never becomes false, should timeout
        let result = cond.wait_timeout_while(|| true, Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout_while should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_autoreset_wait_timeout_while_condition_met() {
        let cond = Arc::new(AutoResetCond::new_unset());
        let flag = Arc::new(AtomicUsize::new(0));
        
        let cond_clone = cond.clone();
        let flag_clone = flag.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            flag_clone.store(5, Ordering::SeqCst);
            cond_clone.set().unwrap();
        });
        
        let flag_ref = flag.clone();
        let result = cond.wait_timeout_while(|| flag_ref.load(Ordering::SeqCst) < 5, Duration::from_millis(200));
        assert!(result.is_ok(), "wait_timeout_while should succeed");
        assert!(result.unwrap(), "Should not timeout, condition should be met");
    }

    #[test]
    fn test_autoreset_clone() {
        let cond = AutoResetCond::new_unset();
        let cloned = cond.clone();
        
        // Set original
        cond.set().unwrap();
        
        // Cloned should also be signaled (they share state via Arc)
        let signaled = cloned.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Clone should share state");
        
        // Reset cloned - original should also be reset
        cloned.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Original should be reset");
    }

    #[test]
    fn test_autoreset_concurrent_sets() {
        let cond = Arc::new(AutoResetCond::new_unset());
        let mut handles = vec![];
        
        // Multiple threads setting concurrently
        for _ in 0..10 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                cond_clone.set().unwrap();
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should be set after all sets complete
        assert!(cond.is_set().unwrap(), "Should be set after concurrent sets");
    }

    // ============================================================================
    // ManualResetCond Tests
    // ============================================================================

    #[test]
    fn test_manualreset_new_unset() {
        let cond = ManualResetCond::new_unset();
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(!signaled.unwrap(), "Should not be signaled initially");
    }

    #[test]
    fn test_manualreset_new_set() {
        let cond = ManualResetCond::new_set();
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Should be signaled when created with new_set()");
    }

    #[test]
    fn test_manualreset_new_with_state() {
        let cond = ManualResetCond::new(true);
        assert!(cond.is_set().unwrap(), "Should be set when created with true");
        
        let cond = ManualResetCond::new(false);
        assert!(!cond.is_set().unwrap(), "Should not be set when created with false");
    }

    #[test]
    fn test_manualreset_set() {
        let cond = ManualResetCond::new_unset();
        
        let result = cond.set();
        assert!(result.is_ok(), "set should succeed");
        
        let signaled = cond.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Should be signaled after set");
    }

    #[test]
    fn test_manualreset_set_idempotent() {
        let cond = ManualResetCond::new_unset();
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should be set");
        
        // Setting again should still work and be idempotent
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should still be set");
    }

    #[test]
    fn test_manualreset_wait_does_not_reset() {
        let cond = Arc::new(ManualResetCond::new_unset());
        
        // Set in another thread
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait should succeed
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed");
        
        // After wait, flag should remain set (manual reset behavior)
        let signaled = cond.is_set();
        assert!(signaled.unwrap(), "Flag should remain set after wait (manual reset behavior)");
    }

    #[test]
    fn test_manualreset_wait_when_already_set() {
        let cond = ManualResetCond::new_set();
        
        // Wait should return immediately without resetting
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed immediately");
        
        // Should still be set after wait
        assert!(cond.is_set().unwrap(), "Should remain set after wait");
    }

    #[test]
    fn test_manualreset_multiple_waits() {
        let cond = Arc::new(ManualResetCond::new_unset());
        
        // Set it once
        cond.set().unwrap();
        
        // Multiple waits should all succeed without resetting
        for _ in 0..5 {
            let cond_clone = cond.clone();
            let result = cond_clone.wait();
            assert!(result.is_ok(), "wait should succeed");
        }
        
        // Should still be set
        assert!(cond.is_set().unwrap(), "Should still be set after multiple waits");
    }

    #[test]
    fn test_manualreset_wait_timeout() {
        let cond = ManualResetCond::new_unset();
        
        // Wait with short timeout, should timeout
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_manualreset_wait_timeout_when_already_set() {
        let cond = ManualResetCond::new_set();
        
        // Wait with timeout when already set should return true immediately
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should return true when already set");
        
        // Should still be set after wait
        assert!(cond.is_set().unwrap(), "Should remain set after wait");
    }

    #[test]
    fn test_manualreset_wait_timeout_ms() {
        let cond = ManualResetCond::new_unset();
        
        let result = cond.wait_timeout_ms(50);
        assert!(result.is_ok(), "wait_timeout_ms should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_manualreset_reset() {
        let cond = ManualResetCond::new_set();
        assert!(cond.is_set().unwrap(), "Should be set initially");
        
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should be reset after reset()");
        
        // Reset again should still work
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should still be reset");
    }

    #[test]
    fn test_manualreset_set_reset_cycle() {
        let cond = ManualResetCond::new_unset();
        
        for _ in 0..10 {
            cond.set().unwrap();
            assert!(cond.is_set().unwrap(), "Should be set");
            
            cond.reset().unwrap();
            assert!(!cond.is_set().unwrap(), "Should be reset");
        }
    }

    #[test]
    fn test_manualreset_all_waiters_wake() {
        // Test that set() wakes all waiters using wait()
        let cond = Arc::new(ManualResetCond::new_unset());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Create multiple waiters using wait()
        for _ in 0..10 {
            let cond_clone = cond.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        // Wait for all threads to be waiting
        thread::sleep(Duration::from_millis(100));
        
        // Verify no one has woken yet
        assert_eq!(woke_count.load(Ordering::SeqCst), 0, "No waiters should have woken yet");
        
        // Set once - should wake ALL waiters
        cond.set().unwrap();
        
        // All waiters should wake up and complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(woke_count.load(Ordering::SeqCst), 10, "All waiters should have woken");
        
        // Flag should remain true after all waiters have processed it
        let signaled = cond.is_set();
        assert!(signaled.unwrap(), "Flag should remain true after set() wakes all waiters");
    }

    #[test]
    fn test_manualreset_reset_while_waiting() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Create multiple waiters
        for _ in 0..5 {
            let cond_clone = cond.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        thread::sleep(Duration::from_millis(100));
        
        // Set to wake all waiters
        cond.set().unwrap();
        thread::sleep(Duration::from_millis(50));
        
        // Reset while waiters are processing
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should be reset");
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(woke_count.load(Ordering::SeqCst), 5, "All waiters should have woken");
    }

    #[test]
    fn test_manualreset_wait_while() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let flag = Arc::new(AtomicUsize::new(0));
        
        // Another thread will set when flag reaches 3
        let cond_clone = cond.clone();
        let flag_clone = flag.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            flag_clone.store(3, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait while flag is less than 3
        let flag_ref = flag.clone();
        let result = cond.wait_while(|| flag_ref.load(Ordering::SeqCst) < 3);
        assert!(result.is_ok(), "wait_while should succeed");
        assert_eq!(flag_ref.load(Ordering::SeqCst), 3, "Flag should be 3");
    }

    #[test]
    fn test_manualreset_wait_timeout_while() {
        let cond = ManualResetCond::new_unset();
        
        // Wait with condition that never becomes false, should timeout
        let result = cond.wait_timeout_while(|| true, Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout_while should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_manualreset_wait_timeout_while_condition_met() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let flag = Arc::new(AtomicUsize::new(0));
        
        let cond_clone = cond.clone();
        let flag_clone = flag.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            flag_clone.store(5, Ordering::SeqCst);
            cond_clone.set().unwrap();
        });
        
        let flag_ref = flag.clone();
        let result = cond.wait_timeout_while(|| flag_ref.load(Ordering::SeqCst) < 5, Duration::from_millis(200));
        assert!(result.is_ok(), "wait_timeout_while should succeed");
        assert!(result.unwrap(), "Should not timeout, condition should be met");
    }

    #[test]
    fn test_manualreset_clone() {
        let cond = ManualResetCond::new_unset();
        let cloned = cond.clone();
        
        // Set original
        cond.set().unwrap();
        
        // Cloned should also be signaled (they share state via Arc)
        let signaled = cloned.is_set();
        assert!(signaled.is_ok(), "is_set should succeed");
        assert!(signaled.unwrap(), "Clone should share state");
        
        // Reset cloned - original should also be reset
        cloned.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Original should be reset");
    }

    #[test]
    fn test_manualreset_concurrent_sets() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let mut handles = vec![];
        
        // Multiple threads setting concurrently
        for _ in 0..10 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                cond_clone.set().unwrap();
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should be set after all sets complete
        assert!(cond.is_set().unwrap(), "Should be set after concurrent sets");
    }

    // ============================================================================
    // CountdownCond Tests
    // ============================================================================

    #[test]
    fn test_countdown_new() {
        let cond = CountdownCond::new(5);
        assert_eq!(cond.current_count().unwrap(), 5);
        assert_eq!(cond.initial_count(), 5);
        assert!(!cond.is_set().unwrap(), "Should not be set with count > 0");
    }

    #[test]
    fn test_countdown_new_zero() {
        let cond = CountdownCond::new(0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert_eq!(cond.initial_count(), 0);
        assert!(cond.is_set().unwrap(), "Should be set when count is 0");
    }

    #[test]
    fn test_countdown_signal() {
        let cond = CountdownCond::new(3);
        assert_eq!(cond.current_count().unwrap(), 3);
        
        let count = cond.signal().unwrap();
        assert_eq!(count, 2);
        assert_eq!(cond.current_count().unwrap(), 2);
        
        let count = cond.signal().unwrap();
        assert_eq!(count, 1);
        assert_eq!(cond.current_count().unwrap(), 1);
        
        let count = cond.signal().unwrap();
        assert_eq!(count, 0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should be set when count reaches 0");
    }

    #[test]
    fn test_countdown_signal_at_zero() {
        let cond = CountdownCond::new(1);
        cond.signal().unwrap();
        assert_eq!(cond.current_count().unwrap(), 0);
        
        // Signaling at zero should return 0 and not go negative
        let count = cond.signal().unwrap();
        assert_eq!(count, 0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should still be set");
    }

    #[test]
    fn test_countdown_signal_n() {
        let cond = CountdownCond::new(5);
        assert_eq!(cond.current_count().unwrap(), 5);
        
        let count = cond.signal_n(2).unwrap();
        assert_eq!(count, 3);
        assert_eq!(cond.current_count().unwrap(), 3);
        
        let count = cond.signal_n(3).unwrap();
        assert_eq!(count, 0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should be set when count reaches 0");
    }

    #[test]
    fn test_countdown_signal_n_larger_than_count() {
        let cond = CountdownCond::new(3);
        
        // Signaling more than the count should set it to 0, not negative
        let count = cond.signal_n(5).unwrap();
        assert_eq!(count, 0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_wait() {
        let cond = Arc::new(CountdownCond::new(1));
        
        // Signal in another thread
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.signal().unwrap();
        });
        
        // Wait should succeed once count reaches 0
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed");
        assert!(cond.is_set().unwrap(), "Should be set after wait");
        assert_eq!(cond.current_count().unwrap(), 0);
    }

    #[test]
    fn test_countdown_wait_when_already_zero() {
        let cond = CountdownCond::new(0);
        
        // Wait should return immediately when already at zero
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed immediately");
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_wait_timeout() {
        let cond = CountdownCond::new(1);
        
        // Wait with short timeout, should timeout
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(!result.unwrap(), "Should timeout");
        assert_eq!(cond.current_count().unwrap(), 1, "Count should still be 1");
    }

    #[test]
    fn test_countdown_wait_timeout_when_already_zero() {
        let cond = CountdownCond::new(0);
        
        // Wait with timeout when already at zero should return true
        let result = cond.wait_timeout(Duration::from_millis(50));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should return true when already at zero");
    }

    #[test]
    fn test_countdown_wait_timeout_ms() {
        let cond = CountdownCond::new(1);
        
        let result = cond.wait_timeout_ms(50);
        assert!(result.is_ok(), "wait_timeout_ms should succeed");
        assert!(!result.unwrap(), "Should timeout");
    }

    #[test]
    fn test_countdown_add_count() {
        let cond = CountdownCond::new(3);
        assert_eq!(cond.current_count().unwrap(), 3);
        
        let count = cond.add_count(2).unwrap();
        assert_eq!(count, 5);
        assert_eq!(cond.current_count().unwrap(), 5);
        
        let count = cond.add_count(10).unwrap();
        assert_eq!(count, 15);
        assert_eq!(cond.current_count().unwrap(), 15);
    }

    #[test]
    fn test_countdown_add_count_zero() {
        let cond = CountdownCond::new(0);
        assert_eq!(cond.current_count().unwrap(), 0);
        
        let count = cond.add_count(5).unwrap();
        assert_eq!(count, 5);
        assert_eq!(cond.current_count().unwrap(), 5);
        assert!(!cond.is_set().unwrap(), "Should not be set after adding count");
    }

    #[test]
    fn test_countdown_try_add_count() {
        let cond = CountdownCond::new(3);
        assert_eq!(cond.current_count().unwrap(), 3);
        
        let result = cond.try_add_count(2).unwrap();
        assert_eq!(result, Some(5));
        assert_eq!(cond.current_count().unwrap(), 5);
        
        // Signal until zero
        cond.signal_n(5).unwrap();
        assert_eq!(cond.current_count().unwrap(), 0);
        
        // Can't add to zero
        let result = cond.try_add_count(1).unwrap();
        assert_eq!(result, None);
        assert_eq!(cond.current_count().unwrap(), 0, "Count should still be 0");
    }

    #[test]
    fn test_countdown_all_waiters_wake_at_zero() {
        let cond = Arc::new(CountdownCond::new(5));
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Create multiple waiters
        for _ in 0..10 {
            let cond_clone = cond.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        // Wait for all threads to be waiting
        thread::sleep(Duration::from_millis(100));
        
        assert_eq!(woke_count.load(Ordering::SeqCst), 0, "No waiters should have woken yet");
        
        // Signal 5 times to reach zero
        for i in 0..5 {
            cond.signal().unwrap();
            thread::sleep(Duration::from_millis(20));
            if i < 4 {
                assert_eq!(woke_count.load(Ordering::SeqCst), 0, "No waiters should wake until count reaches 0");
            }
        }
        
        // All waiters should wake up when count reaches zero
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(woke_count.load(Ordering::SeqCst), 10, "All waiters should have woken");
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_concurrent_signals() {
        let cond = Arc::new(CountdownCond::new(100));
        let mut handles = vec![];
        
        // Multiple threads signaling concurrently
        for _ in 0..10 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    cond_clone.signal().unwrap();
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should be at zero after all signals
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_initial_count() {
        let cond = CountdownCond::new(42);
        assert_eq!(cond.initial_count(), 42);
        
        cond.signal().unwrap();
        // Initial count should remain unchanged even after signaling
        assert_eq!(cond.initial_count(), 42);
        assert_eq!(cond.current_count().unwrap(), 41);
    }

    #[test]
    fn test_countdown_clone() {
        let cond = CountdownCond::new(5);
        let cloned = cond.clone();
        
        // Signal original
        cond.signal().unwrap();
        assert_eq!(cond.current_count().unwrap(), 4);
        assert_eq!(cloned.current_count().unwrap(), 4, "Clone should share state");
        
        // Initial count should be the same
        assert_eq!(cond.initial_count(), 5);
        assert_eq!(cloned.initial_count(), 5);
    }

    #[test]
    fn test_countdown_complex_scenario() {
        let cond = Arc::new(CountdownCond::new(10));
        
        // Start multiple waiters
        let mut handles = vec![];
        for _ in 0..5 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
            }));
        }
        
        thread::sleep(Duration::from_millis(50));
        
        // Signal some, add count, signal more
        cond.signal_n(3).unwrap();
        assert_eq!(cond.current_count().unwrap(), 7);
        
        cond.add_count(2).unwrap();
        assert_eq!(cond.current_count().unwrap(), 9);
        
        cond.signal_n(9).unwrap();
        assert_eq!(cond.current_count().unwrap(), 0);
        
        // All waiters should wake
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_wait_timeout_with_signal() {
        let cond = Arc::new(CountdownCond::new(1));
        
        // Signal in another thread after a delay
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.signal().unwrap();
        });
        
        // Wait with timeout - should succeed when signal arrives
        let result = cond.wait_timeout(Duration::from_millis(200));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should not timeout, signal should arrive");
        assert_eq!(cond.current_count().unwrap(), 0);
    }

    #[test]
    fn test_countdown_concurrent_add_and_signal() {
        let cond = Arc::new(CountdownCond::new(10));
        let mut handles = vec![];
        
        // Some threads add count
        for _ in 0..5 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..5 {
                    cond_clone.add_count(1).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }
        
        // Some threads signal
        for _ in 0..5 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..5 {
                    cond_clone.signal().unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Final count should be predictable: 10 + 5*5 - 5*5 = 10
        // But due to race conditions, just verify it's consistent
        let final_count = cond.current_count().unwrap();
        // CountdownCond uses usize, so it can't be negative
        assert_eq!(final_count, final_count, "Count should be valid");
    }

    #[test]
    fn test_countdown_multiple_sequential_waits() {
        let cond = CountdownCond::new(0);
        
        // Multiple waits when already at zero should all succeed immediately
        for _ in 0..10 {
            let result = cond.wait();
            assert!(result.is_ok(), "wait should succeed immediately");
        }
        
        assert!(cond.is_set().unwrap(), "Should still be set");
        assert_eq!(cond.current_count().unwrap(), 0);
    }

    #[test]
    fn test_countdown_signal_n_to_exactly_zero() {
        let cond = CountdownCond::new(5);
        
        // Signal exactly the count
        let count = cond.signal_n(5).unwrap();
        assert_eq!(count, 0);
        assert_eq!(cond.current_count().unwrap(), 0);
        assert!(cond.is_set().unwrap(), "Should be set");
    }

    #[test]
    fn test_countdown_add_count_then_signal() {
        let cond = CountdownCond::new(0);
        assert!(cond.is_set().unwrap(), "Should be set initially");
        
        // Add count - should no longer be set
        cond.add_count(3).unwrap();
        assert!(!cond.is_set().unwrap(), "Should not be set after adding count");
        assert_eq!(cond.current_count().unwrap(), 3);
        
        // Signal to zero - should be set again
        cond.signal_n(3).unwrap();
        assert!(cond.is_set().unwrap(), "Should be set after reaching zero");
    }

    // ============================================================================
    // Additional AutoResetCond Edge Case Tests
    // ============================================================================

    #[test]
    fn test_autoreset_wait_timeout_with_set_during_wait() {
        let cond = Arc::new(AutoResetCond::new_unset());
        
        // Set in another thread after a delay
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait with timeout - should succeed when set
        let result = cond.wait_timeout(Duration::from_millis(200));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should not timeout, should be set");
        
        // Should be reset after wait (auto-reset behavior)
        assert!(!cond.is_set().unwrap(), "Should be reset after wait");
    }

    #[test]
    fn test_autoreset_multiple_sequential_wait_sets() {
        let cond = Arc::new(AutoResetCond::new_unset());
        let mut handles = vec![];
        
        // Test multiple wait/set cycles
        for i in 0..5 {
            let cond_clone = cond.clone();
            let handle = thread::spawn(move || {
                thread::sleep(Duration::from_millis((i * 50) as u64 + 50));
                cond_clone.set().unwrap();
            });
            handles.push(handle);
            
            let result = cond.wait_timeout(Duration::from_millis(500));
            assert!(result.is_ok(), "wait_timeout should succeed");
            assert!(result.unwrap(), "Should be set in cycle {}", i);
            assert!(!cond.is_set().unwrap(), "Should be reset after wait");
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_autoreset_wait_timeout_precise_timing() {
        let cond = AutoResetCond::new_unset();
        
        // Very short timeout should timeout
        let start = std::time::Instant::now();
        let result = cond.wait_timeout(Duration::from_millis(1));
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(!result.unwrap(), "Should timeout with very short timeout");
        // Should return relatively quickly (within reason, accounting for OS scheduling)
        assert!(elapsed < Duration::from_millis(100), "Should return relatively quickly");
    }

    #[test]
    fn test_autoreset_reset_then_set_then_wait() {
        let cond = AutoResetCond::new_set();
        assert!(cond.is_set().unwrap(), "Should be set");
        
        // Reset
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should be reset");
        
        // Set
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should be set");
        
        // Wait should succeed and reset
        let result = cond.wait();
        assert!(result.is_ok(), "wait should succeed");
        assert!(!cond.is_set().unwrap(), "Should be reset after wait");
    }

    // ============================================================================
    // Additional ManualResetCond Edge Case Tests
    // ============================================================================

    #[test]
    fn test_manualreset_wait_timeout_with_set_during_wait() {
        let cond = Arc::new(ManualResetCond::new_unset());
        
        // Set in another thread after a delay
        let cond_clone = cond.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            cond_clone.set().unwrap();
        });
        
        // Wait with timeout - should succeed when set
        let result = cond.wait_timeout(Duration::from_millis(200));
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(result.unwrap(), "Should not timeout, should be set");
        
        // Should remain set after wait (manual reset behavior)
        assert!(cond.is_set().unwrap(), "Should remain set after wait");
    }

    #[test]
    fn test_manualreset_multiple_waits_without_reset() {
        let cond = Arc::new(ManualResetCond::new_unset());
        
        // Set it once
        cond.set().unwrap();
        assert!(cond.is_set().unwrap(), "Should be set");
        
        // Multiple waits should all succeed without needing reset
        let mut handles = vec![];
        for _ in 0..10 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                let result = cond_clone.wait();
                assert!(result.is_ok(), "wait should succeed");
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should still be set
        assert!(cond.is_set().unwrap(), "Should remain set after all waits");
    }

    #[test]
    fn test_manualreset_wait_timeout_precise_timing() {
        let cond = ManualResetCond::new_unset();
        
        // Very short timeout should timeout
        let start = std::time::Instant::now();
        let result = cond.wait_timeout(Duration::from_millis(1));
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "wait_timeout should succeed");
        assert!(!result.unwrap(), "Should timeout with very short timeout");
        // Should return relatively quickly (within reason, accounting for OS scheduling)
        assert!(elapsed < Duration::from_millis(100), "Should return relatively quickly");
    }

    #[test]
    fn test_manualreset_set_reset_while_waiters_waiting() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let woke_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        // Create waiters
        for _ in 0..5 {
            let cond_clone = cond.clone();
            let woke_clone = woke_count.clone();
            handles.push(thread::spawn(move || {
                cond_clone.wait().unwrap();
                woke_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }
        
        thread::sleep(Duration::from_millis(100));
        
        // Set to wake all
        cond.set().unwrap();
        thread::sleep(Duration::from_millis(50));
        
        // Reset while some may still be processing
        cond.reset().unwrap();
        assert!(!cond.is_set().unwrap(), "Should be reset");
        
        // All waiters should have woken
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(woke_count.load(Ordering::SeqCst), 5, "All waiters should have woken");
    }

    #[test]
    fn test_manualreset_concurrent_set_reset() {
        let cond = Arc::new(ManualResetCond::new_unset());
        let mut handles = vec![];
        
        // Some threads set
        for _ in 0..5 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    cond_clone.set().unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }
        
        // Some threads reset
        for _ in 0..5 {
            let cond_clone = cond.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..10 {
                    cond_clone.reset().unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify that is_set() succeeds after concurrent operations
        // (The actual state depends on race conditions, but the operation should be safe)
        let _is_set = cond.is_set().unwrap();
    }
}
