//! GuardRail FFI wrapper — links the prebuilt `libguardrail_ffi.a` only (no guardrail source).

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};
use std::path::Path;
use std::sync::Arc;

unsafe extern "C" {
    fn guardrail_config_from_json(json_ptr: *const c_char, json_len: usize) -> *mut std::ffi::c_void;
    fn guardrail_config_default() -> *mut std::ffi::c_void;
    fn guardrail_config_clone(handle: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn guardrail_config_drop(handle: *mut std::ffi::c_void);
    fn guardrail_config_policy_name(handle: *mut std::ffi::c_void) -> *mut c_char;
    fn guardrail_config_policy_version(handle: *mut std::ffi::c_void) -> *mut c_char;
    fn guardrail_config_active(handle: *mut std::ffi::c_void) -> bool;

    fn guardrail_enforcer_create(
        config_handle: *mut std::ffi::c_void,
        workflow_id_ptr: *const c_char,
        workflow_id_len: usize,
    ) -> *mut std::ffi::c_void;
    fn guardrail_enforcer_drop(handle: *mut std::ffi::c_void);

    fn guardrail_encode(
        enforcer_handle: *mut std::ffi::c_void,
        json_ptr: *const c_char,
        json_len: usize,
        encode_context: c_uint,
    ) -> *mut c_char;
    fn guardrail_decode(
        enforcer_handle: *mut std::ffi::c_void,
        json_ptr: *const c_char,
        json_len: usize,
        context: c_uint,
    ) -> *mut c_char;
    fn guardrail_free(ptr: *mut c_char);
}

const CONTEXT_TOOL_BOUNDARY: c_uint = 0;
const CONTEXT_LLM_RESPONSE: c_uint = 1;
const CONTEXT_MANUAL_CALL: c_uint = 2;

/// Encode context: Llm adds signature and instruction text; Manual does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeContext {
    /// No signature or instruction (e.g. manual / logging).
    Manual,
    /// Add 3-digit signature to tokens and prepend instruction text for LLM.
    Llm,
}

const ENCODE_CONTEXT_MANUAL: c_uint = 0;
const ENCODE_CONTEXT_LLM: c_uint = 1;

/// Decode context for rehydration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeContext {
    /// Rehydrate at tool boundary so the tool receives real PII.
    ToolBoundary,
    /// Rehydrate LLM output for context.
    LlmResponse,
    /// Explicit host decode.
    ManualCall,
}

/// Opaque config handle (refcounted via clone/drop).
pub struct GuardRailConfigInner {
    pub(crate) handle: *mut std::ffi::c_void,
}

impl Drop for GuardRailConfigInner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { guardrail_config_drop(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Clone for GuardRailConfigInner {
    fn clone(&self) -> Self {
        let handle = if self.handle.is_null() {
            std::ptr::null_mut()
        } else {
            unsafe { guardrail_config_clone(self.handle) }
        };
        Self { handle }
    }
}

// Opaque FFI handle: safe to Send/Sync as the C library manages thread safety.
unsafe impl Send for GuardRailConfigInner {}
unsafe impl Sync for GuardRailConfigInner {}

/// GuardRail policy configuration.
#[derive(Clone)]
pub struct GuardRailConfig {
    pub(crate) inner: Arc<GuardRailConfigInner>,
}

impl GuardRailConfig {
    /// Load from JSON string.
    pub fn new(json: &str) -> Result<Self, String> {
        let c_str = CString::new(json).map_err(|e| e.to_string())?;
        let handle = unsafe { guardrail_config_from_json(c_str.as_ptr(), c_str.as_bytes().len()) };
        if handle.is_null() {
            return Err("GuardRail config from_json failed".into());
        }
        Ok(Self {
            inner: Arc::new(GuardRailConfigInner { handle }),
        })
    }

    /// Load from file.
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::new(&json)
    }

    /// Load from URL (blocking GET).
    pub fn from_url(url: &str) -> Result<Self, String> {
        let client = reqwest::blocking::Client::new();
        let json = client
            .get(url)
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
        Self::new(&json)
    }

    /// Default inactive config.
    pub fn default_config() -> Self {
        let handle = unsafe { guardrail_config_default() };
        assert!(!handle.is_null(), "guardrail_config_default failed");
        Self {
            inner: Arc::new(GuardRailConfigInner { handle }),
        }
    }

    pub(crate) fn ptr(&self) -> *mut std::ffi::c_void {
        self.inner.handle
    }

    /// Policy name.
    pub fn policy_name(&self) -> String {
        if self.ptr().is_null() {
            return String::new();
        }
        let p = unsafe { guardrail_config_policy_name(self.ptr()) };
        if p.is_null() {
            return String::new();
        }
        let s = unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() };
        unsafe { guardrail_free(p) };
        s
    }

