/**
 * Core Concepts Example
 * 
 * Demonstrates fundamental GraphBit concepts in JavaScript/TypeScript.
 */

import {
    init,
    version,
    versionInfo,
    LlmConfig,
    WorkflowBuilder,
    Executor
} from '../index';

async function main() {
    console.log('GraphBit Core Concepts Example\n');

    // === 1. Library Initialization ===
    console.log('=== Library Initialization ===');
    init();
    console.log(`GraphBit version: ${version()}`);

    const info = versionInfo();
    console.log(`Rust version: ${info.rustVersion}`);
    console.log(`NAPI version: ${info.napiVersion}`);

    // === 2. LLM Configuration ===
    console.log('\n=== LLM Provider Configuration ===');

    const ollamaConfig = LlmConfig.ollama({
        model: 'llama2'
    });
    console.log('✓ Ollama config created');

    // === 3. Workflow Creation ===
    console.log('\n=== Workflow Creation ===');

    const workflow = new WorkflowBuilder('Demo Workflow')
        .description('A simple demonstration workflow')
        .addMetadata('purpose', JSON.stringify('demonstration'))
        .build();

    console.log(`Workflow ID: ${await workflow.id()}`);
    console.log(`Workflow name: ${await workflow.name()}`);
    /**
     * Core Concepts Example
     * 
     * Demonstrates fundamental GraphBit concepts in JavaScript/TypeScript.
     */

    import {
        init,
        version,
        versionInfo,
        LlmConfig,
        WorkflowBuilder,
        Executor
    } from '../index';

    async function main() {
        console.log('GraphBit Core Concepts Example\n');

        // === 1. Library Initialization ===
        console.log('=== Library Initialization ===');
        init();
        console.log(`GraphBit version: ${version()}`);

        const info = versionInfo();
        console.log(`Rust version: ${info.rustVersion}`);
        console.log(`NAPI version: ${info.napiVersion}`);

        // === 2. LLM Configuration ===
        console.log('\n=== LLM Provider Configuration ===');

        const ollamaConfig = LlmConfig.ollama({
            model: 'llama2'
        });
        console.log('✓ Ollama config created');

        // === 3. Workflow Creation ===
        console.log('\n=== Workflow Creation ===');

        const workflow = new WorkflowBuilder('Demo Workflow')
            .description('A simple demonstration workflow')
            .addMetadata('purpose', JSON.stringify('demonstration'))
            .build();

        console.log(`Workflow ID: ${await workflow.id()}`);
        console.log(`Workflow name: ${await workflow.name()}`);
        console.log(`Workflow valid: ${await workflow.validate()}`);

        // === 4. Executor===
        console.log('\n=== Executor Configuration ===');

        const executor = new Executor(ollamaConfig, {
            timeoutSeconds: 60,
            debug: false
        });
        void executor; // Suppress unused warning
        console.log('✓ Executor created');
        console.log('  Config: 60s timeout, debug=false');

        console.log('\n=== Core Concepts Demo Complete ===');
    }

    main().catch(console.error);
