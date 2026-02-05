
/**
 * GraphBit JavaScript Bindings
 * 
 * This module provides the main entry point for the GraphBit library,
 * combining the native Rust bindings with a robust async helper layer.
 */

export * from './index';
import { ToolRegistry, ToolResult } from './index';

/**
 * Wraps a user callback to support async/Promise returns.
 * 
 * This implements the Callback ID Pattern where:
 * 1. Native code passes `{ __pendingId, __originalArgs }` to the wrapper
 * 2. Wrapper executes the user's callback (sync or async)
 * 3. If the result is a Promise, it returns `{ __pending: true }` immediately
 * 4. When the Promise resolves, it calls `registry.setPendingResult(pendingId, result)`
 * 5. Native code waits for the result via the pending ID channel
 * 
 * @param registry The registry instance
 * @param userCallback The user's callback function (may be async or return Promise)
 * @returns A wrapped function that handles async state management
 * 
 * @example
 * ```typescript
 * const wrapped = wrapAsync(registry, async (args) => {
 *   const data = await fetch(args.url);
 *   return await data.json();
 * });
 * registry.register('my_tool', 'desc', {}, wrapped);
 * ```
 */
export declare function wrapAsync(registry: ToolRegistry, userCallback: (...args: any[]) => any): (...args: any[]) => any;

/**
 * Alias for wrapAsync for compatibility.
 * @see wrapAsync
 */
export declare function createAsyncTool(registry: ToolRegistry, handler: (...args: any[]) => any): (...args: any[]) => any;

/**
 * Registers a tool with built-in async callback support.
 * This is the recommended way to register tools that perform asynchronous operations.
 * 
 * @param registry The registry instance
 * @param name Tool name
 * @param description Tool description
 * @param parameters JSON Schema for tool parameters
 * @param callback Async callback function
 * 
 * @example
 * ```typescript
 * registerAsync(
 *   registry, 
 *   'fetch_data', 
 *   'Fetches data from API', 
 *   { url: { type: 'string' } }, 
 *   async (args) => {
 *     const response = await fetch(args.url);
 *     return await response.json();
 *   }
 * );
 * ```
 */
export declare function registerAsync(
    registry: ToolRegistry,
    name: string,
    description: string,
    parameters: any,
    callback: (...args: any[]) => Promise<any>
): void;

/**
 * Execute a tool asynchronously (Legacy Wrapper).
 * 
 * @deprecated Use `registry.execute()` directly. The robust wrapper ensures `registry.execute()`
 * automatically handles async tools correctly without needing this special method.
 */
export declare function executeAsync(
    registry: ToolRegistry,
    toolName: string,
    args: any,
    asyncCallback?: ((args: any) => Promise<any>) | undefined | null
): Promise<ToolResult>;