    /// Policy version.
    pub fn policy_version(&self) -> String {
        if self.ptr().is_null() {
            return String::new();
        }
        let p = unsafe { guardrail_config_policy_version(self.ptr()) };
        if p.is_null() {
            return String::new();
        }
        let s = unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() };
        unsafe { guardrail_free(p) };
        s
    }

    /// Whether the policy is active.
    pub fn active(&self) -> bool {
        if self.ptr().is_null() {
            return false;
        }
        unsafe { guardrail_config_active(self.ptr()) }
    }
}

/// Enforcer for one workflow (encode/decode).
pub struct Enforcer {
    pub(crate) handle: *mut std::ffi::c_void,
}

impl Drop for Enforcer {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { guardrail_enforcer_drop(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl std::fmt::Debug for Enforcer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enforcer").finish_non_exhaustive()
    }
}

unsafe impl Send for Enforcer {}
unsafe impl Sync for Enforcer {}

/// Result of encode: payload (masked only) plus optional injection text and metadata.
#[derive(Debug, Clone)]
pub struct EncodeResult {
    pub payload: serde_json::Value,
    /// Rule text to prepend when sending to LLM; empty when not applicable. Caller concatenates with payload.
    pub signature_injection_text: String,
    pub rules_applied_count: u32,
    pub rule_names: Vec<String>,
    pub policy_name: String,
}

/// Result of decode: payload plus metadata.
#[derive(Debug, Clone)]
pub struct DecodeResult {
    pub payload: serde_json::Value,
    pub rules_applied_count: u32,
    pub rule_names: Vec<String>,
    pub policy_name: String,
}

impl Enforcer {
    /// Encode payload (mask PII). When context is Llm, tokens get a 3-digit signature and instruction text is prepended.
    pub fn encode(&self, payload: serde_json::Value, context: EncodeContext) -> EncodeResult {
        let default_result = EncodeResult {
            payload: payload.clone(),
            signature_injection_text: String::new(),
            rules_applied_count: 0,
            rule_names: Vec::new(),
            policy_name: String::new(),
        };
        if self.handle.is_null() {
            return default_result;
        }
        let json = match serde_json::to_string(&payload) {
            Ok(s) => s,
            Err(_) => return default_result,
        };
        let c_str = match CString::new(json.as_bytes()) {
            Ok(c) => c,
            Err(_) => return default_result,
        };
        let enc_ctx = match context {
            EncodeContext::Llm => ENCODE_CONTEXT_LLM,
            EncodeContext::Manual => ENCODE_CONTEXT_MANUAL,
        };
        let out = unsafe {
            guardrail_encode(
                self.handle,
                c_str.as_ptr(),
                c_str.as_bytes().len(),
                enc_ctx,
            )
        };
        if out.is_null() {
            return default_result;
        }
        let out_slice = unsafe { CStr::from_ptr(out).to_bytes() };
        let out_str = String::from_utf8_lossy(out_slice).into_owned();
        unsafe { guardrail_free(out) };
        parse_encode_result(&out_str)
            .or_else(|| parse_encode_result_legacy(&out_str))
            .unwrap_or(default_result)
    }

