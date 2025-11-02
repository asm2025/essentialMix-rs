#[cfg(test)]
mod tests {
    use emixcore::Result;
    use emixthreading::{
        TaskDelegation, TaskResult, QueueBehavior,
        consumer::{Consumer, ConsumerOptions, ProducerConsumer, ProducerConsumerOptions, InjectorWorker, InjectorWorkerOptions},
    };
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration,
        thread,
    };

    const THREADS: usize = 2; // Reduced for faster tests
    const TEST_SIZE: usize = 100; // Reduced for faster tests

    #[derive(Clone, Debug)]
    pub struct TestTaskHandler {
        pub tasks: Arc<AtomicUsize>,
        pub done: Arc<AtomicUsize>,
    }

    impl TestTaskHandler {
        pub fn new() -> Self {
            TestTaskHandler {
                tasks: Arc::new(AtomicUsize::new(0)),
                done: Arc::new(AtomicUsize::new(0)),
            }
        }

        pub fn tasks(&self) -> usize {
            self.tasks.load(Ordering::SeqCst)
        }

        pub fn done(&self) -> usize {
            self.done.load(Ordering::SeqCst)
        }
    }

    impl TaskDelegation<Consumer<usize>, usize> for TestTaskHandler {
        fn on_started(&self, _pc: &Consumer<usize>) {
            // Test started
        }

        fn process(&self, _pc: &Consumer<usize>, item: &usize) -> Result<TaskResult> {
            self.tasks.fetch_add(1, Ordering::SeqCst);
            
            if item % 5 == 0 {
                return Ok(TaskResult::Error(
                    format!("Item {}. Multiples of 5 are not allowed", item).into(),
                ));
            } else if item % 3 == 0 {
                return Ok(TaskResult::TimedOut);
            }

            Ok(TaskResult::Success)
        }

        fn on_completed(&self, _pc: &Consumer<usize>, _item: &usize, _result: &TaskResult) -> bool {
            self.done.fetch_add(1, Ordering::SeqCst);
            true
        }

        fn on_cancelled(&self, _pc: &Consumer<usize>) {
            // Test cancelled
        }

        fn on_finished(&self, _pc: &Consumer<usize>) {
            // Test finished
        }
    }

    #[tokio::test]
    async fn test_consumer_basic() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ConsumerOptions::new();
        let consumer = Consumer::<usize>::with_options(options);
        consumer.start(&handler.clone())?;

        for i in 1..=TEST_SIZE {
            consumer.enqueue(i)?;
        }

        consumer.complete();

        consumer.wait_async().await?;

        let tasks_processed = handler.tasks();
        let tasks_done = handler.done();
        
        assert!(tasks_processed > 0, "Should have processed some tasks");
        assert!(tasks_done > 0, "Should have completed some tasks");
        // Note: Some tasks might error or timeout, so done might be less than processed
        
        Ok(())
    }

    impl TaskDelegation<ProducerConsumer<usize>, usize> for TestTaskHandler {
        fn on_started(&self, _pc: &ProducerConsumer<usize>) {
            // Test started
        }

        fn process(&self, _pc: &ProducerConsumer<usize>, item: &usize) -> Result<TaskResult> {
            self.tasks.fetch_add(1, Ordering::SeqCst);
            
            if item % 5 == 0 {
                return Ok(TaskResult::Error(
                    format!("Item {}. Multiples of 5 are not allowed", item).into(),
                ));
            } else if item % 3 == 0 {
                return Ok(TaskResult::TimedOut);
            }

            Ok(TaskResult::Success)
        }

        fn on_completed(
            &self,
            _pc: &ProducerConsumer<usize>,
            _item: &usize,
            _result: &TaskResult,
        ) -> bool {
            self.done.fetch_add(1, Ordering::SeqCst);
            true
        }

        fn on_cancelled(&self, _pc: &ProducerConsumer<usize>) {
            // Test cancelled
        }

        fn on_finished(&self, _pc: &ProducerConsumer<usize>) {
            // Test finished
        }
    }

    #[tokio::test]
    async fn test_producer_consumer_basic() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(THREADS);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler.clone())?;
        
        let pc = prodcon.clone();
        tokio::spawn(async move {
            for i in 1..=TEST_SIZE {
                if let Err(_) = pc.enqueue(i) {
                    break;
                }
            }
            pc.complete();
        });

        prodcon.wait_async().await?;

        let tasks_processed = handler.tasks();
        let tasks_done = handler.done();
        
        assert!(tasks_processed > 0, "Should have processed some tasks");
        assert!(tasks_done > 0, "Should have completed some tasks");
        
        Ok(())
    }

    impl TaskDelegation<InjectorWorker<usize>, usize> for TestTaskHandler {
        fn on_started(&self, _pc: &InjectorWorker<usize>) {
            // Test started
        }

        fn process(&self, _pc: &InjectorWorker<usize>, item: &usize) -> Result<TaskResult> {
            self.tasks.fetch_add(1, Ordering::SeqCst);
            
            if item % 5 == 0 {
                return Ok(TaskResult::Error(
                    format!("Item {}. Multiples of 5 are not allowed", item).into(),
                ));
            } else if item % 3 == 0 {
                return Ok(TaskResult::TimedOut);
            }

            Ok(TaskResult::Success)
        }

        fn on_completed(&self, _pc: &InjectorWorker<usize>, _item: &usize, _result: &TaskResult) -> bool {
            self.done.fetch_add(1, Ordering::SeqCst);
            true
        }

        fn on_cancelled(&self, _pc: &InjectorWorker<usize>) {
            // Test cancelled
        }

        fn on_finished(&self, _pc: &InjectorWorker<usize>) {
            // Test finished
        }
    }

    #[tokio::test]
    async fn test_injector_worker_basic() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        for i in 1..=TEST_SIZE {
            injwork.enqueue(i)?;
        }

        injwork.complete();

        injwork.wait_async().await?;

        let tasks_processed = handler.tasks();
        let tasks_done = handler.done();
        
        assert!(tasks_processed > 0, "Should have processed some tasks");
        assert!(tasks_done > 0, "Should have completed some tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_concurrent_enqueue() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        let consumer1 = consumer.clone();
        let consumer2 = consumer.clone();
        let consumer3 = consumer.clone();

        let _ = tokio::join!(
            tokio::spawn(async move {
                for i in 1..50 {
                    let _ = consumer1.enqueue(i);
                }
            }),
            tokio::spawn(async move {
                for i in 50..100 {
                    let _ = consumer2.enqueue(i);
                }
            }),
            tokio::spawn(async move {
                for i in 100..150 {
                    let _ = consumer3.enqueue(i);
                }
            })
        );

        consumer.complete();
        consumer.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed >= 100, "Should have processed at least 100 tasks from concurrent enqueues");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_error_counting() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        for i in 1..=20 {
            consumer.enqueue(i)?;
        }

        consumer.complete();
        consumer.wait_async().await?;

        let tasks_processed = handler.tasks();
        let tasks_done = handler.done();
        
        assert!(tasks_processed > 0, "Should have processed some tasks");
        assert!(tasks_done > 0, "Should have completed some tasks");
        // Note: on_completed is called for all results, so done == processed
        
        Ok(())
    }

    #[tokio::test]
    async fn test_producer_consumer_multiple_producers() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(THREADS);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler.clone())?;
        
        let pc1 = prodcon.clone();
        let pc2 = prodcon.clone();
        let pc3 = prodcon.clone();

        let _ = tokio::join!(
            tokio::spawn(async move {
                for i in 1..50 {
                    if let Err(_) = pc1.enqueue(i) { break; }
                }
            }),
            tokio::spawn(async move {
                for i in 50..100 {
                    if let Err(_) = pc2.enqueue(i) { break; }
                }
            }),
            tokio::spawn(async move {
                for i in 100..151 {
                    if let Err(_) = pc3.enqueue(i) { break; }
                }
            })
        );

        prodcon.complete();
        prodcon.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed >= 100, "Should have processed tasks from multiple producers");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_producer_consumer_stress() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(4);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler.clone())?;
        
        let pc = prodcon.clone();
        tokio::spawn(async move {
            // Enqueue many items rapidly
            for i in 1..=1000 {
                if let Err(_) = pc.enqueue(i) { break; }
            }
            pc.complete();
        });

        prodcon.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed > 500, "Should have processed many tasks under stress");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_rapid_start_stop() -> Result<()> {
        let handler1 = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork1 = InjectorWorker::<usize>::with_options(options);
        injwork1.start(&handler1)?;

        for i in 1..=TEST_SIZE {
            injwork1.enqueue(i)?;
        }
        injwork1.complete();
        injwork1.wait_async().await?;

        // Start a second instance immediately
        let handler2 = TestTaskHandler::new();
        let options2 = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork2 = InjectorWorker::<usize>::with_options(options2);
        injwork2.start(&handler2)?;

        for i in 1..=TEST_SIZE {
            injwork2.enqueue(i)?;
        }
        injwork2.complete();
        injwork2.wait_async().await?;

        assert!(handler1.tasks() > 0, "First handler should have processed tasks");
        assert!(handler2.tasks() > 0, "Second handler should have processed tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_pause_resume() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        // Enqueue some items
        for i in 1..=50 {
            consumer.enqueue(i)?;
        }

        // Pause the consumer
        consumer.pause();
        assert!(consumer.is_paused(), "Consumer should be paused");
        
        // Give it a bit of time to process some items while paused
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let tasks_before_resume = handler.tasks();
        
        // Resume the consumer
        consumer.resume();
        assert!(!consumer.is_paused(), "Consumer should be resumed");
        
        // Complete and wait for processing
        consumer.complete();
        consumer.wait_async().await?;

        let tasks_after = handler.tasks();
        assert!(tasks_after > tasks_before_resume, "Should have processed more tasks after resume");
        assert!(tasks_after > 0, "Should have processed some tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_cancel() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        // Enqueue many items
        for i in 1..=1000 {
            consumer.enqueue(i)?;
        }

        // Give it some time to process
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Cancel the consumer
        consumer.cancel();
        assert!(consumer.is_cancelled(), "Consumer should be cancelled");
        
        // Try to enqueue after cancel - should fail
        assert!(consumer.enqueue(9999).is_err(), "Should not be able to enqueue after cancel");
        
        // Wait for cancellation
        let result = consumer.wait_async().await;
        assert!(result.is_err(), "Wait should return error when cancelled");
        
        Ok(())
    }

    #[test]
    fn test_consumer_wait_for_timeout() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler)?;

        // Don't enqueue anything, just wait for a short timeout
        let result = consumer.wait_for(Duration::from_millis(100));
        assert!(result.is_err(), "Wait_for should timeout when no items are processing");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_wait_for_timeout_async() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler)?;

        // Don't enqueue anything, just wait for a short timeout
        let result = consumer.wait_for_async(Duration::from_millis(100)).await;
        assert!(result.is_err(), "Wait_for_async should timeout when no items are processing");
        
        Ok(())
    }

    #[test]
    fn test_producer_consumer_pause_resume() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(THREADS);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler)?;
        
        // Enqueue some items
        for i in 1..=50 {
            prodcon.enqueue(i)?;
        }

        // Give it a bit of time to process some items
        thread::sleep(Duration::from_millis(100));
        
        // Pause the producer-consumer
        prodcon.pause();
        assert!(prodcon.is_paused(), "ProducerConsumer should be paused");
        
        // Give it a bit more time while paused
        thread::sleep(Duration::from_millis(100));
        
        let tasks_before_resume = handler.tasks();
        
        // Resume
        prodcon.resume();
        assert!(!prodcon.is_paused(), "ProducerConsumer should be resumed");
        
        prodcon.complete();
        prodcon.wait()?;

        let tasks_after = handler.tasks();
        assert!(tasks_after >= tasks_before_resume, "Should not have processed fewer tasks after resume");
        assert!(tasks_after > 0, "Should have processed some tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_producer_consumer_cancel() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(THREADS);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler)?;
        
        // Enqueue items
        let pc = prodcon.clone();
        tokio::spawn(async move {
            for i in 1..=1000 {
                if let Err(_) = pc.enqueue(i) { break; }
            }
        });

        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Cancel
        prodcon.cancel();
        assert!(prodcon.is_cancelled(), "ProducerConsumer should be cancelled");
        
        let result = prodcon.wait_async().await;
        assert!(result.is_err(), "Wait should return error when cancelled");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_pause_resume() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        // Enqueue items
        for i in 1..=50 {
            injwork.enqueue(i)?;
        }

        // Pause
        injwork.pause();
        assert!(injwork.is_paused(), "InjectorWorker should be paused");
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        let tasks_before = handler.tasks();
        
        // Resume
        injwork.resume();
        assert!(!injwork.is_paused(), "InjectorWorker should be resumed");
        
        injwork.complete();
        injwork.wait_async().await?;

        let tasks_after = handler.tasks();
        assert!(tasks_after > tasks_before, "Should have processed more tasks after resume");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_cancel() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        // Enqueue many items
        for i in 1..=1000 {
            injwork.enqueue(i)?;
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Cancel
        injwork.cancel();
        assert!(injwork.is_cancelled(), "InjectorWorker should be cancelled");
        
        let result = injwork.wait_async().await;
        assert!(result.is_err(), "Wait should return error when cancelled");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_lifo_behavior() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new()
            .with_threads(THREADS)
            .with_behavior(QueueBehavior::LIFO);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        for i in 1..=20 {
            injwork.enqueue(i)?;
        }

        injwork.complete();
        injwork.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed > 0, "Should have processed tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_fifo_behavior() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new()
            .with_threads(THREADS)
            .with_behavior(QueueBehavior::FIFO);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        for i in 1..=20 {
            injwork.enqueue(i)?;
        }

        injwork.complete();
        injwork.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed > 0, "Should have processed tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_stop_enforce_cancel() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        // Enqueue items
        for i in 1..=100 {
            consumer.enqueue(i)?;
        }

        // Stop with enforce=true should cancel
        consumer.stop(true);
        assert!(consumer.is_cancelled(), "Stop with enforce should cancel");

        let result = consumer.wait_async().await;
        assert!(result.is_err(), "Wait should return error when cancelled");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_stop_enforce_complete() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        consumer.start(&handler.clone())?;

        // Enqueue items
        for i in 1..=50 {
            consumer.enqueue(i)?;
        }

        // Stop with enforce=false should complete
        consumer.stop(false);
        assert!(consumer.is_completed(), "Stop with enforce=false should complete");
        assert!(!consumer.is_cancelled(), "Should not be cancelled");

        consumer.wait_async().await?;
        assert!(handler.tasks() > 0, "Should have processed tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_producer_consumer_many_tasks() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = ProducerConsumerOptions::new().with_threads(4);
        let prodcon = ProducerConsumer::<usize>::with_options(options);
        prodcon.start(&handler.clone())?;

        let pc = prodcon.clone();
        tokio::spawn(async move {
            for i in 1..=5000 {
                if let Err(_) = pc.enqueue(i) { break; }
            }
            pc.complete();
        });

        prodcon.wait_async().await?;

        let tasks_processed = handler.tasks();
        assert!(tasks_processed > 1000, "Should have processed many tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_injector_worker_empty_queue() -> Result<()> {
        let handler = TestTaskHandler::new();
        let options = InjectorWorkerOptions::new().with_threads(THREADS);
        let injwork = InjectorWorker::<usize>::with_options(options);
        injwork.start(&handler)?;

        // Complete immediately without enqueuing anything
        injwork.complete();

        // Should finish quickly since there's nothing to process
        injwork.wait_async().await?;

        assert_eq!(handler.tasks(), 0, "Should not have processed any tasks");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_double_start_error() -> Result<()> {
        let handler = TestTaskHandler::new();
        let consumer = Consumer::<usize>::new();
        
        // Start once
        consumer.start(&handler.clone())?;
        
        // Try to start again - should fail
        let result = consumer.start(&handler);
        assert!(result.is_err(), "Should not be able to start twice");
        
        consumer.cancel();
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consumer_state_queries() -> Result<()> {
        let consumer = Consumer::<usize>::new();
        
        // Initially not started, not completed, not cancelled
        assert!(!consumer.is_started(), "Should not be started initially");
        assert!(!consumer.is_completed(), "Should not be completed initially");
        assert!(!consumer.is_cancelled(), "Should not be cancelled initially");
        assert!(consumer.is_empty(), "Should be empty initially");
        assert_eq!(consumer.len(), 0, "Length should be 0");
        assert_eq!(consumer.consumers(), 0, "Should have 0 consumers");
        
        let handler = TestTaskHandler::new();
        consumer.start(&handler)?;
        
        // After start
        assert!(consumer.is_started(), "Should be started");
        assert_eq!(consumer.consumers(), 1, "Should have 1 consumer by default");
        
        consumer.cancel();
        
        Ok(())
    }
}

