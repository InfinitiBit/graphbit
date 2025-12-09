const {
    init,
    AgentBuilder,
    LlmConfig
} = require('../javascript/index');

/**
 * Live Multi-Agent System Test
 * Tests collaborative agents with real OpenAI API
 */

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;

class MultiAgentSystem {
    constructor(apiKey) {
        this.apiKey = apiKey;
        this.agents = {};
    }

    async initializeAgents() {
        console.log('🤖 Initializing specialized agents...\n');

        const llmConfig = LlmConfig.openai({
            apiKey: this.apiKey,
            model: 'gpt-4o-mini'
        });

        // Researcher Agent
        this.agents.researcher = await new AgentBuilder('Researcher', llmConfig)
            .description('Gathers and organizes information')
            .systemPrompt(`You are a thorough researcher. Gather comprehensive information on the given topic. 
        Organize findings into clear sections. Be objective and factual. Keep responses concise but informative.`)
            .temperature(0.4)
            .maxTokens(800)
            .build();

        console.log('  ✅ Researcher agent ready');

        // Analyst Agent
        this.agents.analyst = await new AgentBuilder('Analyst', llmConfig)
            .description('Analyzes data and identifies patterns')
            .systemPrompt(`You are an analytical expert. Analyze the provided information and identify key patterns, 
        trends, and insights. Draw meaningful conclusions. Provide data-driven recommendations. Be critical and thorough.`)
            .temperature(0.5)
            .maxTokens(800)
            .build();

        console.log('  ✅ Analyst agent ready');

        // Writer Agent
        this.agents.writer = await new AgentBuilder('Writer', llmConfig)
            .description('Creates polished written content')
            .systemPrompt(`You are a professional writer. Create well-structured, engaging content based on research 
        and analysis provided. Use clear, concise language. Ensure logical flow. Maintain professional tone.`)
            .temperature(0.7)
            .maxTokens(1000)
            .build();

        console.log('  ✅ Writer agent ready\n');

        return this.agents;
    }

    async research(topic) {
        console.log(`📚 Phase 1: Research`);
        console.log(`Topic: "${topic}"\n`);

        const prompt = `Research the following topic and provide a structured overview:

Topic: ${topic}

Please include:
- Brief introduction
- Key concepts and characteristics
- Important considerations
- Current relevance

Keep it concise (max 500 words).`;

        const research = await this.agents.researcher.execute(prompt);

        console.log('Research Results:');
        console.log('-'.repeat(70));
        console.log(research);
        console.log('-'.repeat(70) + '\n');

        return research;
    }

    async analyze(researchData, topic) {
        console.log(`🔬 Phase 2: Analysis\n`);

        const prompt = `Analyze the following research about ${topic}:

${researchData}

Provide:
- Key insights and patterns
- Strengths and opportunities
- Potential challenges
- Strategic recommendations

Keep analysis focused and actionable (max 400 words).`;

        const analysis = await this.agents.analyst.execute(prompt);

        console.log('Analysis Results:');
        console.log('-'.repeat(70));
        console.log(analysis);
        console.log('-'.repeat(70) + '\n');

        return analysis;
    }

    async writeReport(research, analysis, topic) {
        console.log(`✍️  Phase 3: Report Writing\n`);

        const prompt = `Create a concise report on "${topic}" using the materials below:

RESEARCH:
${research}

ANALYSIS:
${analysis}

Create a well-structured report with:
- Executive Summary (2-3 sentences)
- Key Findings (bullet points)
- Analysis Highlights
- Conclusion

Keep it professional and concise (max 600 words).`;

        const report = await this.agents.writer.execute(prompt);

        console.log('Final Report:');
        console.log('='.repeat(70));
        console.log(report);
        console.log('='.repeat(70) + '\n');

        return report;
    }

    async generateReport(topic) {
        console.log('\n🚀 Multi-Agent Collaboration Pipeline\n');
        console.log('='.repeat(70) + '\n');

        const startTime = Date.now();

        try {
            // Phase 1: Research
            const research = await this.research(topic);
            await new Promise(resolve => setTimeout(resolve, 1000));

            // Phase 2: Analysis
            const analysis = await this.analyze(research, topic);
            await new Promise(resolve => setTimeout(resolve, 1000));

            // Phase 3: Write Report
            const report = await this.writeReport(research, analysis, topic);

            const duration = ((Date.now() - startTime) / 1000).toFixed(2);

            console.log(`\n✅ Report generated successfully in ${duration}s\n`);

            return {
                topic,
                research,
                analysis,
                report,
                duration
            };

        } catch (error) {
            console.error('❌ Pipeline failed:', error.message);
            throw error;
        }
    }

    async parallelExecution(tasks) {
        console.log(`\n⚡ Parallel Execution Test\n`);
        console.log(`Running ${tasks.length} tasks simultaneously...\n`);

        const startTime = Date.now();

        const results = await Promise.all(
            tasks.map(async (task, idx) => {
                console.log(`  🔄 Starting task ${idx + 1}: ${task.title}`);

                const agent = this.agents[task.agent];
                const result = await agent.execute(task.prompt);

                console.log(`  ✅ Completed task ${idx + 1}`);

                return {
                    task: task.title,
                    agent: task.agent,
                    result: result.substring(0, 150) + '...'
                };
            })
        );

        const duration = ((Date.now() - startTime) / 1000).toFixed(2);

        console.log(`\n✅ All parallel tasks completed in ${duration}s\n`);

        results.forEach((result, idx) => {
            console.log(`Task ${idx + 1}: ${result.task}`);
            console.log(`Agent: ${result.agent}`);
            console.log(`Result: ${result.result}\n`);
        });

        return results;
    }
}

async function main() {
    console.log('🎭 Multi-Agent System Live Test\n');
    console.log('='.repeat(70) + '\n');

    try {
        init();
        console.log('✅ GraphBit initialized\n');

        const system = new MultiAgentSystem(OPENAI_API_KEY);

        // Initialize agents
        await system.initializeAgents();

        // Test 1: Sequential Pipeline
        console.log('\n📖 TEST 1: Sequential Research Pipeline\n');
        const reportResult = await system.generateReport('Artificial Intelligence in Healthcare');

        // Test 2: Parallel Execution
        console.log('\n📖 TEST 2: Parallel Task Execution\n');
        const parallelTasks = [
            {
                title: 'Summarize AI trends',
                agent: 'researcher',
                prompt: 'Provide a brief summary of current AI trends in 2024 (max 150 words)'
            },
            {
                title: 'Analyze AI market',
                agent: 'analyst',
                prompt: 'Analyze the AI market landscape briefly (max 150 words)'
            },
            {
                title: 'Write AI overview',
                agent: 'writer',
                prompt: 'Write a concise overview of AI applications (max 150 words)'
            }
        ];

        const parallelResults = await system.parallelExecution(parallelTasks);

        // Summary
        console.log('\n' + '='.repeat(70));
        console.log('\n📊 TEST SUMMARY\n');
        console.log(`Sequential Pipeline Duration: ${reportResult.duration}s`);
        console.log(`Parallel Execution: ${parallelTasks.length} tasks completed`);
        console.log('\n✨ Multi-Agent System test completed successfully!\n');

    } catch (error) {
        console.error('\n❌ Error:', error.message);
        console.error(error.stack);
        process.exit(1);
    }
}

main();
