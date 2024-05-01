use midoku_limiter::rate_limiter::RateLimiter;
use wasmtime::component::ResourceTable;

pub struct State {
    resource_table: ResourceTable,
    limiter: Option<RateLimiter>,
}

impl State {
    pub fn resource_table(&self) -> &ResourceTable {
        &self.resource_table
    }

    pub fn resource_table_mut(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    pub fn limiter(&self) -> Option<&RateLimiter> {
        self.limiter.as_ref()
    }

    pub fn limiter_mut(&mut self) -> Option<&mut RateLimiter> {
        self.limiter.as_mut()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            resource_table: ResourceTable::new(),
            limiter: None,
        }
    }
}
