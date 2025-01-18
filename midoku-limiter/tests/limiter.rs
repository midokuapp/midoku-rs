use midoku_limiter;

#[tokio::test]
async fn test_rate_limiter() {
    let mut rate_limiter = midoku_limiter::rate_limiter::RateLimiter::default();

    assert_eq!(rate_limiter.burst(), 1);
    assert_eq!(rate_limiter.period_ms(), 1);

    rate_limiter.set_burst(3).unwrap();
    assert_eq!(rate_limiter.burst(), 3);

    rate_limiter.set_period_ms(1000).unwrap();
    assert_eq!(rate_limiter.period_ms(), 1000);

    rate_limiter
        .set_limiter(rate_limiter.burst(), rate_limiter.period_ms())
        .unwrap();

    assert!(rate_limiter.ready());

    for _ in 0..rate_limiter.burst() - 1 {
        let start_time = std::time::Instant::now();
        rate_limiter.block().await;
        let elapsed = start_time.elapsed().as_millis();

        assert!(elapsed < 1);
    }

    assert!(!rate_limiter.ready());
}
