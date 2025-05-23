use super::types::*;
use rpc_router::HandlerResult;
use url::Url;

// TODO: replace this dummy result with actual resources
pub async fn resources_list(
    _request: Option<ListResourcesRequest>,
) -> HandlerResult<ListResourcesResult> {
    let response = ListResourcesResult {
        resources: vec![Resource {
            uri: Url::parse("file:///logs/app.log").unwrap(),
            name: "Application Logs".to_string(),
            description: None,
            mime_type: Some("text/plain".to_string()),
        }],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn resource_read(request: ReadResourceRequest) -> HandlerResult<ReadResourceResult> {
    let response = ReadResourceResult {
        content: ResourceContent {
            uri: request.uri.clone(),
            mime_type: Some("text/plain".to_string()),
            text: Some("2024-11-28T08:19:18.974368Z,INFO,main,this is message".to_string()),
            blob: None,
        },
    };
    Ok(response)
}
