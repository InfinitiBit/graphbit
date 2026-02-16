//! LLM-driven fact extraction and consolidation logic.

use crate::errors::{GraphBitError, GraphBitResult};
use crate::llm::{LlmMessage, LlmProviderTrait, LlmRequest};

use super::types::{Memory, MemoryAction, MemoryDecision};

/// Handles sending conversations to an LLM for fact extraction and
/// deciding how new facts relate to existing memories.
pub struct MemoryProcessor {
    llm_provider: Box<dyn LlmProviderTrait>,
    max_tokens: u32,
    temperature: f32,
}

impl MemoryProcessor {
    /// Create a new processor wrapping the given LLM provider.
    pub fn new(
        llm_provider: Box<dyn LlmProviderTrait>,
        max_tokens: u32,
        temperature: f32,
    ) -> Self {
        Self {
            llm_provider,
            max_tokens,
            temperature,
        }
    }

    /// Extract discrete facts from a list of conversation messages.
    ///
    /// Returns a `Vec<String>` of facts parsed from the LLM's JSON response.
    pub async fn extract_facts(&self, messages: &[LlmMessage]) -> GraphBitResult<Vec<String>> {
        if messages.is_empty() {
            return Ok(Vec::new());
        }

        let conversation = messages
            .iter()
            .map(|m| format!("{}: {}", role_label(&m.role), &m.content))
            .collect::<Vec<_>>()
            .join("\n");

        let system_prompt = concat!(
            "You are a memory extraction assistant. Your task is to extract important facts, ",
            "preferences, and information from the conversation that would be useful to remember ",
            "for future interactions.\n\n",
            "Rules:\n",
            "- Extract only factual, specific information (not greetings or filler).\n",
            "- Each fact should be a single, self-contained sentence.\n",
            "- Do not duplicate facts.\n",
            "- If no meaningful facts exist, return an empty array.\n\n",
            "Return a JSON array of strings. Example: [\"User lives in Munich\", \"User prefers dark mode\"]",
        );

        let request = LlmRequest::with_messages(vec![
            LlmMessage::system(system_prompt),
            LlmMessage::user(format!("Extract facts from this conversation:\n\n{conversation}")),
        ])
        .with_max_tokens(self.max_tokens)
        .with_temperature(self.temperature);

        let response = self.llm_provider.complete(request).await.map_err(|e| {
            GraphBitError::memory(format!("Fact extraction LLM call failed: {e}"))
        })?;

        parse_json_string_array(&response.content)
    }

    /// Given extracted facts and existing memories, ask the LLM to decide
    /// whether each fact should be added, used to update an existing memory,
    /// delete an existing memory, or be ignored.
    pub async fn decide_actions(
        &self,
        facts: &[String],
        existing_memories: &[Memory],
    ) -> GraphBitResult<Vec<MemoryDecision>> {
        if facts.is_empty() {
            return Ok(Vec::new());
        }

        let facts_list = facts
            .iter()
            .enumerate()
            .map(|(i, f)| format!("{}. {f}", i + 1))
            .collect::<Vec<_>>()
            .join("\n");

        let memories_list = if existing_memories.is_empty() {
            "No existing memories.".to_string()
        } else {
            existing_memories
                .iter()
                .map(|m| format!("ID: {} | Content: {}", m.id, m.content))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let system_prompt = concat!(
            "You are a memory management assistant. Given new facts and existing memories, ",
            "decide what action to take for each fact.\n\n",
            "Actions:\n",
            "- ADD: The fact is new information not captured by any existing memory.\n",
            "- UPDATE: The fact refines or corrects an existing memory. Provide the target memory ID.\n",
            "- DELETE: The fact contradicts or invalidates an existing memory. Provide the target memory ID.\n",
            "- NOOP: The fact is already captured or is not worth storing.\n\n",
            "Return a JSON array of objects with keys: \"fact\", \"action\", \"target_memory_id\" (null if ADD/NOOP).\n",
            "Example: [{\"fact\":\"User lives in Berlin\",\"action\":\"UPDATE\",\"target_memory_id\":\"<uuid>\"}]",
        );

        let request = LlmRequest::with_messages(vec![
            LlmMessage::system(system_prompt),
            LlmMessage::user(format!(
                "New facts:\n{facts_list}\n\nExisting memories:\n{memories_list}"
            )),
        ])
        .with_max_tokens(self.max_tokens)
        .with_temperature(self.temperature);

        let response = self.llm_provider.complete(request).await.map_err(|e| {
            GraphBitError::memory(format!("Decision LLM call failed: {e}"))
        })?;

        parse_decisions(&response.content)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn role_label(role: &crate::llm::LlmRole) -> &'static str {
    match role {
        crate::llm::LlmRole::User => "User",
        crate::llm::LlmRole::Assistant => "Assistant",
        crate::llm::LlmRole::System => "System",
        crate::llm::LlmRole::Tool => "Tool",
    }
}

/// Parse a JSON array of strings from potentially messy LLM output.
fn parse_json_string_array(text: &str) -> GraphBitResult<Vec<String>> {
    // Try to find the JSON array in the response.
    let trimmed = text.trim();

    // First try direct parse.
    if let Ok(arr) = serde_json::from_str::<Vec<String>>(trimmed) {
        return Ok(arr);
    }

    // Try to extract the first JSON array from the text.
    if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            let slice = &trimmed[start..=end];
            if let Ok(arr) = serde_json::from_str::<Vec<String>>(slice) {
                return Ok(arr);
            }
        }
    }

    // Fallback: return empty array if we can't parse.
    Ok(Vec::new())
}

/// Parse the decision JSON from the LLM response.
fn parse_decisions(text: &str) -> GraphBitResult<Vec<MemoryDecision>> {
    let trimmed = text.trim();

    // Try to find and parse the JSON array.
    let json_str = if let Some(start) = trimmed.find('[') {
        if let Some(end) = trimmed.rfind(']') {
            &trimmed[start..=end]
        } else {
            trimmed
        }
    } else {
        trimmed
    };

    let raw: Vec<serde_json::Value> = serde_json::from_str(json_str).unwrap_or_default();

    let decisions = raw
        .into_iter()
        .filter_map(|v| {
            let fact = v.get("fact")?.as_str()?.to_string();
            let action_str = v.get("action")?.as_str()?;
            let action = MemoryAction::from_str_lossy(action_str);
            let target_memory_id = v
                .get("target_memory_id")
                .and_then(|t| t.as_str())
                .map(String::from);

            Some(MemoryDecision {
                fact,
                action,
                target_memory_id,
            })
        })
        .collect();

    Ok(decisions)
}
