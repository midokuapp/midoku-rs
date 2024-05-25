use std::num::NonZeroU32;
use std::time::Duration;

use futures::executor;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter as GovernorRateLimiter};

pub struct RateLimiter {
    burst: u32,
    period_ms: u32,
    limiter: DefaultDirectRateLimiter,
}

impl RateLimiter {
    pub fn burst(&self) -> u32 {
        self.burst
    }

    pub fn period_ms(&self) -> u32 {
        self.period_ms
    }

    pub fn set_burst(&mut self, burst: u32) -> Result<(), ()> {
        if burst == 0 {
            return Err(());
        }

        self.set_limiter(burst, self.period_ms)?;
        self.burst = burst;
        Ok(())
    }

    pub fn set_period_ms(&mut self, period_ms: u32) -> Result<(), ()> {
        if period_ms == 0 {
            return Err(());
        }

        self.set_limiter(self.burst, period_ms)?;
        self.period_ms = period_ms;
        Ok(())
    }

    pub fn set_limiter(&mut self, burst: u32, period_ms: u32) -> Result<(), ()> {
        let burst = NonZeroU32::new(burst).ok_or(())?;
        let period_ms = period_ms as u64;

        let quota = Quota::with_period(Duration::from_millis(period_ms))
            .ok_or(())?
            .allow_burst(burst);

        self.limiter = GovernorRateLimiter::direct(quota);
        Ok(())
    }

    pub fn ready(&self) -> bool {
        self.limiter.check().is_ok()
    }

    pub fn block(&self) {
        executor::block_on(self.limiter.until_ready());
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        let burst = 1;
        let period_ms = 1;
        let quota = Quota::with_period(Duration::from_millis(period_ms as u64))
            .unwrap()
            .allow_burst(NonZeroU32::new(burst).unwrap());
        Self {
            burst,
            period_ms,
            limiter: GovernorRateLimiter::direct(quota),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_default() {
        let rate_limiter = RateLimiter::default();

        assert_eq!(rate_limiter.burst, 1);
        assert_eq!(rate_limiter.period_ms, 1);
    }

    #[test]
    fn test_rate_limiter_get_burst() {
        let rate_limiter = RateLimiter::default();

        assert_eq!(rate_limiter.burst(), 1);
    }

    #[test]
    fn test_rate_limiter_get_period_ms() {
        let rate_limiter = RateLimiter::default();

        assert_eq!(rate_limiter.period_ms(), 1);
    }

    #[test]
    fn test_rate_limiter_set_burst() {
        let mut rate_limiter = RateLimiter::default();

        rate_limiter.set_burst(3).unwrap();
        assert_eq!(rate_limiter.burst, 3);
        assert_eq!(rate_limiter.period_ms, 1);
        assert_eq!(rate_limiter.limiter.check().unwrap(), ());
    }

    #[test]
    fn test_rate_limiter_set_period_ms() {
        let mut rate_limiter = RateLimiter::default();

        rate_limiter.set_period_ms(1000).unwrap();
        assert_eq!(rate_limiter.burst, 1);
        assert_eq!(rate_limiter.period_ms, 1000);
        assert_eq!(rate_limiter.limiter.check().unwrap(), ());
    }

    #[test]
    fn test_rate_limiter_set_burst_zero() {
        let mut rate_limiter = RateLimiter::default();

        assert!(rate_limiter.set_burst(0).is_err());
    }

    #[test]
    fn test_rate_limiter_set_period_ms_zero() {
        let mut rate_limiter = RateLimiter::default();

        assert!(rate_limiter.set_period_ms(0).is_err());
    }

    #[test]
    fn test_rate_limiter_set_limiter() {
        let mut rate_limiter = RateLimiter::default();

        rate_limiter.set_limiter(5, 2000).unwrap();

        // burst and period_ms should not change
        assert_eq!(rate_limiter.burst, 1);
        assert_eq!(rate_limiter.period_ms, 1);
    }

    #[test]
    fn test_rate_limiter_set_limiter_burst_zero() {
        let mut rate_limiter = RateLimiter::default();

        assert!(rate_limiter.set_limiter(0, 2000).is_err());
    }

    #[test]
    fn test_rate_limiter_set_limiter_period_ms_zero() {
        let mut rate_limiter = RateLimiter::default();

        assert!(rate_limiter.set_limiter(5, 0).is_err());
    }

    #[test]
    fn test_rate_limiter_set_limiter_zero() {
        let mut rate_limiter = RateLimiter::default();

        assert!(rate_limiter.set_limiter(0, 0).is_err());
    }

    #[test]
    fn test_rate_limiter_ready() {
        let rate_limiter = RateLimiter::default();

        assert!(rate_limiter.ready());
        assert!(!rate_limiter.ready());
    }

    #[test]
    fn test_rate_limiter_block() {
        let rate_limiter = RateLimiter::default();

        let start_time = std::time::Instant::now();
        rate_limiter.block();
        let elapsed = start_time.elapsed().as_millis();

        assert!(elapsed < 1);

        let start_time = std::time::Instant::now();
        rate_limiter.block();
        let elapsed = start_time.elapsed().as_millis();

        assert!(elapsed >= 1);
    }
}
