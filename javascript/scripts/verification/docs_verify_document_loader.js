const { DocumentLoader } = require('../javascript/index');
const fs = require('fs');
const path = require('path');

async function verifyDocumentLoaderUsage() {
    console.log('Verifying DocumentLoader for Documentation...');

    // Create test files
    const testDir = path.join(__dirname, '../test_docs');
    if (!fs.existsSync(testDir)) {
        fs.mkdirSync(testDir, { recursive: true });
    }

    const txtFile = path.join(testDir, 'test.txt');
    const jsonFile = path.join(testDir, 'test.json');

    fs.writeFileSync(txtFile, 'This is a test document with some content for verification.');
    fs.writeFileSync(jsonFile, JSON.stringify({ message: 'Test JSON document', data: [1, 2, 3] }));

    console.log('✅ Test files created');

    // 1. Test basic DocumentLoader constructor
    try {
        const loader = new DocumentLoader();
        console.log('✅ DocumentLoader() constructor works');
    } catch (error) {
        console.error('❌ Basic constructor failed:', error);
        process.exit(1);
    }

    // 2. Test withConfig static method
    try {
        const loaderWithConfig = DocumentLoader.withConfig({
            maxFileSize: 10000000,
            defaultEncoding: 'utf-8',
            preserveFormatting: true
        });
        console.log('✅ DocumentLoader.withConfig() works');

        // Verify config
        const config = loaderWithConfig.config();
        console.log('  Config:', JSON.stringify(config, null, 2));
    } catch (error) {
        console.error('❌ withConfig failed:', error);
        process.exit(1);
    }

    // 3. Test loadFile method with text file
    try {
        const loader = new DocumentLoader();
        const content = await loader.loadFile(txtFile, 'txt');

        console.log('✅ loadFile() works for text files');
        console.log('  Content preview:', content.content.substring(0, 50));
        console.log('  Source:', content.source);
        console.log('  Type:', content.documentType);

        // Verify DocumentContent structure
        if (content.content && content.source && content.documentType) {
            console.log('✅ DocumentContent structure verified');
        } else {
            throw new Error('DocumentContent structure invalid');
        }
    } catch (error) {
        console.error('❌ loadFile (txt) failed:', error);
        process.exit(1);
    }

    // 4. Test loadFile with JSON
    try {
        const loader = new DocumentLoader();
        const content = await loader.loadFile(jsonFile, 'json');

        console.log('✅ loadFile() works for JSON files');
        console.log('  Content preview:', content.content.substring(0, 50));
    } catch (error) {
        console.error('❌ loadFile (json) failed:', error);
        process.exit(1);
    }

    // 5. Test loadText method
    try {
        const loader = new DocumentLoader();
        const content = await loader.loadText('This is test text', 'test-source');

        console.log('✅ loadText() works');
        console.log('  Content:', content.content);
        console.log('  Source:', content.source);

        // Test with no source
        const content2 = await loader.loadText('Test without source');
        console.log('✅ loadText() works without source argument');
    } catch (error) {
        console.error('❌ loadText failed:', error);
        process.exit(1);
    }

    // 6. Test config() method
    try {
        const loader = new DocumentLoader();
        const config = loader.config();
        console.log('✅ config() method works');
        console.log('  Default config:', JSON.stringify(config, null, 2));
    } catch (error) {
        console.error('❌ config() method failed:', error);
        process.exit(1);
    }

    // 7. Test metadata in DocumentContent
    try {
        const loader = new DocumentLoader();
        const content = await loader.loadFile(txtFile, 'txt');

        console.log('\n📊 DocumentContent details:');
        console.log('  Content length:', content.content.length);
        console.log('  Source:', content.source);
        console.log('  Type:', content.documentType);
        console.log('  Metadata:', content.metadata || 'null');
    } catch (error) {
        console.error('❌ Metadata test failed:', error);
    }

    // Cleanup
    try {
        fs.unlinkSync(txtFile);
        fs.unlinkSync(jsonFile);
        fs.rmdirSync(testDir);
        console.log('\n✅ Test files cleaned up');
    } catch (error) {
        console.warn('⚠️ Cleanup warning:', error.message);
    }

    console.log('\n✨ All DocumentLoader methods verified successfully!');
}

verifyDocumentLoaderUsage().catch(console.error);
