/**
 * Animation System Demo - TypeScript Implementation
 * 
 * This example demonstrates the comprehensive animation system with various easing functions,
 * property animations, and timeline management for smooth TUI animations.
 * 
 * Features demonstrated:
 * - Multiple easing functions (bounce, elastic, back, exponential)
 * - Property animations (opacity, position, color, scale)
 * - Animation timelines and sequencing
 * - Parallel and sequential animations
 * - Animation state management and callbacks
 */

import {
    Animation,
    AnimationBuilder,
    AnimationManager,
    AnimationTimeline,
    AnimatedProperty,
    EasingFunction,
    EasingConfig,
    LoopMode,
    AnimationState,
    ColorDefinition,
    fadeIn,
    fadeOut,
    bounce,
    pulse,
    slideInLeft,
    colorTransition,
    scaleUp,
    rotate
} from '../../packages/tui-bun/src/widgets/animation';

/**
 * Main demo application class
 */
class AnimationDemo {
    private animationManager: AnimationManager;
    private startTime: number;
    private currentPhase: number = 0;
    private readonly phaseDuration: number = 6000; // 6 seconds per phase
    private readonly phases: string[] = [
        "Phase 1: Basic Fade Animations",
        "Phase 2: Easing Function Showcase", 
        "Phase 3: Property Animations",
        "Phase 4: Timeline Sequences",
        "Phase 5: Complex Compositions",
    ];

    constructor() {
        this.animationManager = new AnimationManager();
        this.startTime = performance.now();
        this.setupAnimations();
    }

    /**
     * Setup all demonstration animations
     */
    private setupAnimations(): void {
        // Phase 1: Basic fade animations
        this.createBasicFadeAnimations();
        
        // Phase 2: Easing function showcase
        this.createEasingShowcase();
        
        // Phase 3: Property animations
        this.createPropertyAnimations();
        
        // Phase 4: Timeline sequences
        this.createTimelineSequences();
        
        // Phase 5: Complex compositions
        this.createComplexCompositions();
    }

    /**
     * Create basic fade in/out animations
     */
    private createBasicFadeAnimations(): void {
        // Simple fade in
        const fadeInAnim = fadeIn("fade-in-demo", 1000);
        this.animationManager.addAnimation(fadeInAnim);

        // Fade out with callback
        const fadeOutAnim = new AnimationBuilder("fade-out-demo")
            .animateProperty({ type: 'opacity', from: 1, to: 0 })
            .duration(1500)
            .easing(EasingFunction.EaseOut)
            .delay(2000)
            .onComplete((animation) => {
                console.log(`Fade out completed: ${animation.getId()}`);
            })
            .build();
        this.animationManager.addAnimation(fadeOutAnim);
    }

    /**
     * Create showcase of different easing functions
     */
    private createEasingShowcase(): void {
        const easingFunctions: Array<[string, EasingFunction]> = [
            ["linear", EasingFunction.Linear],
            ["ease-in", EasingFunction.EaseIn],
            ["ease-out", EasingFunction.EaseOut],
            ["ease-in-out", EasingFunction.EaseInOut],
            ["bounce", EasingFunction.Bounce],
            ["elastic", EasingFunction.Elastic],
            ["back", EasingFunction.Back],
            ["exponential", EasingFunction.Expo],
            ["circular", EasingFunction.Circ],
            ["sine", EasingFunction.Sine],
        ];

        easingFunctions.forEach(([name, easing], i) => {
            const animation = new AnimationBuilder(`easing-${name}`)
                .animateProperty({ 
                    type: 'position', 
                    from: { x: 0, y: i * 3 }, 
                    to: { x: 50, y: i * 3 } 
                })
                .duration(2000)
                .easing(easing)
                .delay(500 + i * 200)
                .loopMode(LoopMode.PingPong)
                .onUpdate((_animation, values) => {
                    // Animation update callback for real-time visualization
                    if (values) {
                        // Handle animation values update
                    }
                })
                .build();
            
            this.animationManager.addAnimation(animation);
        });
    }

