#!/usr/bin/env bun

/**
 * Example Runner for Reactive TUI TypeScript
 * 
 * This script allows you to easily run individual examples from the command line.
 * 
 * Usage:
 *   bun run run-example.ts [example-name]
 *   bun run run-example.ts --list
 *   bun run run-example.ts --help
 */

import { EXAMPLES, runExample } from './src/examples'

const EXAMPLE_DESCRIPTIONS = {
  helloWorld: 'Simple Hello World demonstration',
  buttons: 'Interactive button examples with different variants',
  cards: 'Grid-based card layout with responsive design',
  dashboard: 'Comprehensive dashboard with metrics and data visualization',
  toasts: 'Toast notification system demonstration',
  themes: 'Color themes and styling showcase'
}

function showHelp(): void {
  console.log(`
🚀 Reactive TUI TypeScript Examples Runner

Usage:
  bun run run-example.ts [example-name]    Run a specific example
  bun run run-example.ts --list            List all available examples
  bun run run-example.ts --help            Show this help message

Examples:
  bun run run-example.ts helloWorld        Run the Hello World example
  bun run run-example.ts dashboard         Run the dashboard example
  bun run run-example.ts buttons           Run the button examples

Available Examples:`)

  Object.entries(EXAMPLE_DESCRIPTIONS).forEach(([name, description]) => {
    console.log(`  ${name.padEnd(15)} - ${description}`)
  })

  console.log(`
For more information, see EXAMPLES_GUIDE.md
`)
}

function listExamples(): void {
  console.log(`
📋 Available Examples:

`)
  Object.entries(EXAMPLE_DESCRIPTIONS).forEach(([name, description]) => {
    console.log(`🎯 ${name}`)
    console.log(`   ${description}`)
    console.log(`   Command: bun run run-example.ts ${name}`)
    console.log()
  })
}

function main(): void {
  const args = process.argv.slice(2)
  
  if (args.length === 0) {
    console.log('❌ No example specified. Use --help for usage information.')
    process.exit(1)
  }

  const command = args[0]

  switch (command) {
    case '--help':
    case '-h':
      showHelp()
      break

    case '--list':
    case '-l':
      listExamples()
      break

    default:
      if (command in EXAMPLES) {
        console.log(`🚀 Running example: ${command}`)
        console.log(`📝 Description: ${EXAMPLE_DESCRIPTIONS[command as keyof typeof EXAMPLE_DESCRIPTIONS]}`)
        console.log()
        
        try {
          runExample(command as keyof typeof EXAMPLES)
        } catch (error) {
          console.error('❌ Error running example:', error)
          process.exit(1)
        }
      } else {
        console.log(`❌ Unknown example: ${command}`)
        console.log('📋 Available examples:')
        Object.keys(EXAMPLES).forEach(name => {
          console.log(`   - ${name}`)
        })
        console.log('\nUse --help for more information.')
        process.exit(1)
      }
      break
  }
}

if (import.meta.main) {
  main()
}
