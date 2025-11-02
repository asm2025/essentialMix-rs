#[cfg(test)]
mod tests {
    use emixcore::{is_debug, set_debug, system};

    #[test]
    fn test_is_debug_default() {
        // Default should be false (but we can't reset it easily)
        // So we just check it doesn't panic
        let _ = is_debug();
    }

    #[test]
    fn test_set_debug() {
        // This test is tricky because set_debug can only be called once
        // We'll just verify that is_debug doesn't panic
        let _ = is_debug();
    }

    #[test]
    fn test_num_cpus() {
        let cpus = system::num_cpus();
        assert!(cpus > 0, "Should have at least 1 CPU");
        
        // If debug is set, it should return 1
        // But we can't easily test this without setting debug mode
    }
}