    /**
     * Create various property animations
     */
    private createPropertyAnimations(): void {
        // Color transition animation
        const colorAnim = new AnimationBuilder("color-transition")
            .animateProperty({ 
                type: 'color', 
                from: { r: 255, g: 0, b: 0 },    // Red
                to: { r: 0, g: 255, b: 255 }     // Cyan
            })
            .duration(3000)
            .easing(EasingFunction.EaseInOut)
            .loopMode(LoopMode.PingPong)
            .build();
        this.animationManager.addAnimation(colorAnim);

        // Scale animation
        const scaleAnim = new AnimationBuilder("scale-animation")
            .animateProperty({ type: 'scale', from: 1.0, to: 1.5 })
            .duration(1000)
            .easing(EasingFunction.Back)
            .loopMode(LoopMode.PingPong)
            .build();
        this.animationManager.addAnimation(scaleAnim);

        // Rotation animation
        const rotationAnim = new AnimationBuilder("rotation-animation")
            .animateProperty({ type: 'rotation', from: 0, to: 360 })
            .duration(4000)
            .easing(EasingFunction.Linear)
            .loopMode(LoopMode.Infinite)
            .build();
        this.animationManager.addAnimation(rotationAnim);

        // Multiple properties animation
        const multiAnim = new AnimationBuilder("multi-property")
            .animateProperty({ 
                type: 'multiple', 
                properties: [
                    { type: 'opacity', from: 0.5, to: 1.0 },
                    { type: 'scale', from: 0.8, to: 1.2 },
                    { type: 'position', from: { x: 10, y: 10 }, to: { x: 30, y: 30 } },
                ]
            })
            .duration(2500)
            .easing(EasingFunction.Bounce)
            .build();
        this.animationManager.addAnimation(multiAnim);
    }

    /**
     * Create timeline-based sequential animations
     */
    private createTimelineSequences(): void {
        // Sequential timeline
        const sequentialTimeline = new AnimationTimeline("sequential-demo", true);
        
        sequentialTimeline.addAnimation(fadeIn("seq-1", 500));
        sequentialTimeline.addAnimation(slideInLeft("seq-2", -20, 0, 10, 750));
        sequentialTimeline.addAnimation(bounce("seq-3", 1000));
        sequentialTimeline.addAnimation(fadeOut("seq-4", 500));
        
        this.animationManager.addTimeline(sequentialTimeline);

        // Parallel timeline
        const parallelTimeline = new AnimationTimeline("parallel-demo", false);
        
        parallelTimeline.addAnimation(pulse("par-1", 2000));
        parallelTimeline.addAnimation(
            colorTransition("par-2", 
                { r: 0, g: 255, b: 0 }, 
                { r: 255, g: 0, b: 255 }, 
                2000)
        );
        parallelTimeline.addAnimation(
            rotate("par-3", 0, 180, 2000)
        );
        
        this.animationManager.addTimeline(parallelTimeline);
    }

    /**
     * Create complex animation compositions with advanced effects
     */
    private createComplexCompositions(): void {
        // Staggered entrance animation
        for (let i = 0; i < 8; i++) {
            const staggerAnim = new AnimationBuilder(`stagger-${i}`)
                .animateProperty({
                    type: 'multiple',
                    properties: [
                        { type: 'opacity', from: 0, to: 1 },
                        { type: 'position', from: { x: -10, y: i * 4 }, to: { x: 0, y: i * 4 } },
                        { type: 'scale', from: 0.5, to: 1.0 },
                    ]
                })
                .duration(800)
                .easing(EasingFunction.Back)
                .delay(i * 100)
                .build();
            
            this.animationManager.addAnimation(staggerAnim);
        }

        // Wave effect animation
        const waveAnim = new AnimationBuilder("wave-effect")
            .animateProperty({
                type: 'custom',
                name: 'wave_phase',
                from: 0,
                to: 6.28 // 2π
            })
            .duration(3000)
            .easing(EasingFunction.Sine)
            .loopMode(LoopMode.Infinite)
            .onUpdate((_animation, _values) => {
                // Custom wave calculation would be applied here
            })
            .build();
        this.animationManager.addAnimation(waveAnim);

        // Elastic bounce sequence
        const elasticSequence = new AnimationBuilder("elastic-sequence")
            .animateProperty({
                type: 'multiple',
                properties: [
                    { type: 'position', from: { x: 40, y: 20 }, to: { x: 40, y: 5 } },
                    { type: 'scale', from: 1.0, to: 0.8 },
                ]
            })
            .duration(1200)
            .easing(EasingFunction.Elastic)
            .loopMode(LoopMode.Count, 3)
            .autoReverse(true)
            .onLoop((_animation, loopCount) => {
                console.log(`Elastic sequence loop ${loopCount} completed`);
            })
            .build();
        this.animationManager.addAnimation(elasticSequence);
    }

