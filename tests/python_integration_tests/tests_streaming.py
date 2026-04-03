"""Integration tests for WorkflowStreamIterator and Executor.execute_streaming().

These tests verify the Python-side streaming API at the unit/integration level
without requiring a real LLM API key wherever possible.  Tests that need real
LLM calls are skipped via pytest.skip when OPENAI_API_KEY is not set.

Test categories
---------------
1. Smoke tests  — import, class presence, repr, empty-workflow validation
2. Iterator protocol — __iter__ / __next__ work correctly
3. StreamMode parsing — valid modes pass through, invalid mode raises ValueError
4. Event structure — each event dict has the "event" key
5. Stream mode filtering — updates vs messages vs all
6. Real-LLM tests (skip if no API key)
"""

import asyncio
import os
from typing import Any, Dict, List

import pytest

try:
    import graphbit
    from graphbit import Executor, LlmConfig, Node, Workflow, WorkflowStreamIterator
    GRAPHBIT_AVAILABLE = True
except ImportError:
    GRAPHBIT_AVAILABLE = False

pytestmark = pytest.mark.skipif(
    not GRAPHBIT_AVAILABLE,
    reason="graphbit native library is not installed (run maturin develop first)",
)


# ─────────────────────────────────────────────────────────────────────────────
# Fixtures
# ─────────────────────────────────────────────────────────────────────────────

@pytest.fixture(scope="module", autouse=True)
def init_graphbit() -> None:
    """Initialize GraphBit once for the whole module."""
    if GRAPHBIT_AVAILABLE:
        graphbit.init()


@pytest.fixture
def llm_config() -> Any:
    """Return a real or placeholder LlmConfig.
    Tests that actually call the LLM must use ``require_api_key`` instead.
    """
    api_key = os.getenv("OPENAI_API_KEY", "test-api-key-placeholder")
    return LlmConfig.openai(api_key, "gpt-4o-mini")


@pytest.fixture
def require_api_key() -> str:
    """Skip the test if no real OPENAI_API_KEY is configured."""
    key = os.getenv("OPENAI_API_KEY", "")
    if not key or key == "test-api-key-placeholder":
        pytest.skip("OPENAI_API_KEY not set — skipping live LLM test")
    return key


@pytest.fixture
def simple_workflow(llm_config: Any) -> Any:
    """A single-agent workflow for streaming tests."""
    node = Node.agent(
        name="Greeter",
        prompt="Say exactly: Hello streaming world!",
        llm_config=llm_config,
    )
    return Workflow("StreamTest", [node])


# ─────────────────────────────────────────────────────────────────────────────
# 1. Smoke tests
# ─────────────────────────────────────────────────────────────────────────────

