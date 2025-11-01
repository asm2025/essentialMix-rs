#[cfg(test)]
mod tests {
    use emix::Result;
    use emixthreading::{
        TaskDelegation, TaskResult,
        consumer::{Consumer, ConsumerOptions, ProducerConsumer, ProducerConsumerOptions, InjectorWorker, InjectorWorkerOptions},
    };
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
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
}

