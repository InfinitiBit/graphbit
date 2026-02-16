//! Unique identifiers for agents, workflows, and nodes.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for agents
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    /// Create a new random agent ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create an agent ID from a string
    /// If the string is a valid UUID, it's used directly
    /// Otherwise, a deterministic UUID is generated from the string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self(uuid));
        }
        let namespace = Uuid::NAMESPACE_DNS;
        let uuid = Uuid::new_v5(&namespace, s.as_bytes());
        Ok(Self(uuid))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AgentId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AgentId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for workflows
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkflowId(pub Uuid);

impl WorkflowId {
    /// Create a new random workflow ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a workflow ID from a string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for WorkflowId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WorkflowId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for workflow nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    /// Create a new random node ID
    #[inline]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a node ID from a string
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self(uuid));
        }
        let namespace = Uuid::NAMESPACE_OID;
        let uuid = Uuid::new_v5(&namespace, s.as_bytes());
        Ok(Self(uuid))
    }

    /// Get the underlying UUID
    #[inline]
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for NodeId {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