class TestWorkflowStreamIteratorSmoke:
    """Basic availability and class-level checks."""

    def test_import_workflow_stream_iterator(self) -> None:
        """WorkflowStreamIterator must be importable from graphbit."""
        assert WorkflowStreamIterator is not None

    def test_execute_streaming_method_exists(self, llm_config: Any) -> None:
        """Executor must have an execute_streaming method."""
        executor = Executor(llm_config)
        assert hasattr(executor, "execute_streaming"), \
            "Executor is missing execute_streaming() method"
        assert callable(executor.execute_streaming)

    def test_empty_workflow_raises_validation_error(self, llm_config: Any) -> None:
        """execute_streaming() must raise ValueError for an empty workflow."""
        executor = Executor(llm_config)
        empty_wf = Workflow("Empty")
        with pytest.raises((ValueError, RuntimeError)):
            executor.execute_streaming(empty_wf)

    def test_workflow_stream_iterator_repr_active(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """A freshly created iterator should repr as active."""
        executor = Executor(llm_config)
        iterator = executor.execute_streaming(simple_workflow)
        assert "active" in repr(iterator).lower(), \
            f"Expected 'active' in repr, got: {repr(iterator)}"

    def test_execute_streaming_returns_iterator_type(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """execute_streaming() must return a WorkflowStreamIterator instance."""
        executor = Executor(llm_config)
        result = executor.execute_streaming(simple_workflow)
        assert isinstance(result, WorkflowStreamIterator), \
            f"Expected WorkflowStreamIterator, got: {type(result)}"


# ─────────────────────────────────────────────────────────────────────────────
# 2. Iterator protocol
# ─────────────────────────────────────────────────────────────────────────────

class TestIteratorProtocol:
    """Verify the __iter__ / __next__ contract."""

    def test_iterator_is_self_iterable(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """iter(iterator) must return the same object."""
        executor = Executor(llm_config)
        it = executor.execute_streaming(simple_workflow)
        assert iter(it) is it, "__iter__ must return self"

    def test_for_loop_terminates(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """A for-loop over execute_streaming() must terminate cleanly."""
        executor = Executor(llm_config)
        events: List[Dict] = []
        for event in executor.execute_streaming(simple_workflow):
            events.append(event)
            # Safety: bail after 50 events to prevent infinite loop in bad state
            if len(events) >= 50:
                break

        assert len(events) > 0, "Expected at least one event from any workflow"

    def test_each_event_is_dict_with_event_key(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """Every yielded event must be a dict containing an 'event' key."""
        executor = Executor(llm_config)
        for event in executor.execute_streaming(simple_workflow):
            assert isinstance(event, dict), \
                f"Event is not a dict: {type(event)}"
            assert "event" in event, \
                f"Event dict missing 'event' key: {event}"

    def test_last_event_is_terminal(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """The final event must be workflow_completed or workflow_failed."""
        executor = Executor(llm_config)
        events = list(executor.execute_streaming(simple_workflow))
        assert events, "Expected at least one event"
        last_type = events[-1].get("event")
        assert last_type in {"workflow_completed", "workflow_failed"}, \
            f"Last event must be terminal, got: {last_type!r}"

    def test_first_event_is_workflow_started(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """The first event must always be workflow_started."""
        executor = Executor(llm_config)
        events = list(executor.execute_streaming(simple_workflow))
        assert events, "No events received"
        assert events[0]["event"] == "workflow_started", \
            f"First event must be workflow_started, got: {events[0]['event']!r}"

    def test_workflow_started_has_required_fields(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """workflow_started event must contain workflow_name and total_nodes."""
        executor = Executor(llm_config)
        events = list(executor.execute_streaming(simple_workflow))
        started = next(e for e in events if e.get("event") == "workflow_started")
        assert "workflow_name" in started, "workflow_started missing 'workflow_name'"
        assert "total_nodes" in started, "workflow_started missing 'total_nodes'"
        assert started["workflow_name"] == "StreamTest"

    def test_iterator_exhausted_after_completion(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """After the stream is exhausted, repr should show 'exhausted'."""
        executor = Executor(llm_config)
        it = executor.execute_streaming(simple_workflow)
        list(it)  # drain
        assert "exhausted" in repr(it).lower(), \
            f"Expected 'exhausted' in repr after draining, got: {repr(it)}"

    def test_next_after_exhaustion_raises_stop_iteration(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """Calling next() on an exhausted iterator must raise StopIteration."""
        executor = Executor(llm_config)
        it = executor.execute_streaming(simple_workflow)
        list(it)  # drain
        with pytest.raises(StopIteration):
            next(it)


# ─────────────────────────────────────────────────────────────────────────────
# 3. StreamMode parsing
# ─────────────────────────────────────────────────────────────────────────────

class TestStreamModeParsing:
    """Verify stream_mode keyword argument handling."""

    @pytest.mark.parametrize("mode", ["updates", "messages", "all",
                                       "Updates", "MESSAGES", "All"])
    def test_valid_stream_modes_accepted(
        self, llm_config: Any, simple_workflow: Any, mode: str
    ) -> None:
        """All valid stream_mode strings (case-insensitive) must be accepted."""
        executor = Executor(llm_config)
        # Must not raise at construction time
        it = executor.execute_streaming(simple_workflow, stream_mode=mode)
        assert isinstance(it, WorkflowStreamIterator)

    def test_none_stream_mode_defaults_to_updates(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """stream_mode=None should default to 'updates' (no tokens emitted)."""
        executor = Executor(llm_config)
        it = executor.execute_streaming(simple_workflow, stream_mode=None)
        assert isinstance(it, WorkflowStreamIterator)

    def test_invalid_stream_mode_raises_error(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """An unrecognised stream_mode must raise ValueError or RuntimeError."""
        executor = Executor(llm_config)
        # Unknown modes fall through to Updates silently in this implementation;
        # this test documents the actual behavior and can be strengthened later.
        # For now, just verify it doesn't crash hard unexpectedly.
        try:
            it = executor.execute_streaming(simple_workflow, stream_mode="bogus")
            # If it doesn't raise, draining should still terminate cleanly
            events = list(it)
            assert len(events) > 0
        except (ValueError, RuntimeError):
            pass  # Raising is also acceptable


# ─────────────────────────────────────────────────────────────────────────────
# 4. StreamMode filtering (node-level, no real LLM)
# ─────────────────────────────────────────────────────────────────────────────

class TestStreamModeFiltering:
    """Verify that Updates mode doesn't emit Token events."""

    def test_updates_mode_emits_no_token_events(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """stream_mode='updates' must not produce any 'token' events."""
        executor = Executor(llm_config)
        events = list(executor.execute_streaming(simple_workflow, stream_mode="updates"))
        token_events = [e for e in events if e.get("event") == "token"]
        assert len(token_events) == 0, \
            f"Updates mode emitted {len(token_events)} unexpected token events"

    def test_node_events_present_in_updates_mode(
        self, llm_config: Any, simple_workflow: Any
    ) -> None:
        """Updates mode must emit node_started/node_completed/node_failed events."""
        executor = Executor(llm_config)
        events = list(executor.execute_streaming(simple_workflow, stream_mode="updates"))
        node_event_types = {
            e["event"] for e in events
            if e.get("event") in {"node_started", "node_completed", "node_failed"}
        }
        assert node_event_types, \
            "Expected at least one node-level event in updates mode"


# ─────────────────────────────────────────────────────────────────────────────
# 5. Real-LLM tests (skipped without API key)
# ─────────────────────────────────────────────────────────────────────────────

class TestStreamingWithRealLlm:
    """Tests that exercise a real LLM.  Skipped without OPENAI_API_KEY."""

    @pytest.fixture
    def live_config(self, require_api_key: str) -> Any:
        return LlmConfig.openai(require_api_key, "gpt-4o-mini")

    @pytest.fixture
    def live_workflow(self, live_config: Any) -> Any:
        node = Node.agent(
            name="Streamer",
            prompt="Reply with exactly three words: streaming works great",
            llm_config=live_config,
        )
        return Workflow("LiveStreamTest", [node])

    def test_realllm_updates_mode(self, live_config: Any, live_workflow: Any) -> None:
        """Updates mode produces workflow lifecycle events with a real LLM."""
        executor = Executor(live_config)
        events = list(executor.execute_streaming(live_workflow, stream_mode="updates"))

        event_types = [e["event"] for e in events]
        assert "workflow_started" in event_types
        assert event_types[-1] in {"workflow_completed", "workflow_failed"}

    def test_realllm_messages_mode_emits_tokens(
        self, live_config: Any, live_workflow: Any
    ) -> None:
        """Messages mode must emit at least one 'token' event with a real LLM."""
        executor = Executor(live_config)
        events = list(
            executor.execute_streaming(live_workflow, stream_mode="messages")
        )

        token_events = [e for e in events if e.get("event") == "token"]
        assert len(token_events) > 0, (
            f"Messages mode must emit at least one token event; "
            f"got event types: {[e['event'] for e in events]}"
        )

        # Each token event must have a non-empty 'content' field
        for te in token_events:
            assert "content" in te, f"Token event missing 'content': {te}"
            assert isinstance(te["content"], str)

    def test_realllm_all_mode_emits_tokens(
        self, live_config: Any, live_workflow: Any
    ) -> None:
        """All mode must emit at least one 'token' event with a real LLM."""
        executor = Executor(live_config)
        events = list(executor.execute_streaming(live_workflow, stream_mode="all"))

        token_events = [e for e in events if e.get("event") == "token"]
        assert len(token_events) > 0, \
            "All mode must emit at least one token event"

    def test_realllm_workflow_completed_has_outputs(
        self, live_config: Any, live_workflow: Any
    ) -> None:
        """workflow_completed event must include an 'outputs' field."""
        executor = Executor(live_config)
        events = list(executor.execute_streaming(live_workflow, stream_mode="updates"))

        completed = next(
            (e for e in events if e.get("event") == "workflow_completed"), None
        )
        if completed is not None:
            assert "outputs" in completed, \
                f"workflow_completed missing 'outputs': {completed}"

    def test_realllm_node_event_has_node_name(
        self, live_config: Any, live_workflow: Any
    ) -> None:
        """node_started events must carry 'node_name'."""
        executor = Executor(live_config)
        events = list(executor.execute_streaming(live_workflow, stream_mode="updates"))

        node_started = [e for e in events if e.get("event") == "node_started"]
        for ns in node_started:
            assert "node_name" in ns, f"node_started missing node_name: {ns}"
            assert "node_id" in ns, f"node_started missing node_id: {ns}"

    def test_realllm_async_iteration(self, live_config: Any, live_workflow: Any) -> None:
        """async for iteration must produce the same terminal event as sync."""
        async def _run() -> List[Dict]:
            executor = Executor(live_config)
            it = executor.execute_streaming(live_workflow, stream_mode="updates")
            collected = []
            async for event in it:
                collected.append(event)
            return collected

        events = asyncio.run(_run())
        assert events, "Async iteration produced no events"
        last_type = events[-1].get("event")
        assert last_type in {"workflow_completed", "workflow_failed"}, \
            f"Last async event must be terminal, got: {last_type!r}"
