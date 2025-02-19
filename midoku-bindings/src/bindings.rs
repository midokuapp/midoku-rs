use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use midoku_settings::types::Value;
use midoku_types::chapter::Chapter;
use midoku_types::filter::Filter;
use midoku_types::manga::Manga;
use midoku_types::page::Page;
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use wasmtime::component::{Component, Linker, TypedFunc};
use wasmtime::{Config, Engine, Store};

use crate::func::FuncExt;
use crate::instance_impl::midoku_http::map_midoku_http;
use crate::instance_impl::midoku_limiter::map_midoku_limiter;
use crate::instance_impl::midoku_settings::map_midoku_settings;
use crate::state::State;

/// Bindings to a Midoku source.
///
/// This struct contains the bindings to a Midoku source. It is used to call
/// functions in the WebAssembly component.
pub struct Bindings {
    store: Arc<RwLock<Store<State>>>,
    initialize: TypedFunc<(), (Result<(), ()>,)>,
    get_manga_list: TypedFunc<(Vec<Filter>, u32), (Result<(Vec<Manga>, bool), ()>,)>,
    get_manga_details: TypedFunc<(String,), (Result<Manga, ()>,)>,
    get_chapter_list: TypedFunc<(String,), (Result<Vec<Chapter>, ()>,)>,
    get_page_list: TypedFunc<(String, String), (Result<Vec<Page>, ()>,)>,
}

#[doc(hidden)]
macro_rules! get_typed_func {
    ($instance:expr, $store:expr, $api:expr, $name:expr) => {{
        let index = $instance
            .get_export(&mut $store, Some(&$api), $name)
            .unwrap();
        $instance.get_typed_func(&mut $store, index)
    }};
}

impl Bindings {
    /// Create a new instance of the bindings from a WebAssembly component
    /// .wasm file.
    ///
    /// # Example
    /// ```ignore
    /// let bindings = Bindings::from_file("example.wasm").await?;
    ///
    /// // Call the wasm `initialize` function which contains the initialization
    /// // logic, such as the rate limiter configuration.
    /// bindings.initialize().await?;
    ///
    /// // Call the wasm `get_manga_list` function to get the list of manga.
    /// let (manga_list, has_next) = bindings.get_manga_list(0).await?;
    /// ```
    pub async fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Config::default();
        config.async_support(true);

        let engine = Engine::new(&config).unwrap();

        let mut store = Store::new(&engine, State::default());

        let component = Component::from_file(&engine, path.as_ref())?;

        let mut linker: Linker<State> = Linker::new(&engine);
        map_midoku_http(&mut linker)?;
        map_midoku_limiter(&mut linker)?;
        map_midoku_settings(&mut linker)?;

        let instance = linker.instantiate_async(&mut store, &component).await?;

        let api = instance
            .get_export(&mut store, None, "midoku:bindings/api@0.1.0")
            .ok_or("export not found")?;

        let initialize = get_typed_func!(instance, store, api, "initialize")?;
        let get_manga_list = get_typed_func!(instance, store, api, "get-manga-list")?;
        let get_manga_details = get_typed_func!(instance, store, api, "get-manga-details")?;
        let get_chapter_list = get_typed_func!(instance, store, api, "get-chapter-list")?;
        let get_page_list = get_typed_func!(instance, store, api, "get-page-list")?;

        Ok(Self {
            store: Arc::new(RwLock::new(store)),
            initialize,
            get_manga_list,
            get_manga_details,
            get_chapter_list,
            get_page_list,
        })
    }

    /// Initialize the bindings instance.
    ///
    /// Sources may have initialization logic that needs to be called before
    /// calling other functions. This may include setting up rate limiters or
    /// other configuration.
    pub async fn initialize(&self) -> Result<(), ()> {
        self.initialize.execute(self.store.clone(), ()).await?
    }

    /// Get a list of manga from the source.
    ///
    /// # Arguments
    ///
    /// * `filters` - A list of filters to apply to the manga list.
    /// * `page` - The page number to get.
    pub async fn get_manga_list(
        &self,
        filters: Vec<Filter>,
        page: u32,
    ) -> Result<(Vec<Manga>, bool), ()> {
        self.get_manga_list
            .execute(self.store.clone(), (filters, page))
            .await?
    }

    /// Get details for a specific manga.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the manga to get details for.
    pub async fn get_manga_details(&self, id: String) -> Result<Manga, ()> {
        self.get_manga_details
            .execute(self.store.clone(), (id,))
            .await?
    }

    /// Get a list of chapters for a specific manga.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the manga to get chapters for.
    pub async fn get_chapter_list(&self, id: String) -> Result<Vec<Chapter>, ()> {
        self.get_chapter_list
            .execute(self.store.clone(), (id,))
            .await?
    }

    /// Get a list of pages for a specific chapter.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the manga.
    /// * `chapter_id` - The ID of the chapter.
    pub async fn get_page_list(&self, id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        self.get_page_list
            .execute(self.store.clone(), (id, chapter_id))
            .await?
    }

    /// Get a reference to the settings
    pub async fn settings(&self) -> RwLockReadGuard<'_, HashMap<String, Value>> {
        RwLockReadGuard::map(self.store.read().await, |store| store.data().settings())
    }

    /// Get a mutable reference to the settings.
    ///
    /// This allow modifying settings for the component (e.g. User-Agent, etc.).
    ///
    /// # Example
    ///
    /// ```ignore
    /// bindings.settings_mut().await.insert(
    ///     "key".to_string(),
    ///     Value::String("value".to_string())
    /// );
    /// ```
    pub async fn settings_mut(&mut self) -> RwLockMappedWriteGuard<'_, HashMap<String, Value>> {
        RwLockWriteGuard::map(self.store.write().await, |store| {
            store.data_mut().settings_mut()
        })
    }
}