    /**
     * Update the demo - call this in your main loop
     */
    update(): void {
        // Update all animations
        this.animationManager.update();

        // Phase management
        const elapsed = performance.now() - this.startTime;
        const newPhase = Math.floor(elapsed / this.phaseDuration) % this.phases.length;
        
        if (newPhase !== this.currentPhase) {
            this.currentPhase = newPhase;
            console.log(`Entering: ${this.phases[this.currentPhase]}`);
            
            // Trigger phase-specific animations
            this.triggerPhaseAnimations(this.currentPhase);
        }
    }

    /**
     * Trigger animations for specific demo phase
     */
    private triggerPhaseAnimations(phase: number): void {
        switch (phase) {
            case 0: {
                // Start basic fade animations
                const fadeInAnim = this.animationManager.getAnimation("fade-in-demo");
                if (fadeInAnim) {
                    fadeInAnim.play();
                }
                break;
            }
            case 1: {
                // Start easing showcase
                const easingNames = ["linear", "ease-in", "ease-out", "ease-in-out", 
                                   "bounce", "elastic", "back", "exponential", "circular", "sine"];
                easingNames.forEach(name => {
                    const animation = this.animationManager.getAnimation(`easing-${name}`);
                    if (animation) {
                        animation.play();
                    }
                });
                break;
            }
            case 2: {
                // Start property animations
                const propertyAnimIds = ["color-transition", "scale-animation", "rotation-animation", "multi-property"];
                propertyAnimIds.forEach(id => {
                    const animation = this.animationManager.getAnimation(id);
                    if (animation) {
                        animation.play();
                    }
                });
                break;
            }
            case 3: {
                // Start timeline sequences
                // Timelines would be triggered through the timeline system
                break;
            }
            case 4: {
                // Start complex compositions
                for (let i = 0; i < 8; i++) {
                    const animation = this.animationManager.getAnimation(`stagger-${i}`);
                    if (animation) {
                        animation.play();
                    }
                }
                const waveAnim = this.animationManager.getAnimation("wave-effect");
                if (waveAnim) {
                    waveAnim.play();
                }
                const elasticAnim = this.animationManager.getAnimation("elastic-sequence");
                if (elasticAnim) {
                    elasticAnim.play();
                }
                break;
            }
        }
    }

    /**
     * Render the current demo state
     */
    render(): string {
        const elapsed = (performance.now() - this.startTime) / 1000;
        
        let output = '';
        
        // Header
        output += '🎬 TUI Animation System Demo\n';
        output += '━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n';
        output += `Current: ${this.phases[this.currentPhase]} | Active Animations: ${this.animationManager.getActiveCount()}\n\n`;

        // Demo status
        output += '📊 Animation Statistics:\n';
        output += `• Total Active: ${this.animationManager.getActiveCount()}\n`;
        output += `• Current Phase: ${this.currentPhase + 1}/${this.phases.length}\n`;
        output += `• Runtime: ${elapsed.toFixed(1)}s\n\n`;

        // Feature showcase
        output += '🎯 Features Demonstrated:\n';
        output += '• 15+ Easing Functions (Linear, Bounce, Elastic, Back, Exponential...)\n';
        output += '• Property Animations (Opacity, Position, Size, Color, Scale, Rotation)\n';
        output += '• Timeline Management (Sequential & Parallel)\n';
        output += '• Animation States (Play, Pause, Stop, Reverse, Loop)\n';
        output += '• Callback System (Start, Update, Complete, Loop events)\n';
        output += '• Performance Optimization (Frame-based timing, Efficient interpolation)\n\n';

        // Current phase details
        output += '📈 Current Phase Details:\n';
        switch (this.currentPhase) {
            case 0:
                output += 'Basic fade in/out animations with different durations and delays';
                break;
            case 1:
                output += 'Showcase of 10 different easing functions with position animations';
                break;
            case 2:
                output += 'Property animations: color transitions, scaling, rotation, multi-property';
                break;
            case 3:
                output += 'Timeline sequences: sequential and parallel animation coordination';
                break;
            case 4:
                output += 'Complex compositions: staggered entrances, wave effects, elastic sequences';
                break;
            default:
                output += 'Animation cycle complete - restarting demonstration';
        }
        output += '\n\n';

        // Usage example
        output += '💡 Usage Example:\n';
        output += '```typescript\n';
        output += 'const animation = new AnimationBuilder("my-animation")\n';
        output += '    .animateProperty({ type: "opacity", from: 0, to: 1 })\n';
        output += '    .duration(1000)\n';
        output += '    .easing(EasingFunction.Bounce)\n';
        output += '    .onComplete(anim => console.log("Done!"))\n';
        output += '    .build();\n';
        output += 'animation.play();\n';
        output += '```\n';

        return output;
    }

