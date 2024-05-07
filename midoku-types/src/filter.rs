use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(record)]
pub struct FilterTitle {
    pub query: String,
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(record)]
pub struct FilterSort {
    pub name: String,
    #[component(name = "can-be-reversed")]
    pub can_be_reversed: bool,
    pub options: Vec<String>,
    #[component(name = "default-option-index")]
    pub default_option_index: u32,
    #[component(name = "default-option-reversed")]
    pub default_option_reversed: bool,
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(variant)]
pub enum Filter {
    #[component(name = "title")]
    Title(FilterTitle),
    #[component(name = "sort")]
    Sort(FilterSort),
}
