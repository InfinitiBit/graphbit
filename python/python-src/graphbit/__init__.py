# Re-export all Rust bindings at the top level
# This allows: from graphbit import LlmConfig, Executor, Workflow, Node, etc.

# Core functions
from .graphbit import (
    init,
    version,
    get_system_info,
    health_check,
    configure_runtime,
    shutdown,
)

# Document loader classes
from .graphbit import (
    DocumentLoaderConfig,
    DocumentContent,
    DocumentLoader,
)

# LLM classes
from .graphbit import (
    LlmConfig,
    LlmClient,
    LlmUsage,
    FinishReason,
    LlmToolCall,
    LlmResponse,
)

# Workflow classes
from .graphbit import (
    Node,
    Workflow,
    WorkflowContext,
    WorkflowResult,
    Executor,
)

# Embedding classes
from .graphbit import (
    EmbeddingConfig,
    EmbeddingClient,
)

# Text splitter classes
from .graphbit import (
    TextSplitterConfig,
    TextChunk,
    CharacterSplitter,
    TokenSplitter,
    SentenceSplitter,
    RecursiveSplitter,
    TextSplitter,
)

# Tool system classes
from .graphbit import (
    ToolResult,
    ToolRegistry,
    ToolDecorator,
    ToolExecutor,
    ExecutorConfig,
    ToolResultCollection,
)

# Tool functions
from .graphbit import (
    tool,
    get_tool_registry,
    clear_tools,
    execute_tool,
    get_registered_tools,
    execute_workflow_tool_calls,
    execute_production_tool_calls,
    sync_global_tools_to_workflow,
)

# Module metadata
from .graphbit import __version__, __author__, __description__

# Define __all__ for explicit exports
__all__ = [
    # Core functions
    "init",
    "version",
    "get_system_info",
    "health_check",
    "configure_runtime",
    "shutdown",
    # Document loader
    "DocumentLoaderConfig",
    "DocumentContent",
    "DocumentLoader",
    # LLM
    "LlmConfig",
    "LlmClient",
    "LlmUsage",
    "FinishReason",
    "LlmToolCall",
    "LlmResponse",
    # Workflow
    "Node",
    "Workflow",
    "WorkflowContext",
    "WorkflowResult",
    "Executor",
    # Embeddings
    "EmbeddingConfig",
    "EmbeddingClient",
    # Text splitter
    "TextSplitterConfig",
    "TextChunk",
    "CharacterSplitter",
    "TokenSplitter",
    "SentenceSplitter",
    "RecursiveSplitter",
    "TextSplitter",
    # Tools
    "ToolResult",
    "ToolRegistry",
    "ToolDecorator",
    "ToolExecutor",
    "ExecutorConfig",
    "ToolResultCollection",
    "tool",
    "get_tool_registry",
    "clear_tools",
    "execute_tool",
    "get_registered_tools",
    "execute_workflow_tool_calls",
    "execute_production_tool_calls",
    "sync_global_tools_to_workflow",
    # Metadata
    "__version__",
    "__author__",
    "__description__",
]

# The providers submodule is accessible as: from graphbit.providers import Huggingface
# This is handled by the providers/__init__.py file
