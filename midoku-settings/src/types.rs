use wasmtime::component::{ComponentType, Lift, Lower};

#[derive(ComponentType, Lift, Lower, Debug, Clone, Copy, PartialEq)]
#[component(variant)]
pub enum Number {
    #[component(name = "s64")]
    S64(i64),
    #[component(name = "u64")]
    U64(u64),
    #[component(name = "f64")]
    F64(f64),
}

#[derive(ComponentType, Lift, Lower, Debug, Clone, PartialEq)]
#[component(variant)]
pub enum Value {
    #[component(name = "bool")]
    Bool(bool),
    #[component(name = "number")]
    Number(Number),
    #[component(name = "string")]
    String(String),
    #[component(name = "array")]
    Array(Vec<String>),
    #[component(name = "map")]
    Map(Vec<(String, String)>),
}
