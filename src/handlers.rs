use std::collections::HashMap;

use acap_vapix::HttpClient;
use anyhow::Context;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::header::CONTENT_TYPE,
    response::Response,
};
use log::debug;
use serde_json::{json, Value};

use crate::error::{AppError, Result};

pub async fn relay_request(
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(client): State<HttpClient>,
) -> Result<Response> {
    let body = parse_body(params)?;
    debug!("Relaying request to {path:?} with body {body:?}");
    let incming_response = client
        .post(&path)
        .context("Could not create request builder")
        .map_err(AppError::InvalidPath)?
        .replace_with(|b| b.json(&body))
        .send()
        .await
        .context("Could not send request")
        .map_err(AppError::Internal)?;

    let mut outgoing_response = Response::builder().status(incming_response.status());
    if let Some(content_type) = incming_response.headers().get(CONTENT_TYPE) {
        outgoing_response = outgoing_response.header(
            CONTENT_TYPE,
            content_type
                .to_str()
                .context("Could not convert header to string")
                .map_err(AppError::Internal)?,
        );
    }
    outgoing_response
        .body(Body::from(
            incming_response
                .bytes()
                .await
                .context("Could not read response body")
                .map_err(AppError::Internal)?,
        ))
        .context("Could not build response")
        .map_err(AppError::Internal)
}

fn parse_body(params: HashMap<String, String>) -> Result<Value> {
    let mut data = json!({});

    for (path, value) in params {
        let (path, value) = match path.strip_suffix(".") {
            None => (path.as_str(), Value::String(value)),
            Some(path) => (
                path,
                serde_json::from_str::<Value>(&value)
                    .with_context(|| format!("Could not parse value {value} at {path}"))
                    .map_err(AppError::Other)?,
            ),
        };

        insert(&mut data, path, value)?;
    }

    Ok(data)
}
fn insert(data: &mut Value, path: &str, value: Value) -> Result<()> {
    let mut current = data;
    let parts: Vec<&str> = path.split('.').collect();
    for (i, part) in parts.iter().enumerate() {
        let is_last = i == parts.len() - 1;

        if is_last {
            current
                .as_object_mut()
                .with_context(|| format!("Could not insert intermediate object for path {path}"))
                .map_err(AppError::InvalidQuery)?
                .insert(part.to_string(), value.clone());
        } else {
            current = current
                .as_object_mut()
                .with_context(|| format!("Could not insert intermediate object for path {path}"))
                .map_err(AppError::InvalidQuery)?
                .entry(part.to_string())
                .or_insert_with(|| json!({}));
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::*;

    #[test]
    fn can_parse() {
        let mut query = HashMap::new();
        query.insert("null.".to_string(), "null".to_string());
        query.insert("boolean.".to_string(), "true".to_string());
        query.insert("integer.".to_string(), "2".to_string());
        query.insert("decimal.".to_string(), "3.0".to_string());
        query.insert("string".to_string(), "4".to_string());
        query.insert("nested.string".to_string(), "5".to_string());
        query.insert("nested.array.".to_string(), "[6]".to_string());
        query.insert("nested.object.".to_string(), "{\"key\":7}".to_string());
        let actual = parse_body(query).unwrap();
        let expected = json!({
            "null": null,
            "boolean": true,
            "integer": 2,
            "decimal": 3.0,
            "string": "4",
            "nested": {
                "string": "5",
                "array": [6],
                "object": {"key": 7}
            }
        });
        assert_eq!(actual, expected);
    }
}
