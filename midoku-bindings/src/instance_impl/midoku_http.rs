use midoku_http::outgoing_handler::handle;
use midoku_http::types::{IncomingResponse, Method};
use wasmtime::component::{Linker, LinkerInstance, Resource, ResourceType};
use wasmtime::StoreContextMut;

use crate::state::State;

pub fn map_midoku_http(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut types_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:http/types@0.1.0")?;
    types_instance.resource(
        "incoming-response",
        ResourceType::host::<IncomingResponse>(),
        HostIncomingResponse::destructor,
    )?;
    types_instance.func_wrap(
        "[method]incoming-response.status-code",
        HostIncomingResponse::status_code,
    )?;
    types_instance.func_wrap(
        "[method]incoming-response.headers",
        HostIncomingResponse::headers,
    )?;
    types_instance.func_wrap(
        "[method]incoming-response.bytes",
        HostIncomingResponse::bytes,
    )?;

    let mut outgoing_handler_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:http/outgoing-handler@0.1.0")?;
    outgoing_handler_instance.func_wrap("handle", host_handle)?;

    Ok(())
}

/// Helper macro to get a resource from the resource table in the store.
#[doc(hidden)]
macro_rules! resource_table_get {
    ($store:expr, $resource:expr) => {
        $store.data().resource_table().get(&$resource)
    };
}

/// Helper macro to push a resource to the resource table in the store.
#[doc(hidden)]
macro_rules! resource_table_push {
    ($store:expr, $resource:expr) => {
        $store.data_mut().resource_table_mut().push($resource)
    };
}

/// Helper macro to delete a resource from the resource table in the store.
#[doc(hidden)]
macro_rules! resource_table_delete {
    ($store:expr, $rep:expr) => {
        $store.data_mut().resource_table_mut().delete($rep)
    };
}

/// Host functions implementation for the `incoming response` resource.
struct HostIncomingResponse;

impl HostIncomingResponse {
    /// Host function implementation for the `destructor` behavior of the
    /// `IncomingResponse` resource.
    fn destructor(
        mut store: StoreContextMut<State>,
        resource_rep: u32,
    ) -> Result<(), wasmtime::Error> {
        let incoming_response: Resource<IncomingResponse> = Resource::new_own(resource_rep);
        resource_table_delete!(store, incoming_response)?;
        Ok(())
    }

    /// Host function implementation for the `status_code` method of the
    /// `IncomingResponse` resource.
    fn status_code(
        store: StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(u16,), wasmtime::Error> {
        let incoming_response = resource_table_get!(store, resource)?;
        Ok((incoming_response.status_code(),))
    }

    /// Host function implementation for the `headers` method of the
    /// `IncomingResponse` resource.
    fn headers(
        store: StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(Vec<(String, String)>,), wasmtime::Error> {
        let incoming_response = resource_table_get!(store, resource)?;
        Ok((incoming_response.headers().clone(),))
    }

    /// Host function implementation for the `bytes` method of the
    /// `IncomingResponse` resource.
    fn bytes(
        store: StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(Vec<u8>,), wasmtime::Error> {
        let incoming_response = resource_table_get!(store, resource)?;
        Ok((incoming_response.bytes().clone(),))
    }
}

/// Host function implementation for the `handle` function.
fn host_handle(
    mut store: StoreContextMut<State>,
    (method, url, headers, body): (
        Method,
        Box<str>,
        Option<Box<[(String, String)]>>,
        Option<Box<[u8]>>,
    ),
) -> Result<(Result<Resource<IncomingResponse>, ()>,), wasmtime::Error> {
    let headers = headers.map(|headers| headers.to_vec());
    let body = body.map(|body| body.to_vec());

    let incoming_response = handle(method, url.to_string(), headers, body);
    if incoming_response.is_err() {
        return Ok((Err(()),));
    }

    let incoming_response_resource = resource_table_push!(store, incoming_response.unwrap())?;
    Ok((Ok(incoming_response_resource),))
}
