/**
 * Document Loader Example
 * 
 * Demonstrates loading and processing documents in various formats.
 */

import { DocumentLoader } from '../index';
import * as fs from 'fs';
import * as path from 'path';

async function main() {
    console.log('Document Loader Example\n');

    // Create a sample text file for demonstration
    const testFile = path.join(__dirname, 'sample.txt');
    fs.writeFileSync(testFile, 'This is a sample document for testing.\nIt has multiple lines.\nGraphBit can process this!');

    try {
        // Basic usage - Default configuration
        const loader = new DocumentLoader();

        // Load a text document
        const content = await loader.loadText(
            'This is a sample text content',
            'inline-source'
        );

        console.log('Loaded from text:');
        console.log('  Content:', content.content);
        console.log('  Source:', content.source);
        console.log('  Type:', content.documentType);

        // Load from file
        const fileContent = await loader.loadFile(testFile, 'txt');
        console.log('\nLoaded from file:');
        console.log('  Content length:', fileContent.content.length);
        console.log('  Source:', fileContent.source);

        // Custom configuration
        const customLoader = DocumentLoader.withConfig({
            maxFileSize: 10_000_000, // 10MB
            defaultEncoding: 'utf-8',
            preserveFormatting: true
        });

        const customContent = await customLoader.loadFile(testFile, 'txt');
        console.log('\nLoaded with custom config:');
        console.log('  Success:', customContent.content.length > 0);

        // Cleanup
        fs.unlinkSync(testFile);
        console.log('\nExample completed successfully!');

    } catch (error) {
        console.error('Error:', error);
        // Cleanup on error
        if (fs.existsSync(testFile)) {
            fs.unlinkSync(testFile);
        }
    }
}

main().catch(console.error);
