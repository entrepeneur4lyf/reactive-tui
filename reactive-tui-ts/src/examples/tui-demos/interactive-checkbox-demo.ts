#!/usr/bin/env bun

/**
 * Interactive Checkbox TUI Demo
 *
 * This demo creates a fully interactive terminal interface where users can:
 * - Navigate between checkboxes using arrow keys
 * - Toggle checkboxes with spacebar or enter
 * - See real-time animations and state changes
 * - Experience the complete TUI interaction model
 *
 * NOTE: This is a conceptual demo showing the TUI interaction patterns.
 * The actual TUI framework implementation is still in development.
 */

class InteractiveCheckboxDemo {
    private currentFocus: number = 0;
    private checkboxStates: CheckboxState[] = [];
    private running: boolean = true;

    constructor() {
        this.initializeStates();
        this.setupKeyBindings();
    }

    private initializeStates() {
        this.checkboxStates = [
            { id: 'notifications', label: 'Enable notifications', checked: false, focused: false },
            { id: 'autosave', label: 'Auto-save documents', checked: true, focused: false },
            { id: 'darkmode', label: 'Dark mode', checked: false, focused: false },
            { id: 'sync', label: 'Cloud sync', checked: true, focused: false },
            { id: 'analytics', label: 'Usage analytics', checked: false, focused: false }
        ];
        this.updateFocus();
    }

    private setupKeyBindings() {
        // Set up stdin for raw mode to capture key presses
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(true);
            process.stdin.resume();
            process.stdin.setEncoding('utf8');

            process.stdin.on('data', (key: string) => {
                if (key === '\u0003' || key === 'q') { // Ctrl+C or 'q'
                    this.quit();
                } else if (key === '\u001b[A') { // Arrow Up
                    this.navigateUp();
                } else if (key === '\u001b[B') { // Arrow Down
                    this.navigateDown();
                } else if (key === ' ' || key === '\r') { // Space or Enter
                    this.toggleCurrentCheckbox();
                }
            });
        }
    }

    private updateFocus() {
        this.checkboxStates.forEach((checkbox, index) => {
            checkbox.focused = index === this.currentFocus;
        });
    }

    private navigateUp() {
        this.currentFocus = Math.max(0, this.currentFocus - 1);
        this.updateFocus();
        this.render();
    }

    private navigateDown() {
        this.currentFocus = Math.min(this.checkboxStates.length - 1, this.currentFocus + 1);
        this.updateFocus();
        this.render();
    }

    private toggleCurrentCheckbox() {
        if (this.currentFocus >= 0 && this.currentFocus < this.checkboxStates.length) {
            const checkbox = this.checkboxStates[this.currentFocus];
            checkbox.checked = !checkbox.checked;

            // Show animation effect
            this.renderWithAnimation(checkbox);

            // Return to normal after animation
            setTimeout(() => {
                this.render();
            }, 300);
        }
    }

    private renderWithAnimation(checkbox: CheckboxState) {
        // Clear screen and show animated state
        console.clear();
        console.log('🎯 Interactive Checkbox Demo - ANIMATING!');
        console.log('Use ↑↓ to navigate, SPACE/ENTER to toggle, Q to quit\n');

        this.checkboxStates.forEach((cb) => {
            const icon = cb === checkbox ? '✨✅✨' : (cb.checked ? '☑' : '☐');
            const prefix = cb.focused ? '→ ' : '  ';
            const style = cb.focused ? '\x1b[1m\x1b[33m' : ''; // Bold yellow for focused
            const reset = cb.focused ? '\x1b[0m' : '';

            console.log(`${style}${prefix}${icon} ${cb.label}${reset}`);
        });

        console.log('\n📊 Current Settings:');
        console.log(this.checkboxStates.map(cb => `${cb.id}: ${cb.checked ? '✅' : '❌'}`).join(' | '));
        console.log('\n💡 Watch the smooth scaling animations when toggling checkboxes!');
    }

    private render() {
        // Clear screen
        console.clear();

        // Header
        console.log('🎯 Interactive Checkbox Demo');
        console.log('Use ↑↓ to navigate, SPACE/ENTER to toggle, Q to quit\n');

        // Settings section
        console.log('⚙️ Settings:');
        this.checkboxStates.forEach((checkbox) => {
            const icon = checkbox.checked ? '☑' : '☐';
            const prefix = checkbox.focused ? '→ ' : '  ';
            const style = checkbox.focused ? '\x1b[1m\x1b[33m' : ''; // Bold yellow for focused
            const reset = checkbox.focused ? '\x1b[0m' : '';

            console.log(`${style}${prefix}${icon} ${checkbox.label}${reset}`);
        });

        // Status section
        console.log('\n📊 Current Settings:');
        console.log(this.checkboxStates.map(cb => `${cb.id}: ${cb.checked ? '✅' : '❌'}`).join(' | '));

        // Help
        console.log('\n💡 This demonstrates the TUI interaction patterns for checkbox widgets!');
        console.log('🎬 In the full implementation, you would see smooth scaling animations.');
    }

    private quit() {
        this.running = false;
        if (process.stdin.isTTY) {
            process.stdin.setRawMode(false);
        }
        console.clear();
        console.log('\n✅ Interactive Checkbox Demo completed!');
        console.log('📊 Final settings:');
        this.checkboxStates.forEach(checkbox => {
            console.log(`  ${checkbox.id}: ${checkbox.checked ? '✅ enabled' : '❌ disabled'}`);
        });
        process.exit(0);
    }

    public async start() {
        console.log('🚀 Starting Interactive Checkbox Demo...');
        console.log('📋 Features:');
        console.log('  • Arrow key navigation (↑↓)');
        console.log('  • Spacebar/Enter to toggle');
        console.log('  • Focus indicators');
        console.log('  • Real-time state updates');
        console.log('  • Conceptual TUI interaction patterns');
        console.log('');
        console.log('Press any key to start...');

        // Wait for initial keypress
        await new Promise<void>((resolve) => {
            const handler = () => {
                process.stdin.removeListener('data', handler);
                resolve();
            };
            process.stdin.once('data', handler);
        });

        this.render();

        // Keep the demo running until quit
        while (this.running) {
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    }
}

// Start the interactive demo
async function main() {
    const demo = new InteractiveCheckboxDemo();
    await demo.start();
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Demo interrupted by user');
    process.exit(0);
});

// Start the demo
main().catch(error => {
    console.error('💥 Demo failed:', error);
    process.exit(1);
});