    /**
     * Get runtime statistics
     */
    getStats(): Record<string, any> {
        return {
            activeAnimations: this.animationManager.getActiveCount(),
            currentPhase: this.currentPhase + 1,
            totalPhases: this.phases.length,
            runtime: (performance.now() - this.startTime) / 1000,
            phaseName: this.phases[this.currentPhase]
        };
    }
}

/**
 * Demo runner function
 */
export function runAnimationDemo(): void {
    console.log('Starting TUI Animation System Demo (TypeScript)...\n');
    
    const demo = new AnimationDemo();
    let frameCount = 0;
    const maxFrames = 1800; // 30 seconds at 60fps
    
    const gameLoop = () => {
        // Update animations
        demo.update();
        
        // Render every 10th frame to avoid spam
        if (frameCount % 10 === 0) {
            console.clear();
            console.log(demo.render());
        }
        
        frameCount++;
        
        // Continue loop or exit
        if (frameCount < maxFrames) {
            setTimeout(gameLoop, 16); // ~60 FPS
        } else {
            console.log('\n🎉 Animation Demo Complete!');
            console.log('The animation system provides comprehensive support for:');
            console.log('• Smooth property transitions with 15+ easing functions');
            console.log('• Timeline management for complex sequences');
            console.log('• Performance-optimized frame-based timing');
            console.log('• Rich callback system for animation lifecycle events');
            console.log('• Full dual-language implementation (Rust + TypeScript)');
        }
    };
    
    // Start the demo
    gameLoop();
}

/**
 * Simple usage examples for testing
 */
export function createAnimationExamples(): Animation[] {
    const examples: Animation[] = [];
    
    // Example 1: Simple fade in
    examples.push(fadeIn("example-fade", 1000));
    
    // Example 2: Bounce animation
    examples.push(bounce("example-bounce", 1500));
    
    // Example 3: Color transition with proper typing
    const redColor: ColorDefinition = { r: 255, g: 0, b: 0 };
    const blueColor: ColorDefinition = { r: 0, g: 0, b: 255 };
    examples.push(colorTransition("example-color", redColor, blueColor, 2000));
    
    // Example 4: Scale up animation
    examples.push(scaleUp("example-scale", 0, 1, 800));

    // Example 5: Custom complex animation with proper typing
    const multiProperty: AnimatedProperty = {
        type: 'multiple',
        properties: [
            { type: 'opacity', from: 0, to: 1 },
            { type: 'scale', from: 0.5, to: 1.2 },
            { type: 'rotation', from: 0, to: 360 },
        ]
    };

    const customEasing: EasingConfig = {
        type: 'cubic-bezier',
        x1: 0.25,
        y1: 0.1,
        x2: 0.25,
        y2: 1
    };

    examples.push(
        new AnimationBuilder("example-complex")
            .animateProperty(multiProperty)
            .duration(2000)
            .easing(customEasing)
            .loopMode(LoopMode.PingPong)
            .onComplete((anim) => {
                console.log(`Complex animation ${anim.getId()} completed`);
                console.log(`Final state: ${AnimationState.Completed}`);
            })
            .build()
    );
    
    return examples;
}

// Export demo class for external usage
export { AnimationDemo };

// Run demo if called directly
if (typeof require !== 'undefined' && require.main === module) {
    runAnimationDemo();
}