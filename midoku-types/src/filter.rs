use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(record)]
pub struct FilterTitle {
    pub query: String,
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(record)]
pub struct FilterSort {
    #[component(name = "option-index")]
    pub option_index: u32,

    /// If true, the sort order is reversed.
    ///
    /// This is analogous to an ascending if true or descending if false sort
    /// order.
    #[component(name = "option-reversed")]
    pub option_reversed: bool,
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(variant)]
pub enum Filter {
    #[component(name = "title")]
    Title(FilterTitle),
    #[component(name = "sort")]
    Sort(FilterSort),
}
