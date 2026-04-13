use crate::errors::{GraphBitError, GraphBitResult};

/// Parsed SSE payload line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SseDataLine {
    Json(String),
    Done,
}

/// Incremental SSE line buffer for `data: ...` style streams.
#[derive(Debug, Default)]
pub(crate) struct SseLineBuffer {
    buffer: String,
}

impl SseLineBuffer {
    /// Append an incoming byte chunk.
    pub(crate) fn push_chunk(&mut self, bytes: &[u8]) -> GraphBitResult<()> {
        let chunk = std::str::from_utf8(bytes).map_err(|e| {
            GraphBitError::llm_provider("openai_compat", format!("SSE chunk UTF-8 decode failed: {e}"))
        })?;
        self.buffer.push_str(chunk);
        Ok(())
    }

    /// Drain fully-delimited lines into parsed `data` payloads.
    pub(crate) fn drain_data_lines(&mut self) -> Vec<SseDataLine> {
        let mut out = Vec::new();
        while let Some(pos) = self.buffer.find('\n') {
            let line = self.buffer[..pos].trim_end_matches('\r').trim().to_string();
            self.buffer.drain(..=pos);
            if line.is_empty() || line.starts_with(':') {
                continue;
            }
            if let Some(payload) = line.strip_prefix("data:") {
                let payload = payload.trim_start();
                if payload == "[DONE]" {
                    out.push(SseDataLine::Done);
                } else if !payload.is_empty() {
                    out.push(SseDataLine::Json(payload.to_string()));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drains_json_lines_and_done_marker() {
        let mut buf = SseLineBuffer::default();
        buf.push_chunk(b"data: {\"a\":1}\n").unwrap();
        buf.push_chunk(b": keepalive\n").unwrap();
        buf.push_chunk(b"data: [DONE]\n").unwrap();

        let lines = buf.drain_data_lines();
        assert_eq!(
            lines,
            vec![SseDataLine::Json("{\"a\":1}".to_string()), SseDataLine::Done]
        );
    }

    #[test]
    fn handles_fragmented_chunks() {
        let mut buf = SseLineBuffer::default();
        buf.push_chunk(b"data: {\"part\"").unwrap();
        assert!(buf.drain_data_lines().is_empty());
        buf.push_chunk(b": 1}\n").unwrap();

        let lines = buf.drain_data_lines();
        assert_eq!(lines, vec![SseDataLine::Json("{\"part\": 1}".to_string())]);
    }

    #[test]
    fn rejects_invalid_utf8() {
        let mut buf = SseLineBuffer::default();
        let result = buf.push_chunk(&[0xF0, 0x28, 0x8C, 0x28]);
        assert!(result.is_err());
    }
}
