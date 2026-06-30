#[tokio::test]
async fn concurrent_tasks_complete() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let c = counter.clone();
        let handle = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            c.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

#[tokio::test]
async fn timeout_kills_slow_task() {
    use std::time::Duration;
    use tokio::time::timeout;

    let result = timeout(Duration::from_millis(50), async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        "completed"
    })
    .await;

    assert!(result.is_err());
}
