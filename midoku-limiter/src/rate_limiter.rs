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
