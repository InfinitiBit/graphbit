//! Template variable resolution for workflows.

use lazy_static::lazy_static;
use regex::Regex;

use crate::types::WorkflowContext;

lazy_static! {
    static ref NODE_REF_PATTERN: Regex = Regex::new(r"\{\{node\.([a-zA-Z0-9_\-\.]+)\}\}").unwrap();
}

/// Resolve template variables in a string, supporting both node references and regular variables
pub fn resolve_template_variables(template: &str, context: &WorkflowContext) -> String {
    let mut result = template.to_string();

    // Replace node references like {{node.node_id}} or {{node.node_id.property}}
    for cap in NODE_REF_PATTERN.captures_iter(template) {
        if let Some(reference) = cap.get(1) {
            let reference = reference.as_str();
            if let Some(value) = context.get_nested_output(reference) {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    _ => value.to_string().trim_matches('"').to_string(),
                };
                result = result.replace(&cap[0], &value_str);
            }
        }
    }

    // Replace simple variables for backward compatibility
    for (key, value) in &context.variables {
        let placeholder = format!("{{{key}}}");
        if let Ok(value_str) = serde_json::to_string(value) {
            let value_str = value_str.trim_matches('"');
            result = result.replace(&placeholder, value_str);
        }
    }

    result
}