    /// Decode payload (rehydrate PII).
    pub fn decode(&self, payload: serde_json::Value, context: DecodeContext) -> DecodeResult {
        let default_result = DecodeResult {
            payload: payload.clone(),
            rules_applied_count: 0,
            rule_names: Vec::new(),
            policy_name: String::new(),
        };
        if self.handle.is_null() {
            return default_result;
        }
        let json = match serde_json::to_string(&payload) {
            Ok(s) => s,
            Err(_) => return default_result,
        };
        let c_str = match CString::new(json.as_bytes()) {
            Ok(c) => c,
            Err(_) => return default_result,
        };
        let ctx = match context {
            DecodeContext::ToolBoundary => CONTEXT_TOOL_BOUNDARY,
            DecodeContext::LlmResponse => CONTEXT_LLM_RESPONSE,
            DecodeContext::ManualCall => CONTEXT_MANUAL_CALL,
        };
        let out =
            unsafe { guardrail_decode(self.handle, c_str.as_ptr(), c_str.as_bytes().len(), ctx) };
        if out.is_null() {
            return default_result;
        }
        let out_slice = unsafe { CStr::from_ptr(out).to_bytes() };
        let out_str = String::from_utf8_lossy(out_slice).into_owned();
        unsafe { guardrail_free(out) };
        parse_decode_result(&out_str)
            .or_else(|| parse_decode_result_legacy(&out_str))
            .unwrap_or(default_result)
    }
}

/// Legacy FFI return: raw encoded payload as JSON (old guardrail_encode returned Value serialized).
fn parse_encode_result_legacy(s: &str) -> Option<EncodeResult> {
    let payload: serde_json::Value = serde_json::from_str(s).ok()?;
    Some(EncodeResult {
        payload,
        signature_injection_text: String::new(),
        rules_applied_count: 0,
        rule_names: Vec::new(),
        policy_name: String::new(),
    })
}

/// Legacy FFI return: raw decoded payload as JSON (old guardrail_decode returned Value serialized).
fn parse_decode_result_legacy(s: &str) -> Option<DecodeResult> {
    let payload: serde_json::Value = serde_json::from_str(s).ok()?;
    Some(DecodeResult {
        payload,
        rules_applied_count: 0,
        rule_names: Vec::new(),
        policy_name: String::new(),
    })
}

fn parse_encode_result(s: &str) -> Option<EncodeResult> {
    let v: serde_json::Value = serde_json::from_str(s).ok()?;
    let payload = v.get("payload")?.clone();
    let signature_injection_text = v
        .get("signature_injection_text")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let rules_applied_count = v.get("rules_applied_count")?.as_u64()? as u32;
    let rule_names: Vec<String> = v
        .get("rule_names")?
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str().map(String::from))
        .collect();
    let policy_name = v.get("policy_name")?.as_str()?.to_string();
    Some(EncodeResult {
        payload,
        signature_injection_text,
        rules_applied_count,
        rule_names,
        policy_name,
    })
}

fn parse_decode_result(s: &str) -> Option<DecodeResult> {
    let v: serde_json::Value = serde_json::from_str(s).ok()?;
    let payload = v.get("payload")?.clone();
    let rules_applied_count = v.get("rules_applied_count")?.as_u64()? as u32;
    let rule_names: Vec<String> = v
        .get("rule_names")?
        .as_array()?
        .iter()
        .filter_map(|x| x.as_str().map(String::from))
        .collect();
    let policy_name = v.get("policy_name")?.as_str()?.to_string();
    Some(DecodeResult {
        payload,
        rules_applied_count,
        rule_names,
        policy_name,
    })
}

/// Singleton entry point to create enforcers.
#[derive(Debug, Clone)]
pub struct GuardRail;

impl GuardRail {
    /// Initialize (no-op for FFI; state is inside the lib).
    #[must_use]
    pub fn init() -> Self {
        Self
    }

    /// Create an enforcer for this workflow.
    pub fn enforcer_for(config: Arc<GuardRailConfig>, workflow_id: impl Into<String>) -> Enforcer {
        let workflow_id = workflow_id.into();
        let (ptr, len) = if workflow_id.is_empty() {
            (std::ptr::null(), 0)
        } else {
            (workflow_id.as_ptr() as *const c_char, workflow_id.len())
        };
        let handle = unsafe { guardrail_enforcer_create(config.ptr(), ptr, len) };
        assert!(!handle.is_null(), "guardrail_enforcer_create failed");
        Enforcer { handle }
    }
}
