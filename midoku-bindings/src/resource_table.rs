use wasmtime::component::{Resource, ResourceTableError};

/// A specialized `ResourceTable<T>` type that maps a `Resource<T>` to its `T`.
pub(crate) struct ResourceTable<T> {
    entries: Vec<Option<T>>,
    free_cells: Vec<usize>,
}

impl<T> ResourceTable<T>
where
    T: 'static,
{
    /// Create an empty table
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_cells: Vec::new(),
        }
    }

    /// Get an immutable reference to a resource at a given index.
    ///
    /// Multiple shared references can be borrowed at any given time.
    pub fn get(&self, key: &Resource<T>) -> Result<&T, ResourceTableError> {
        self.entries
            .get(key.rep() as usize)
            .and_then(|entry| entry.as_ref())
            .ok_or(ResourceTableError::NotPresent)
    }

    /// Inserts a new value `T` into this table, returning a corresponding
    /// `Resource<T>` which can be used to refer to it after it was inserted.
    pub fn push(&mut self, entry: T) -> Result<Resource<T>, ResourceTableError> {
        let entry = Some(entry);

        let idx = match self.free_cells.pop() {
            Some(idx) => {
                self.entries[idx] = entry;
                idx
            }
            None => {
                self.entries.push(entry);
                self.entries.len() - 1
            }
        };

        Ok(Resource::new_own(idx as u32))
    }

    /// Deletes the resource at a given index and returns the holded value.
    pub fn delete(&mut self, resource: Resource<T>) -> Result<T, ResourceTableError> {
        let idx = resource.rep() as usize;

        if self.entries.get(idx).is_none() {
            return Err(ResourceTableError::NotPresent);
        }

        let entry = std::mem::replace(&mut self.entries[idx], None)
            .ok_or(ResourceTableError::NotPresent)?;

        self.free_cells.push(idx);

        Ok(entry)
    }
}
