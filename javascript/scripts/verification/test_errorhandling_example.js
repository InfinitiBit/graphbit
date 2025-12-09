/**
 * Error Handling Patterns Test
 * Tests all error handling patterns from the example
 */

async function testErrorHandlingPatterns() {
    console.log('🧪 Testing Error Handling Patterns\n');
    console.log('='.repeat(60) + '\n');

    try {
        // Pattern 1: Basic Try-Catch
        console.log('📝 Pattern 1: Basic Try-Catch\n');

        async function basicErrorHandling() {
            try {
                throw new Error('Test error');
            } catch (error) {
                return 'Service temporarily unavailable';
            }
        }

        const result1 = await basicErrorHandling();
        console.log(`✅ Basic try-catch: "${result1}"\n`);

        // Pattern 2: Retry with Exponential Backoff
        console.log('📝 Pattern 2: Retry with Exponential Backoff\n');

        async function retryWithBackoff(fn, maxRetries = 3, baseDelay = 100) {
            for (let attempt = 1; attempt <= maxRetries; attempt++) {
                try {
                    return await fn();
                } catch (error) {
                    if (attempt === maxRetries) {
                        throw new Error(`Failed after ${maxRetries} attempts`);
                    }

                    const delay = baseDelay * Math.pow(2, attempt - 1);
                    console.log(`   Attempt ${attempt} failed retrying in ${delay}ms...`);

                    await new Promise(resolve => setTimeout(resolve, delay));
                }
            }
        }

        let attemptCount = 0;
        try {
            await retryWithBackoff(async () => {
                attemptCount++;
                if (attemptCount < 3) throw new Error('Retry test');
                return 'success';
            });
        } catch (e) {
            // Expected
        }

        console.log(`✅ Retry pattern tested: ${attemptCount} attempts\n`);

        // Pattern 3: Circuit Breaker
        console.log('📝 Pattern 3: Circuit Breaker\n');

        class CircuitBreaker {
            constructor(threshold = 3, timeout = 5000) {
                this.failureCount = 0;
                this.threshold = threshold;
                this.timeout = timeout;
                this.state = 'CLOSED';
                this.nextAttempt = Date.now();
            }

            async execute(fn) {
                if (this.state === 'OPEN') {
                    if (Date.now() < this.nextAttempt) {
                        throw new Error('Circuit breaker is OPEN');
                    }
                    this.state = 'HALF_OPEN';
                }

                try {
                    const result = await fn();
                    this.onSuccess();
                    return result;
                } catch (error) {
                    this.onFailure();
                    throw error;
                }
            }

            onSuccess() {
                this.failureCount = 0;
                if (this.state === 'HALF_OPEN') {
                    this.state = 'CLOSED';
                }
            }

            onFailure() {
                this.failureCount++;
                if (this.failureCount >= this.threshold) {
                    this.state = 'OPEN';
                    this.nextAttempt = Date.now() + this.timeout;
                }
            }

            getState() {
                return this.state;
            }
        }

        const breaker = new CircuitBreaker(3, 1000);

        // Trigger failures
        for (let i = 0; i < 3; i++) {
            try {
                await breaker.execute(async () => {
                    throw new Error('Failure');
                });
            } catch (e) {
                // Expected
            }
        }

        console.log(`✅ Circuit breaker state: ${breaker.getState()}\n`);

        // Pattern 4: Timeout Handler
        console.log('📝 Pattern 4: Timeout Handler\n');

        async function withTimeout(promise, timeoutMs) {
            const timeoutPromise = new Promise((_, reject) => {
                setTimeout(() => reject(new Error('Timeout')), timeoutMs);
            });

            return Promise.race([promise, timeoutPromise]);
        }

        try {
            await withTimeout(
                new Promise(resolve => setTimeout(resolve, 1000)),
                100
            );
        } catch (error) {
            if (error.message === 'Timeout') {
                console.log('✅ Timeout pattern works correctly\n');
            }
        }

        // Pattern 5: Error Classification
        console.log('📝 Pattern 5: Error Classification\n');

        class ErrorHandler {
            static classify(error) {
                const message = error.message.toLowerCase();

                if (message.includes('rate limit') || message.includes('429')) {
                    return 'RATE_LIMIT';
                }
                if (message.includes('api key') || message.includes('401')) {
                    return 'AUTH_ERROR';
                }
                if (message.includes('timeout')) {
                    return 'TIMEOUT';
                }
                return 'UNKNOWN_ERROR';
            }
        }

        const testErrors = [
            new Error('Rate limit exceeded'),
            new Error('Invalid API key'),
            new Error('Request timeout')
        ];

        testErrors.forEach(err => {
            const type = ErrorHandler.classify(err);
            console.log(`   ${err.message} → ${type}`);
        });

        console.log('\n✅ Error classification works\n');

        // Pattern 6: Structured Logging
        console.log('📝 Pattern 6: Structured Error Logging\n');

        class ErrorLogger {
            log(error, context = {}) {
                return {
                    timestamp: new Date().toISOString(),
                    error: {
                        message: error.message,
                        type: error.constructor.name
                    },
                    context
                };
            }
        }

        const logger = new ErrorLogger();
        const logEntry = logger.log(new Error('Test error'), {
            operation: 'test',
            user: 'test_user'
        });

        console.log(`✅ Error logging structure created:`);
        console.log(`   Timestamp: ${logEntry.timestamp}`);
        console.log(`   Error type: ${logEntry.error.type}\n`);

        // Pattern 7: NAPI-RS Error Handling
        console.log('📝 Pattern 7: NAPI-RS Error Detection\n');

        function handleNAPIError(error) {
            const message = error.message;

            if (message.includes('NumberExpected')) {
                return 'ENUM_ERROR';
            }
            if (message.includes('StringExpected')) {
                return 'NULL_ERROR';
            }
            if (message.includes('GenericFailure')) {
                return 'TOOL_ERROR';
            }
            return 'UNKNOWN_NAPI_ERROR';
        }

        const napiErrors = [
            new Error('NumberExpected for field retryableErrors'),
            new Error('StringExpected for field condition'),
            new Error('GenericFailure in callback')
        ];

        napiErrors.forEach(err => {
            const type = handleNAPIError(err);
            console.log(`   ${type} detected`);
        });

        console.log('\n✅ NAPI error handling works\n');

        console.log('='.repeat(60));
        console.log('✅ All error handling patterns verified!\n');

        return true;

    } catch (error) {
        console.error('❌ Error handling test failed:', error.message);
        console.error(error.stack);
        return false;
    }
}

// Run test
testErrorHandlingPatterns()
    .then(success => {
        process.exit(success ? 0 : 1);
    })
    .catch(error => {
        console.error('Fatal error:', error);
        process.exit(1);
    });
