use std::cell::{Ref, RefCell};

use futures::executor;
use reqwest::Response as ReqwestResponse;

use crate::bindings::exports::midoku::midoku_http::http;

pub(crate) struct Response {
    response: RefCell<Option<ReqwestResponse>>,
}

impl Response {
    pub(crate) fn new(val: ReqwestResponse) -> Self {
        Self {
            response: RefCell::new(Some(val)),
        }
    }

    fn borrow(&self) -> Result<Ref<ReqwestResponse>, http::ResponseError> {
        if self.response.borrow().is_none() {
            return Err(http::ResponseError::Moved);
        }

        Ok(Ref::map(
            self.response.borrow(),
            |response: &Option<ReqwestResponse>| {
                response.as_ref().unwrap_or_else(|| unreachable!())
            },
        ))
    }

    fn take(&self) -> Result<ReqwestResponse, http::ResponseError> {
        if self.response.borrow().is_none() {
            return Err(http::ResponseError::Moved);
        }

        Ok(self.response.take().unwrap_or_else(|| unreachable!()))
    }
}

impl http::GuestResponse for Response {
    fn status_code(&self) -> Result<u16, http::ResponseError> {
        Ok(self.borrow()?.status().as_u16())
    }

    fn headers(&self) -> Result<Vec<(String, String)>, http::ResponseError> {
        let response = self.take()?;
        let header_map = response.headers();

        let mut headers = Vec::with_capacity(header_map.len());
        for (header_name, header_value) in header_map {
            let header_name = header_name.to_string();
            let header_value = header_value
                .to_str()
                .unwrap_or_else(|_| unreachable!())
                .to_string();
            headers.push((header_name, header_value));
        }

        Ok(headers)
    }

    fn bytes(&self) -> Result<Vec<u8>, http::ResponseError> {
        let response = self.take()?;

        Ok(executor::block_on(response.bytes())
            .map(|bytes| bytes.to_vec())
            .unwrap_or(vec![]))
    }
}
