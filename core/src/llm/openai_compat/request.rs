use crate::errors::{GraphBitError, GraphBitResult};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Serialize a request body and merge provider-specific extra params.
pub(crate) fn build_request_json_with_extra_params<T: Serialize>(
    provider_name: &str,
    body: &T,
    extra_params: HashMap<String, Value>,
) -> GraphBitResult<Value> {
    let mut request_json = serde_json::to_value(body)?;
    match request_json {
        Value::Object(ref mut map) => {
            for (key, value) in extra_params {
                map.insert(key, value);
            }
            Ok(request_json)
        }
        _ => Err(GraphBitError::llm_provider(
            provider_name,
            "Serialized request body is not a JSON object",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Serialize)]
    struct DummyBody {
        model: String,
        temperature: f32,
    }

    #[derive(Serialize)]
    struct NonObjectBody(i32);

    #[test]
    fn merges_extra_params_and_overrides_existing_keys() {
        let body = DummyBody {
            model: "gpt-4o-mini".to_string(),
            temperature: 0.2,
        };
        let mut extras = HashMap::new();
        extras.insert("temperature".to_string(), json!(0.9));
        extras.insert("max_completion_tokens".to_string(), json!(256));

        let value = build_request_json_with_extra_params("openai", &body, extras)
            .expect("request merge should succeed");

        assert_eq!(value["model"], "gpt-4o-mini");
        assert_eq!(value["temperature"], 0.9);
        assert_eq!(value["max_completion_tokens"], 256);
    }

    #[test]
    fn rejects_non_object_serialized_body() {
        let body = NonObjectBody(5);
        let result = build_request_json_with_extra_params("openai", &body, HashMap::new());
        assert!(result.is_err());
    }
}
