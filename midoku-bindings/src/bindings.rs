use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::path::Path;

use midoku_settings::types::Value;
use midoku_types::chapter::Chapter;
use midoku_types::filter::Filter;
use midoku_types::manga::Manga;
use midoku_types::page::Page;
use wasmtime::component::{Component, Linker, TypedFunc};
use wasmtime::{AsContextMut, Engine, Store};

use crate::instance_impl::midoku_http::map_midoku_http;
use crate::instance_impl::midoku_limiter::map_midoku_limiter;
use crate::instance_impl::midoku_settings::map_midoku_settings;
use crate::state::State;

pub struct Bindings {
    store: RefCell<Store<State>>,
    initialize: TypedFunc<(), (Result<(), ()>,)>,
    get_manga_list: TypedFunc<(Vec<Filter>, u32), (Result<(Vec<Manga>, bool), ()>,)>,
    get_manga_details: TypedFunc<(String,), (Result<Manga, ()>,)>,
    get_chapter_list: TypedFunc<(String,), (Result<Vec<Chapter>, ()>,)>,
    get_page_list: TypedFunc<(String, String), (Result<Vec<Page>, ()>,)>,
}

/// Macro to get the mutable context from the store.
#[doc(hidden)]
macro_rules! store_context_mut {
    ($self:expr) => {
        $self.store.borrow_mut().as_context_mut()
    };
}

/// Macro to call a function and get the result.
#[doc(hidden)]
macro_rules! call_func {
    ($self:expr, $func:ident, $args:expr) => {
        $self
            .$func
            .call(&mut store_context_mut!($self), $args)
            .map_err(|_| ())?
            .0
    };
}

/// Macro to clean up after calling a function.
#[doc(hidden)]
macro_rules! post_return {
    ($self:expr, $func:ident) => {
        $self
            .$func
            .post_return(&mut store_context_mut!($self))
            .map_err(|_| ())?;
    };
}

impl Bindings {
    /// Create a new instance of the bindings from a WebAssembly component
    /// .wasm file.
    ///
    /// # Example
    /// ```no_run
    /// let bindings = Bindings::from_file("example.wasm")?;
    ///
    /// // Call the wasm `initialize` function which contains the initialization
    /// // logic, such as the rate limiter configuration.
    /// bindings.initialize()?;
    ///
    /// // Call the wasm `get_manga_list` function to get the list of manga.
    /// let (manga_list, has_next) = bindings.get_manga_list(0)?;
    /// ```
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = Engine::default();
        let mut store = Store::new(&engine, State::default());

        let component = Component::from_file(&engine, path.as_ref())?;

        let mut linker: Linker<State> = Linker::new(&engine);
        map_midoku_http(&mut linker)?;
        map_midoku_limiter(&mut linker)?;
        map_midoku_settings(&mut linker)?;

        let instance: wasmtime::component::Instance = linker.instantiate(&mut store, &component)?;

        let mut export = instance.exports(&mut store);
        let mut api = export
            .instance("midoku:bindings/api@0.1.0")
            .ok_or("export not found")?;

        let initialize = api.typed_func::<(), (Result<(), ()>,)>("initialize")?;
        let get_manga_list = api
            .typed_func::<(Vec<Filter>, u32), (Result<(Vec<Manga>, bool), ()>,)>(
                "get-manga-list",
            )?;
        let get_manga_details =
            api.typed_func::<(String,), (Result<Manga, ()>,)>("get-manga-details")?;
        let get_chapter_list =
            api.typed_func::<(String,), (Result<Vec<Chapter>, ()>,)>("get-chapter-list")?;
        let get_page_list =
            api.typed_func::<(String, String), (Result<Vec<Page>, ()>,)>("get-page-list")?;

        drop(export);

        Ok(Self {
            store: RefCell::new(store),
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
    pub fn initialize(&self) -> Result<(), ()> {
        let result_initialize = call_func!(self, initialize, ());
        post_return!(self, initialize);
        result_initialize
    }

    pub fn get_manga_list(&self, filter: Vec<Filter>, page: u32) -> Result<(Vec<Manga>, bool), ()> {
        let result_get_manga_list = call_func!(self, get_manga_list, (filter, page,));
        post_return!(self, get_manga_list);
        result_get_manga_list
    }

    pub fn get_manga_details(&self, id: String) -> Result<Manga, ()> {
        let result_get_manga_details = call_func!(self, get_manga_details, (id,));
        post_return!(self, get_manga_details);
        result_get_manga_details
    }

    pub fn get_chapter_list(&self, id: String) -> Result<Vec<Chapter>, ()> {
        let result_get_chapter_list = call_func!(self, get_chapter_list, (id,));
        post_return!(self, get_chapter_list);
        result_get_chapter_list
    }

    pub fn get_page_list(&self, id: String, chapter_id: String) -> Result<Vec<Page>, ()> {
        let result_get_page_list = call_func!(self, get_page_list, (id, chapter_id));
        post_return!(self, get_page_list);
        result_get_page_list
    }

    /// Get a reference to the settings.
    pub fn settings(&self) -> Ref<HashMap<String, Value>> {
        Ref::map(self.store.borrow(), |s| s.data().settings())
    }

    /// Get a mutable reference to the settings.
    pub fn settings_mut(&mut self) -> RefMut<HashMap<String, Value>> {
        RefMut::map(self.store.borrow_mut(), |s| s.data_mut().settings_mut())
    }
}
