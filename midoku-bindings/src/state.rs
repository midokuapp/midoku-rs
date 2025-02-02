use std::collections::HashMap;

use midoku_http::types::IncomingResponse;
use midoku_limiter::rate_limiter::RateLimiter;
use midoku_settings::types::Value;

use crate::resource_table::ResourceTable;

pub(crate) struct ResourceTables {
    pub incoming_response: ResourceTable<IncomingResponse>,
}

pub struct State {
    pub(crate) resource_tables: ResourceTables,
    limiter: Option<RateLimiter>,
    settings: HashMap<String, Value>,
}

impl State {
    pub fn limiter(&self) -> Option<&RateLimiter> {
        self.limiter.as_ref()
    }

    pub fn limiter_mut(&mut self) -> Option<&mut RateLimiter> {
        self.limiter.as_mut()
    }

    pub fn set_limiter(&mut self, limiter: RateLimiter) {
        self.limiter = Some(limiter);
    }

    pub fn settings(&self) -> &HashMap<String, Value> {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.settings
    }
}

impl Default for State {
    fn default() -> Self {
        let resource_tables = ResourceTables {
            incoming_response: ResourceTable::new(),
        };

        Self {
            resource_tables,
            limiter: None,
            settings: HashMap::new(),
        }
    }
}
