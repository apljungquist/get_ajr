use std::collections::HashMap;

use acap_vapix::{HttpClient, HttpErrorKind};
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
};
use serde_json::{json, Value};

use crate::error::{AppError, Result};

pub async fn relay_request(
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(client): State<HttpClient>,
) -> Result<Response> {
    let body = parse_body(params)?;
    let res = client
        .post(&path)
        .map_err(|e| AppError::InvalidUrlPath(e.into()))?
        .replace_with(|b| b.json(&body))
        .send()
        .await;
    match res {
        Ok(v) => Ok((
            v.status(),
            v.text().await.map_err(|e| AppError::Internal(e.into()))?,
        )
            .into_response()),
        Err(e) => match e.kind() {
            HttpErrorKind::Other => Err(AppError::Other(e.into())),
            _ => Err(AppError::Internal(e.into())),
        },
    }
}

fn parse_body(params: HashMap<String, String>) -> Result<Value> {
    let mut json_data = json!({});

    for (json_path, value) in params {
        let parsed_value = parse_simple_json_value(&value);
        apply_json_path(&mut json_data, &json_path, parsed_value)?;
    }

    Ok(json_data)
}

fn parse_simple_json_value(value: &str) -> Value {
    if let Ok(num) = value.parse::<i64>() {
        return Value::Number(serde_json::Number::from(num));
    }

    if let Ok(float) = value.parse::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(float) {
            return Value::Number(num);
        }
    }

    match value.to_lowercase().as_str() {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        _ => Value::String(value.to_string()),
    }
}

fn apply_json_path(data: &mut Value, json_path: &str, value: Value) -> Result<()> {
    if json_path == "$" {
        *data = value;
        return Ok(());
    }

    let parts: Vec<&str> = json_path.split('.').collect();
    if parts.is_empty() || parts[0] != "$" {
        return Err(AppError::InvalidJsonPath);
    }

    let mut current = data;

    for (i, part) in parts[1..].iter().enumerate() {
        let is_last = i == parts.len() - 2;

        if current.is_null() {
            *current = json!({});
        }

        if !current.is_object() {
            return Err(AppError::InvalidJsonPath);
        }

        if is_last {
            current
                .as_object_mut()
                .unwrap()
                .insert(part.to_string(), value.clone());
        } else {
            current = current
                .as_object_mut()
                .unwrap()
                .entry(part.to_string())
                .or_insert_with(|| json!({}));
        }
    }

    Ok(())
}
