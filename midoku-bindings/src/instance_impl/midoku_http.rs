use midoku_http::outgoing_handler::handle;
use midoku_http::types::{IncomingResponse, Method};
use wasmtime::component::{Linker, LinkerInstance, Resource, ResourceType};

use crate::state::State;

pub fn map_midoku_http(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut midoku_http_types_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:http/types@0.1.0")?;
    map_midoku_http_types(&mut midoku_http_types_instance)?;

    let mut midoku_http_outgoing_handler_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:http/outgoing-handler@0.1.0")?;
    map_midoku_http_outgoing_handler(&mut midoku_http_outgoing_handler_instance)?;

    Ok(())
}

fn map_midoku_http_types(instance: &mut LinkerInstance<'_, State>) -> Result<(), Box<dyn std::error::Error>> {
    instance.resource(
        "incoming-response",
        ResourceType::host::<IncomingResponse>(),
        |mut store, rep| {
            let resource: Resource<IncomingResponse> = Resource::new_own(rep);
            store.data_mut().resource_table_mut().delete(resource)?;
            Ok(())
        },
    )?;
    instance.func_wrap(
        "[method]incoming-response.status-code",
        |store, (self_,): (Resource<IncomingResponse>,)| {
            let incoming_response: &IncomingResponse = store.data().resource_table().get(&self_)?;
            let status_code: u16 = incoming_response.status_code();
            Ok((status_code,))
        },
    )?;
    instance.func_wrap(
        "[method]incoming-response.headers",
        |store, (self_,): (Resource<IncomingResponse>,)| {
            let incoming_response: &IncomingResponse = store.data().resource_table().get(&self_)?;
            let headers: Vec<(String, String)> = incoming_response.headers().clone();
            Ok((headers,))
        },
    )?;
    instance.func_wrap(
        "[method]incoming-response.bytes",
        |store, (self_,): (Resource<IncomingResponse>,)| {
            let incoming_response: &IncomingResponse = store.data().resource_table().get(&self_)?;
            let bytes: Vec<u8> = incoming_response.bytes().clone();
            Ok((bytes,))
        },
    )?;
    Ok(())
}

fn map_midoku_http_outgoing_handler(
    instance: &mut LinkerInstance<'_, State>,
) -> Result<(), Box<dyn std::error::Error>> {
    instance.func_wrap(
        "handle",
        |mut store,
         (method, url, headers, body): (
            Method,
            Box<str>,
            Option<Box<[(String, String)]>>,
            Option<Box<[u8]>>,
        )| {
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
        },
    )?;
    Ok(())
}
