use midoku_http::outgoing_handler::handle;
use midoku_http::types::{IncomingResponse, Method};
use wasmtime::component::{Linker, LinkerInstance, Resource, ResourceType};

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

struct HostIncomingResponse;

impl HostIncomingResponse {
    fn destructor(
        mut store: wasmtime::StoreContextMut<State>,
        resource_rep: u32,
    ) -> Result<(), wasmtime::Error> {
        let incoming_response: Resource<IncomingResponse> = Resource::new_own(resource_rep);
        store
            .data_mut()
            .resource_table_mut()
            .delete(incoming_response)?;
        Ok(())
    }

    fn status_code(
        store: wasmtime::StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(u16,), wasmtime::Error> {
        let incoming_response: &IncomingResponse = store.data().resource_table().get(&resource)?;
        let status_code: u16 = incoming_response.status_code();
        Ok((status_code,))
    }

    fn headers(
        store: wasmtime::StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(Vec<(String, String)>,), wasmtime::Error> {
        let incoming_response: &IncomingResponse = store.data().resource_table().get(&resource)?;
        let headers: Vec<(String, String)> = incoming_response.headers().clone();
        Ok((headers,))
    }

    fn bytes(
        store: wasmtime::StoreContextMut<State>,
        (resource,): (Resource<IncomingResponse>,),
    ) -> Result<(Vec<u8>,), wasmtime::Error> {
        let incoming_response: &IncomingResponse = store.data().resource_table().get(&resource)?;
        let bytes: Vec<u8> = incoming_response.bytes().clone();
        Ok((bytes,))
    }
}

fn host_handle(
    mut store: wasmtime::StoreContextMut<State>,
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

    let incoming_response_resource: Resource<IncomingResponse> = store
        .data_mut()
        .resource_table_mut()
        .push(incoming_response.unwrap())?;
    let result: Result<Resource<IncomingResponse>, ()> = Ok(incoming_response_resource);
    Ok((result,))
}
