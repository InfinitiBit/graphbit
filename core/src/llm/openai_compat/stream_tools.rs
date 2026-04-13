use crate::llm::LlmToolCall;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// Streamed function delta for one tool call.
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct StreamToolFunctionDelta {
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) arguments: Option<String>,
}

/// Streamed tool-call delta (OpenAI-shape).
#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct StreamToolCallDelta {
    #[serde(default)]
    pub(crate) index: Option<u32>,
    #[serde(default)]
    pub(crate) id: Option<String>,
    #[serde(default)]
    pub(crate) function: Option<StreamToolFunctionDelta>,
}

/// Accumulated partial state per tool-call slot.
#[derive(Debug, Clone, Default)]
pub(crate) struct StreamToolCallAccum {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) arguments: String,
}

/// Merge streamed tool-call deltas by index.
pub(crate) fn merge_stream_tool_call_deltas(
    acc: &mut HashMap<u32, StreamToolCallAccum>,
    deltas: &[StreamToolCallDelta],
) {
    for delta in deltas {
        let idx = delta.index.unwrap_or(0);
        let entry = acc.entry(idx).or_default();
        if let Some(id) = &delta.id {
            if !id.is_empty() {
                entry.id.clone_from(id);
            }
        }
        if let Some(function) = &delta.function {
            if let Some(name) = &function.name {
                if !name.is_empty() {
                    entry.name.clone_from(name);
                }
            }
            if let Some(arguments) = &function.arguments {
                entry.arguments.push_str(arguments);
            }
        }
    }
}

/// Render incremental assistant text fragments for tool-call stream deltas.
pub(crate) fn render_stream_tool_call_delta_fragment(
    deltas: &[StreamToolCallDelta],
    acc: &HashMap<u32, StreamToolCallAccum>,
    announced: &mut HashSet<u32>,
) -> String {
    let mut out = String::new();
    for delta in deltas {
        let idx = delta.index.unwrap_or(0);
        if !announced.contains(&idx) {
            let name = delta
                .function
                .as_ref()
                .and_then(|f| f.name.as_deref())
                .or_else(|| acc.get(&idx).map(|t| t.name.as_str()))
                .unwrap_or("tool");
            out.push_str(&format!("[tool_call:{name}] "));
            announced.insert(idx);
        }
        if let Some(arguments) = delta.function.as_ref().and_then(|f| f.arguments.as_deref()) {
            out.push_str(arguments);
        }
    }
    out
}

/// Convert accumulated streamed tool-call state into final [`LlmToolCall`]s.
pub(crate) fn stream_tool_accum_to_llm_calls(
    acc: &HashMap<u32, StreamToolCallAccum>,
) -> Vec<LlmToolCall> {
    let mut pairs: Vec<(u32, &StreamToolCallAccum)> = acc.iter().map(|(i, t)| (*i, t)).collect();
    pairs.sort_by_key(|(idx, _)| *idx);
    pairs
        .into_iter()
        .map(|(_, tool)| {
            let parameters = if tool.arguments.trim().is_empty() {
                serde_json::Value::Object(serde_json::Map::new())
            } else {
                match serde_json::from_str(&tool.arguments) {
                    Ok(value) => value,
                    Err(e) => {
                        tracing::warn!(
                            "Failed to parse streamed tool call arguments for '{}': {e}",
                            tool.name
                        );
                        serde_json::json!({ "raw_arguments": tool.arguments })
                    }
                }
            };
            LlmToolCall {
                id: tool.id.clone(),
                name: tool.name.clone(),
                parameters,
            }
        })
        .collect()
}

/// Keep text content unchanged while tool-call details flow via `tool_calls`.
pub(crate) fn assistant_text_for_tool_calls(content: String, _tool_calls: &[LlmToolCall]) -> String {
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_and_materializes_streamed_tool_deltas() {
        let chunk1 = vec![StreamToolCallDelta {
            index: Some(0),
            id: Some("call_abc".to_string()),
            function: Some(StreamToolFunctionDelta {
                name: Some("add".to_string()),
                arguments: Some("{\"a\": 1".to_string()),
            }),
        }];
        let chunk2 = vec![StreamToolCallDelta {
            index: Some(0),
            id: None,
            function: Some(StreamToolFunctionDelta {
                name: None,
                arguments: Some(", \"b\": 2}".to_string()),
            }),
        }];
        let mut acc = HashMap::new();
        merge_stream_tool_call_deltas(&mut acc, &chunk1);
        merge_stream_tool_call_deltas(&mut acc, &chunk2);
        let calls = stream_tool_accum_to_llm_calls(&acc);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].id, "call_abc");
        assert_eq!(calls[0].name, "add");
        assert_eq!(calls[0].parameters["a"], 1);
        assert_eq!(calls[0].parameters["b"], 2);
    }

    #[test]
    fn renders_fragment_with_single_header_per_tool() {
        let deltas = vec![
            StreamToolCallDelta {
                index: Some(0),
                id: Some("call_1".to_string()),
                function: Some(StreamToolFunctionDelta {
                    name: Some("search".to_string()),
                    arguments: Some("{\"q\":\"gr".to_string()),
                }),
            },
            StreamToolCallDelta {
                index: Some(0),
                id: None,
                function: Some(StreamToolFunctionDelta {
                    name: None,
                    arguments: Some("aphbit\"}".to_string()),
                }),
            },
        ];

        let mut acc = HashMap::new();
        merge_stream_tool_call_deltas(&mut acc, &deltas);
        let mut announced = HashSet::new();
        let out = render_stream_tool_call_delta_fragment(&deltas, &acc, &mut announced);
        assert!(out.contains("[tool_call:search]"));
        assert!(out.contains("{\"q\":\"graphbit\"}"));
        assert_eq!(announced.len(), 1);
    }
}
