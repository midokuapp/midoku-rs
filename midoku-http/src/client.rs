use std::cell::{Ref, RefCell};
use std::num::NonZeroU32;
use std::time::Duration;

use futures::executor;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client as ReqwestClient;

use crate::bindings::exports::midoku::midoku_http::http;

use crate::response::Response;

pub(crate) struct Client {
    period_seconds: RefCell<Option<u32>>,
    burst_size: RefCell<Option<u32>>,
    limiter: RefCell<Option<DefaultDirectRateLimiter>>,
    client: ReqwestClient,
}

impl Client {
    fn get_period_seconds(&self) -> Ref<Option<u32>> {
        self.period_seconds.borrow()
    }

    fn get_burst_size(&self) -> Ref<Option<u32>> {
        self.burst_size.borrow()
    }

    fn get_limiter(&self) -> Ref<Option<DefaultDirectRateLimiter>> {
        self.limiter.borrow()
    }

    fn set_period_seconds(&self, period_seconds: u32) -> Result<(), ()> {
        if period_seconds == 0 {
            return Err(());
        }

        *self.period_seconds.borrow_mut() = Some(period_seconds);

        Ok(())
    }

    fn set_burst_size(&self, burst_size: u32) -> Result<(), ()> {
        if burst_size == 0 {
            return Err(());
        }

        *self.burst_size.borrow_mut() = Some(burst_size);

        Ok(())
    }

    fn set_limiter(&self, period_seconds: u32, burst_size: u32) -> Result<(), ()> {
        if period_seconds == 0 || burst_size == 0 {
            return Err(());
        }

        let period_seconds = period_seconds as u64;
        let burst_size = NonZeroU32::new(burst_size).unwrap_or_else(|| unreachable!());

        let quota = Quota::with_period(Duration::from_secs(period_seconds))
            .ok_or(())?
            .allow_burst(burst_size);

        *self.limiter.borrow_mut() = Some(RateLimiter::direct(quota));

        Ok(())
    }
}

impl http::GuestClient for Client {
    fn new() -> Self {
        Self {
            period_seconds: RefCell::new(None),
            burst_size: RefCell::new(None),
            limiter: RefCell::new(None),
            client: ReqwestClient::new(),
        }
    }

    fn set_rate_limit_period(&self, seconds: u32) -> Result<(), ()> {
        self.set_period_seconds(seconds)?;
        let burst_size = self.get_burst_size().clone().unwrap_or(1);

        self.set_limiter(seconds, burst_size)
    }

    fn set_rate_limit_burst(&self, burst_size: u32) -> Result<(), ()> {
        let period_seconds = self.get_period_seconds().clone().unwrap_or(1);
        self.set_burst_size(burst_size)?;

        self.set_limiter(period_seconds, burst_size)
    }

    fn send(
        &self,
        method: http::Method,
        url: String,
        headers: Option<Vec<(String, String)>>,
        body_bytes: Option<Vec<u8>>,
    ) -> Result<http::Response, ()> {
        if let Some(limiter) = self.get_limiter().as_ref() {
            executor::block_on(limiter.until_ready());
        }

        let mut header_map = HeaderMap::new();
        if let Some(headers) = headers {
            for (header_name, header_value) in headers {
                let header_name = header_name.to_lowercase();
                let header_name = HeaderName::from_lowercase(header_name.as_bytes());
                let header_name = header_name.map_err(|_| ())?;
                let header_value = HeaderValue::from_str(&header_value);
                let header_value = header_value.map_err(|_| ())?;
                header_map.insert(header_name, header_value);
            }
        }

        let mut request_builder = self.client.request(method.into(), url).headers(header_map);
        if let Some(body_bytes) = body_bytes {
            request_builder = request_builder.body(body_bytes);
        }

        let response = executor::block_on(request_builder.send()).map_err(|_| ())?;
        let response = Response::new(response);

        Ok(http::Response::new(response))
    }
}
