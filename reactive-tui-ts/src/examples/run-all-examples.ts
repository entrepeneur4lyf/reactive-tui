#!/usr/bin/env bun

/**
 * Master Example Runner - Run All TUI Framework Examples
 * 
 * This script runs all examples in the TUI framework to demonstrate
 * the complete feature set and ensure everything is working correctly.
 */

import { spawn } from 'bun';
import { readdirSync } from 'fs';
import { join } from 'path';

interface ExampleCategory {
    name: string;
    description: string;
    directory: string;
    examples: string[];
}

const categories: ExampleCategory[] = [
    {
        name: 'TUI Demos',
        description: 'Interactive terminal user interfaces',
        directory: 'tui-demos',
        examples: []
    },
    {
        name: 'API Demos',
        description: 'Component API demonstrations (JSON output)',
        directory: 'api-demos',
        examples: []
    }
];

async function discoverExamples() {
    console.log('🔍 Discovering examples...\n');

    // Get the examples directory path
    const examplesDir = join(process.cwd(), 'examples');

    for (const category of categories) {
        const dirPath = join(examplesDir, category.directory);

        try {
            const files = readdirSync(dirPath)
                .filter(file => file.endsWith('.ts'))
                .sort();

            category.examples = files;
            console.log(`📁 ${category.name}: ${files.length} examples found`);
        } catch (error) {
            console.log(`⚠️  ${category.name}: Directory not found at ${dirPath}`);
            console.debug(error);
        }
    }

    console.log();
}

async function runExample(category: string, example: string): Promise<boolean> {
    const examplePath = join('examples', category, example);
    console.log(`🚀 Running: ${examplePath}`);

    try {
        const proc = spawn(['bun', 'run', examplePath], {
            cwd: process.cwd(),
            stdio: ['inherit', 'pipe', 'pipe']
        });
        
        const result = await proc.exited;
        
        if (result === 0) {
            console.log(`✅ ${example} completed successfully\n`);
            return true;
        } else {
            console.log(`❌ ${example} failed with exit code ${result}\n`);
            return false;
        }
    } catch (error) {
        console.log(`💥 ${example} crashed: ${error}\n`);
        return false;
    }
}

async function runCategoryExamples(category: ExampleCategory, interactive: boolean = false) {
    console.log(`\n🎯 ${category.name}`);
    console.log(`📝 ${category.description}`);
    console.log('─'.repeat(50));
    
    if (category.examples.length === 0) {
        console.log('📭 No examples found in this category\n');
        return { total: 0, passed: 0, failed: 0 };
    }
    
    let passed = 0;
    let failed = 0;
    
    for (const example of category.examples) {
        if (interactive) {
            console.log(`\n❓ Run ${example}? (y/n/q): `);
            // In a real implementation, you'd wait for user input
            // For now, we'll just run all examples
        }
        
        const success = await runExample(category.directory, example);
        if (success) {
            passed++;
        } else {
            failed++;
        }
        
        // Add a small delay between examples
        await new Promise(resolve => setTimeout(resolve, 1000));
    }
    
    return { total: category.examples.length, passed, failed };
}

async function runAllExamples(interactive: boolean = false) {
    console.log('🎉 TUI Framework - Complete Example Suite\n');
    
    await discoverExamples();
    
    let totalExamples = 0;
    let totalPassed = 0;
    let totalFailed = 0;
    
    for (const category of categories) {
        const results = await runCategoryExamples(category, interactive);
        totalExamples += results.total;
        totalPassed += results.passed;
        totalFailed += results.failed;
    }
    
    // Summary
    console.log('\n' + '='.repeat(60));
    console.log('📊 EXAMPLE EXECUTION SUMMARY');
    console.log('='.repeat(60));
    console.log(`📋 Total Examples: ${totalExamples}`);
    console.log(`✅ Passed: ${totalPassed}`);
    console.log(`❌ Failed: ${totalFailed}`);
    console.log(`📈 Success Rate: ${totalExamples > 0 ? Math.round((totalPassed / totalExamples) * 100) : 0}%`);
    
    if (totalFailed === 0) {
        console.log('\n🎉 All examples completed successfully!');
        console.log('🚀 The TUI framework is working perfectly!');
    } else {
        console.log(`\n⚠️  ${totalFailed} examples failed. Please check the output above.`);
    }
    
    console.log('\n🔗 For more information, see examples/README.md');
}

async function runSpecificCategory(categoryName: string) {
    await discoverExamples();
    
    const category = categories.find(cat => 
        cat.name.toLowerCase().includes(categoryName.toLowerCase()) ||
        cat.directory.toLowerCase() === categoryName.toLowerCase()
    );
    
    if (!category) {
        console.log(`❌ Category '${categoryName}' not found.`);
        console.log('Available categories:');
        categories.forEach(cat => console.log(`  - ${cat.directory} (${cat.name})`));
        return;
    }
    
    await runCategoryExamples(category);
}

async function listExamples() {
    await discoverExamples();
    
    console.log('📋 Available Examples:\n');
    
    for (const category of categories) {
        console.log(`🎯 ${category.name}`);
        console.log(`   ${category.description}`);
        
        if (category.examples.length === 0) {
            console.log('   📭 No examples found');
        } else {
            category.examples.forEach(example => {
                console.log(`   📄 ${category.directory}/${example}`);
            });
        }
        console.log();
    }
}

// Command line interface
async function main() {
    const args = process.argv.slice(2);
    const command = args[0];
    
    switch (command) {
        case 'list':
        case 'ls':
            await listExamples();
            break;
            
        case 'tui-demos':
        case 'tui':
        case 'api-demos':
        case 'api':
        case 'components':
            await runSpecificCategory(command);
            break;

        case 'interactive':
        case '-i':
            await runAllExamples(true);
            break;
            
        case 'help':
        case '-h':
        case '--help':
            console.log('🎯 TUI Framework Example Runner\n');
            console.log('Usage:');
            console.log('  bun run examples/run-all-examples.ts [command]\n');
            console.log('Commands:');
            console.log('  (no args)    Run all examples');
            console.log('  list         List all available examples');
            console.log('  tui-demos    Run interactive TUI demonstrations');
            console.log('  api-demos    Run component API examples');
            console.log('  interactive  Run with interactive prompts');
            console.log('  help         Show this help message\n');
            console.log('Examples:');
            console.log('  bun run examples/run-all-examples.ts');
            console.log('  bun run examples/run-all-examples.ts tui-demos');
            console.log('  bun run examples/run-all-examples.ts api-demos');
            console.log('  bun run examples/run-all-examples.ts list');
            break;
            
        default:
            await runAllExamples();
            break;
    }
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Example runner interrupted by user');
    process.exit(0);
});

// Start the runner
main().catch(error => {
    console.error('💥 Example runner failed:', error);
    process.exit(1);
});
