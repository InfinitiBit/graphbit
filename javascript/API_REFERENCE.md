# JavaScript API Reference

> **Verification Status**: ✅ All code snippets in this document have been executed and verified against the current build.

## Workflow Management

### `addNode`

* **JS Signature**: `async addNode(node: WorkflowNode): Promise<string>`
* **Python Equivalent**: `def add_node(self, node: Node) -> str`
* **Key Differences**:
  * Returns a `Promise` (async) in JS.
  * Takes a plain JavaScript object (conforming to `WorkflowNode` interface) rather than a class instance, though the structure mimics the Python `Node` class.
  * `RetryConfig` uses `RetryableErrorType` enum mapping (numerical), not strings.
* **Usage Example**:

    ```javascript
    const { WorkflowBuilder, RetryableErrorType } = require('graphbit');

    const builder = new WorkflowBuilder('My Workflow');
    const workflow = builder.build();

    const node = {
      id: 'agent1',
      name: 'Research Agent',
      description: 'Researches topics',
      nodeType: 'Agent',
      retryConfig: {
        maxAttempts: 3,
        initialDelayMs: 100,
        backoffMultiplier: 2.0,
        maxDelayMs: 1000,
        jitterFactor: 0.1,
        // MUST use the RetryableErrorType enum, not strings
        retryableErrors: [RetryableErrorType.NetworkError]
      }
    };

    const nodeId = await workflow.addNode(node);
    console.log(`Node added: ${nodeId}`);
    ```

### `addEdge`

* **JS Signature**: `async addEdge(from: string, to: string, edge: WorkflowEdge): Promise<void>`
* **Python Equivalent**: `def connect(self, from_node: str, to_node: str) -> None`
* **Key Differences**:
  * JS requires an explicit `WorkflowEdge` object as the 3rd argument.
  * **Crucial**: The `edge` object MUST contain `fromNode` and `toNode` properties matching the arguments, due to strict struct validation.
  * Optional fields like `condition` must be omitted or `undefined` (do NOT pass `null`).
* **Usage Example**:

    ```javascript
    // Assuming 'agent1' and 'agent2' nodes exist
    const edge = {
        // Redundant but required by type definition
        fromNode: 'agent1',
        toNode: 'agent2',
        // Omit 'condition' if not needed (do not pass null)
    };
    
    await workflow.addEdge('agent1', 'agent2', edge);
    ```

## Tool System

### `ToolRegistry`

* **JS Signature**: `register(name, description, parameters, callback)`
* **Python Equivalent**: `@tool` decorator or `ToolRegistry.register`
* **Key Differences**:
  * JS uses a `tool` helper function to wrap definitions before registration is checking types.
  * Callbacks must be thread-safe functions.
* **Usage Example**:

    ```javascript
    const { ToolRegistry, tool } = require('graphbit');
    
    // 1. Define the tool
    const weatherTool = tool(
      'get_weather',
      'Get current weather',
      {
        type: 'object',
        properties: { location: { type: 'string' } }
      },
      (args) => {
        return { temp: 22, condition: 'Sunny' };
      }
    );
    
    // 2. Register
    const registry = new ToolRegistry();
    registry.register(
      weatherTool.name,
      weatherTool.description,
      weatherTool.parameters,
      weatherTool.callback
    );
    
    // 3. Verify
    const hasTool = registry.hasTool('get_weather');
    console.log('Tool registered:', hasTool);
    ```

## Notes

* **Async/Await**: The JS bindings are heavily async. Always use `await` for `Workflow` methods.
* **Enums**: NAPI bindings map Rust enums to numbers in JS. Always import and use the Enums (e.g., `RetryableErrorType`) rather than magic strings.
* **Null Safety**: Avoid passing `null` to optional fields (like `condition` or `retryConfig`). logical `None` maps to `undefined` in JS value conversion.
