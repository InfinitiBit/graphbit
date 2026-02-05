
/**
 * Safely extract error message from any thrown value
 */
function safeErrorMessage(error) {
    if (error === null) return 'null was thrown';
    if (error === undefined) return 'undefined was thrown';
    if (typeof error === 'string') return error;
    if (error instanceof Error) return error.message || error.toString();
    if (typeof error === 'object') {
        try {
            return JSON.stringify(error);
        } catch {
            return String(error);
        }
    }
    return String(error);
}

/**
* Sanitize values that cannot be serialized to JSON
*/
function sanitizeValue(value) {
    if (value === undefined) return null;
    if (typeof value === 'number') {
        if (!Number.isFinite(value)) return null; // Infinity, -Infinity, NaN
        return value;
    }
    if (value === null) return null;
    if (Array.isArray(value)) {
        return value.map(sanitizeValue);
    }
    if (typeof value === 'object') {
        const result = {};
        for (const [key, val] of Object.entries(value)) {
            result[key] = sanitizeValue(val);
        }
        return result;
    }
    return value;
}

/**
* Wraps a user callback to support async/Promise returns.
*/
function wrapAsync(registry, userCallback) {
    if (typeof userCallback !== 'function') {
        throw new TypeError('wrapAsync expects a function as second argument');
    }

    return function asyncCallbackWrapper(wrapperArgs) {
        const pendingId = wrapperArgs.__pendingId;
        const originalArgs = wrapperArgs.__originalArgs;

        if (!pendingId) {
            return userCallback(wrapperArgs);
        }

        try {
            const maybePromise = userCallback(originalArgs);

            if (maybePromise && typeof maybePromise.then === 'function') {
                Promise.resolve(maybePromise)
                    .then(result => {
                        const sanitized = sanitizeValue(result);
                        registry.setPendingResult(pendingId, sanitized, null);
                    })
                    .catch(error => {
                        registry.setPendingResult(pendingId, null, safeErrorMessage(error));
                    });

                return { __pending: true };
            } else {
                return sanitizeValue(maybePromise);
            }
        } catch (error) {
            return { __error: safeErrorMessage(error) };
        }
    };
}

function createAsyncTool(registry, handler) {
    return wrapAsync(registry, handler);
}

function registerAsync(registry, name, description, parameters, callback) {
    const wrappedCallback = wrapAsync(registry, callback);
    return registry.register(name, description, parameters, wrappedCallback);
}

async function executeAsync(registry, toolName, args, asyncCallback) {
    if (asyncCallback && typeof asyncCallback === 'function') {
        try {
            const result = await asyncCallback(args);
            return {
                success: true,
                result: result,
                error: null,
                executionTimeMs: 0
            };
        } catch (error) {
            return {
                success: false,
                result: null,
                error: error.message || String(error),
                executionTimeMs: 0
            };
        }
    } else {
        return await registry.execute(toolName, args);
    }
}

module.exports = {
    wrapAsync,
    createAsyncTool,
    registerAsync,
    executeAsync,
    safeErrorMessage,
    sanitizeValue
};
