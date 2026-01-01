import { init, version, versionInfo, LlmConfig } from '../index';

function main() {
    console.log('Testing GraphBit installation...');

    // Initialize
    init();

    // Check version
    console.log(`GraphBit version: ${version()}`);

    // Check detailed version info
    const info = versionInfo();
    console.log('Version Info:', info);

    // Test LLM configuration
    if (process.env.OPENAI_API_KEY) {
        const config = LlmConfig.openai({
            apiKey: process.env.OPENAI_API_KEY,
            model: 'gpt-4o-mini'
        });
        // Use config to avoid unused variable error
        console.log(`LLM Config created for model: ${process.env.OPENAI_API_KEY ? 'gpt-4o-mini' : 'unknown'}`);
        // Actually LlmConfig doesn't expose model getter in JS bindings directly on the instance?
        // index.d.ts says LlmConfig is a class but doesn't show methods on instance?
        // Wait, index.d.ts: export declare class LlmConfig { static openai(...): LlmConfig ... }
        // It doesn't show instance methods.
        // So I can't call config.model().
        // I'll just log that it was created.
        console.log('LLM Config created successfully');
        void config; // Suppress unused variable
    } else {
        console.log('No OPENAI_API_KEY found - set up API keys to use LLM features');
    }

    console.log('Installation successful!');
}

main();
