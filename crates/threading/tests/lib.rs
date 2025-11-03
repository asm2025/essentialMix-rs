#[cfg(test)]
mod tests {
    use emixthreading::{QueueBehavior, TaskResult};

    #[test]
    fn test_task_result_none() {
        let result = TaskResult::None;
        assert_eq!(result.to_string(), "");
    }

    #[test]
    fn test_task_result_cancelled() {
        let result = TaskResult::Cancelled;
        assert_eq!(result.to_string(), "Cancelled");
    }

    #[test]
    fn test_task_result_timed_out() {
        let result = TaskResult::TimedOut;
        assert_eq!(result.to_string(), "Timedout");
    }

    #[test]
    fn test_task_result_error() {
        let result = TaskResult::Error("test error".to_string());
        assert_eq!(result.to_string(), "Error: test error");
    }

    #[test]
    fn test_task_result_success() {
        let result = TaskResult::Success;
        assert_eq!(result.to_string(), "Success");
    }

    #[test]
    fn test_task_result_default() {
        assert_eq!(TaskResult::default(), TaskResult::None);
    }

    #[test]
    fn test_task_result_clone() {
        let result = TaskResult::Success;
        let cloned = result.clone();
        assert_eq!(result, cloned);
    }

    #[test]
    fn test_task_result_partial_eq() {
        assert_eq!(TaskResult::None, TaskResult::None);
        assert_ne!(TaskResult::None, TaskResult::Success);
        assert_eq!(
            TaskResult::Error("err".to_string()),
            TaskResult::Error("err".to_string())
        );
    }

    #[test]
    fn test_queue_behavior_fifo() {
        let behavior = QueueBehavior::FIFO;
        assert_eq!(behavior.to_string(), "FIFO");
    }

    #[test]
    fn test_queue_behavior_lifo() {
        let behavior = QueueBehavior::LIFO;
        assert_eq!(behavior.to_string(), "LIFO");
    }

    #[test]
    fn test_queue_behavior_default() {
        assert_eq!(QueueBehavior::default(), QueueBehavior::FIFO);
    }

    #[test]
    fn test_queue_behavior_clone() {
        let behavior = QueueBehavior::FIFO;
        let cloned = behavior.clone();
        assert_eq!(behavior, cloned);
    }

    #[test]
    fn test_queue_behavior_partial_eq() {
        assert_eq!(QueueBehavior::FIFO, QueueBehavior::FIFO);
        assert_ne!(QueueBehavior::FIFO, QueueBehavior::LIFO);
    }

    #[test]
    fn test_queue_behavior_ordering() {
        assert!(QueueBehavior::FIFO < QueueBehavior::LIFO);
    }

    #[test]
    fn test_queue_behavior_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(QueueBehavior::FIFO, "first");
        map.insert(QueueBehavior::LIFO, "last");

        assert_eq!(map.get(&QueueBehavior::FIFO), Some(&"first"));
        assert_eq!(map.get(&QueueBehavior::LIFO), Some(&"last"));
    }

    #[test]
    fn test_threading_constants() {
        use emixthreading::constants::*;
        use std::time::Duration;

        assert_eq!(CAPACITY_DEF, 0);
        assert_eq!(THREADS_DEF, 1);
        assert_eq!(THREADS_MIN, 1);
        assert_eq!(THREADS_MAX, 255);
        assert_eq!(QUEUE_BEHAVIOR_DEF, QueueBehavior::FIFO);
        assert_eq!(THRESHOLD_DEF, Duration::ZERO);
        assert_eq!(SLEEP_AFTER_SEND_DEF, Duration::ZERO);
        assert_eq!(PEEK_TIMEOUT_DEF, Duration::from_millis(50));
        assert_eq!(PEEK_TIMEOUT_MIN, Duration::from_millis(10));
        assert_eq!(PEEK_TIMEOUT_MAX, Duration::from_secs(5));
        assert_eq!(PAUSE_TIMEOUT_DEF, Duration::from_millis(50));
        assert_eq!(PAUSE_TIMEOUT_MIN, Duration::from_millis(10));
        assert_eq!(PAUSE_TIMEOUT_MAX, Duration::from_secs(5));
        assert_eq!(INTERVAL, 100);
    }
}

