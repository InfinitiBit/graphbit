import 'dotenv/config';
import { init, ToolRegistry } from 'graphbit';

console.log('Testing ToolRegistry API...');
init();

const registry = new ToolRegistry();

registry.register('add', 'Add two numbers', {
    a: { type: 'number' },
    b: { type: 'number' }
}, async (args: any) => args.a + args.b);

console.log('Tool registered with function API');

const hasTool = registry.hasTool('add');
console.log('hasTool:', hasTool);

const tools = registry.getRegisteredTools();
console.log('getRegisteredTools:', tools);

const count = registry.getToolCount();
console.log('getToolCount:', count);

registry.execute('add', { a: 5, b: 3 }).then(result => {
    console.log('Execute result:', result);
    console.log('\\n✅ All API calls successful!');
});
