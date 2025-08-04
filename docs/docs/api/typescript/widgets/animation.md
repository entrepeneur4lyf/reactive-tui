# Animation Widget

The Animation widget provides a comprehensive animation system with smooth property transitions, advanced easing functions (bounce, elastic, back, etc.), timeline management for sequential and parallel animations, frame-based timing, and complete animation lifecycle control for creating dynamic, interactive terminal interfaces.

## Basic Usage

```typescript
import { Animation, AnimationBuilder, EasingFunction, LoopMode } from 'reactive-tui';

// Basic fade animation
const fadeAnimation = new AnimationBuilder('fade-in')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(500)
  .easing(EasingFunction.EaseOut)
  .onComplete((animation) => {
    console.log('Fade animation completed');
  })
  .build();

// Start the animation
fadeAnimation.play();

// Using convenience functions
const fadeInAnim = fadeIn('quick-fade', 300);
const fadeOutAnim = fadeOut('fade-away', 500);
```

## Configuration

### AnimationConfig Interface

```typescript
interface AnimationConfig {
  duration: number;                // Animation duration in milliseconds (default: 500)
  easing: EasingConfig;           // Easing function (default: EaseInOut)
  delay: number;                  // Start delay in milliseconds (default: 0)
  loop_mode: LoopMode;           // Loop behavior (default: None)
  loop_count?: number;           // Number of loops for Count mode
  reverse: boolean;              // Play in reverse (default: false)
  speed: number;                 // Playback speed multiplier (default: 1.0)
  auto_play: boolean;            // Start automatically (default: false)
  auto_reverse: boolean;         // Auto-reverse on loop (default: false)
}
```

### AnimatedProperty Interface

```typescript
interface AnimatedProperty {
  type: 'opacity' | 'position' | 'size' | 'color' | 'scale' | 'rotation' | 'custom' | 'multiple';
  from?: any;                    // Starting value
  to?: any;                      // Ending value
  name?: string;                 // Property name for custom animations
  properties?: AnimatedProperty[]; // Multiple properties for 'multiple' type
}
```

### Easing Functions

```typescript
enum EasingFunction {
  Linear = 'linear',            // Constant speed
  EaseIn = 'ease-in',          // Slow start
  EaseOut = 'ease-out',        // Slow end
  EaseInOut = 'ease-in-out',   // Slow start and end
  Bounce = 'bounce',           // Bouncing effect
  Elastic = 'elastic',         // Elastic spring effect
  Back = 'back',               // Overshoot and return
  Expo = 'expo',               // Exponential acceleration
  Circ = 'circ',               // Circular motion
  Sine = 'sine',               // Sine wave easing
  Quad = 'quad',               // Quadratic easing
  Cubic = 'cubic',             // Cubic easing
  Quart = 'quart',             // Quartic easing
  Quint = 'quint'              // Quintic easing
}

// Custom cubic bezier easing
interface CubicBezierEasing {
  type: 'cubic-bezier';
  x1: number;                   // Control point 1 x
  y1: number;                   // Control point 1 y
  x2: number;                   // Control point 2 x
  y2: number;                   // Control point 2 y
}
```

### Loop Modes

```typescript
enum LoopMode {
  None = 'none',               // Play once
  Infinite = 'infinite',       // Loop forever
  Count = 'count',             // Loop specific number of times
  PingPong = 'ping-pong'       // Reverse direction each loop
}
```

## Core Features

### Property Animation Types

```typescript
// Opacity animation (fade in/out)
const opacityAnim = new AnimationBuilder('opacity')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(600)
  .easing(EasingFunction.EaseOut)
  .build();

// Position animation (movement)
const positionAnim = new AnimationBuilder('move')
  .animateProperty({
    type: 'position',
    from: { x: 0, y: 0 },
    to: { x: 100, y: 50 }
  })
  .duration(800)
  .easing(EasingFunction.EaseInOut)
  .build();

// Size animation (scaling/resizing)
const sizeAnim = new AnimationBuilder('resize')
  .animateProperty({
    type: 'size',
    from: { width: 50, height: 20 },
    to: { width: 100, height: 40 }
  })
  .duration(400)
  .easing(EasingFunction.Back)
  .build();

// Color animation (color transitions)
const colorAnim = new AnimationBuilder('color-change')
  .animateProperty({
    type: 'color',
    from: { r: 255, g: 0, b: 0 },      // Red
    to: { r: 0, g: 255, b: 0 }         // Green
  })
  .duration(1000)
  .easing(EasingFunction.EaseInOut)
  .build();

// Scale animation (uniform scaling)
const scaleAnim = new AnimationBuilder('scale')
  .animateProperty({ type: 'scale', from: 0.5, to: 1.5 })
  .duration(500)
  .easing(EasingFunction.Elastic)
  .build();

// Rotation animation (spinning)
const rotationAnim = new AnimationBuilder('spin')
  .animateProperty({ type: 'rotation', from: 0, to: 360 })
  .duration(2000)
  .easing(EasingFunction.Linear)
  .loopMode(LoopMode.Infinite)
  .build();

// Custom property animation
const customAnim = new AnimationBuilder('custom')
  .animateProperty({
    type: 'custom',
    name: 'brightness',
    from: 0.3,
    to: 1.0
  })
  .duration(750)
  .easing(EasingFunction.Sine)
  .build();

// Multiple properties simultaneously
const multiAnim = new AnimationBuilder('multi')
  .animateProperty({
    type: 'multiple',
    properties: [
      { type: 'opacity', from: 0, to: 1 },
      { type: 'scale', from: 0.8, to: 1.0 },
      { type: 'position', from: { x: -10, y: 0 }, to: { x: 0, y: 0 } }
    ]
  })
  .duration(600)
  .easing(EasingFunction.EaseOut)
  .build();
```

### Animation Playback Control

```typescript
const animation = new AnimationBuilder('controllable')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(1000)
  .build();

// Playback controls
animation.play();          // Start animation
animation.pause();         // Pause animation
animation.stop();          // Stop and reset
animation.reverse();       // Reverse direction

// State queries
console.log('Is playing:', animation.isPlaying());
console.log('Is completed:', animation.isCompleted());
console.log('Current progress:', animation.getProgress()); // 0.0 to 1.0
console.log('Current state:', animation.getState());

// Seek to specific progress
animation.seek(0.5);       // Jump to 50% progress

// Change playback speed
animation.setSpeed(2.0);   // Double speed
animation.setSpeed(0.5);   // Half speed

// Get current animated values
const currentValues = animation.getCurrentValues();
console.log('Current values:', currentValues);
```

### Advanced Easing Functions

```typescript
// Built-in easing functions
const easingExamples = [
  { name: 'Bounce', easing: EasingFunction.Bounce },
  { name: 'Elastic', easing: EasingFunction.Elastic },
  { name: 'Back', easing: EasingFunction.Back },
  { name: 'Exponential', easing: EasingFunction.Expo },
  { name: 'Circular', easing: EasingFunction.Circ }
];

easingExamples.forEach(({ name, easing }) => {
  const anim = new AnimationBuilder(`${name.toLowerCase()}-demo`)
    .animateProperty({ type: 'position', from: { x: 0, y: 0 }, to: { x: 100, y: 0 } })
    .duration(1000)
    .easing(easing)
    .onComplete((animation) => {
      console.log(`${name} animation completed`);
    })
    .build();
  
  anim.play();
});

// Custom cubic bezier easing
const customEasing: CubicBezierEasing = {
  type: 'cubic-bezier',
  x1: 0.25,
  y1: 0.1,
  x2: 0.25,
  y2: 1.0
};

const customEasingAnim = new AnimationBuilder('custom-easing')
  .animateProperty({ type: 'scale', from: 1, to: 1.5 })
  .duration(800)
  .easing(customEasing)
  .build();
```

### Loop Animations

```typescript
// Infinite loop
const infiniteLoop = new AnimationBuilder('infinite-pulse')
  .animateProperty({ type: 'opacity', from: 1, to: 0.3 })
  .duration(800)
  .easing(EasingFunction.EaseInOut)
  .loopMode(LoopMode.Infinite)
  .build();

// Count-based loop
const countLoop = new AnimationBuilder('three-bounces')
  .animateProperty({ type: 'scale', from: 1, to: 1.2 })
  .duration(400)
  .easing(EasingFunction.Bounce)
  .loopMode(LoopMode.Count, 3)  // Loop 3 times
  .onLoop((animation, loopCount) => {
    console.log(`Completed loop ${loopCount}`);
  })
  .build();

// Ping-pong loop (reverse direction each time)
const pingPong = new AnimationBuilder('ping-pong-move')
  .animateProperty({
    type: 'position',
    from: { x: 0, y: 0 },
    to: { x: 50, y: 0 }
  })
  .duration(1000)
  .easing(EasingFunction.EaseInOut)
  .loopMode(LoopMode.PingPong)
  .autoReverse(true)
  .build();
```

## Animation Timeline Management

```typescript
import { AnimationTimeline, AnimationManager } from 'reactive-tui';

// Sequential timeline (animations play one after another)
const sequentialTimeline = new AnimationTimeline('intro-sequence', true);

const fadeIn = new AnimationBuilder('fade-in')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(500)
  .build();

const slideIn = new AnimationBuilder('slide-in')
  .animateProperty({
    type: 'position',
    from: { x: -100, y: 0 },
    to: { x: 0, y: 0 }
  })
  .duration(600)
  .easing(EasingFunction.EaseOut)
  .build();

const scaleUp = new AnimationBuilder('scale-up')
  .animateProperty({ type: 'scale', from: 0.8, to: 1.0 })
  .duration(400)
  .easing(EasingFunction.Back)
  .build();

// Add animations to timeline
sequentialTimeline.addAnimation(fadeIn);
sequentialTimeline.addAnimation(slideIn);
sequentialTimeline.addAnimation(scaleUp);

// Play the sequence
sequentialTimeline.play();

// Parallel timeline (animations play simultaneously)
const parallelTimeline = new AnimationTimeline('entrance-effect', false);

const fadeInParallel = new AnimationBuilder('parallel-fade')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(800)
  .build();

const bounceInParallel = new AnimationBuilder('parallel-bounce')
  .animateProperty({ type: 'scale', from: 0.5, to: 1.0 })
  .duration(800)
  .easing(EasingFunction.Bounce)
  .build();

const colorFadeParallel = new AnimationBuilder('parallel-color')
  .animateProperty({
    type: 'color',
    from: { r: 100, g: 100, b: 100 },
    to: { r: 255, g: 255, b: 255 }
  })
  .duration(800)
  .build();

parallelTimeline.addAnimation(fadeInParallel);
parallelTimeline.addAnimation(bounceInParallel);
parallelTimeline.addAnimation(colorFadeParallel);

parallelTimeline.play();
```

### Animation Manager

```typescript
// Global animation manager
const animationManager = new AnimationManager();

// Add animations to manager
const managedAnimation1 = pulse('pulse-1', 1000);
const managedAnimation2 = bounce('bounce-1', 800);

animationManager.addAnimation(managedAnimation1);
animationManager.addAnimation(managedAnimation2);
animationManager.addTimeline(sequentialTimeline);

// Update animations in main loop
function gameLoop() {
  animationManager.update(); // Updates all animations with delta time
  
  // Check active animation count
  console.log('Active animations:', animationManager.getActiveCount());
  
  requestAnimationFrame(gameLoop);
}

gameLoop();

// Utility methods
animationManager.cleanupCompleted(); // Remove completed animations
animationManager.stopAll();          // Stop all animations
```

## Builder Pattern

```typescript
// Comprehensive animation builder
const advancedAnimation = new AnimationBuilder('advanced-demo')
  .animateProperty({
    type: 'multiple',
    properties: [
      { type: 'opacity', from: 0, to: 1 },
      { type: 'position', from: { x: -50, y: 10 }, to: { x: 0, y: 0 } },
      { type: 'scale', from: 0.8, to: 1.0 },
      { type: 'color', from: { r: 150, g: 150, b: 150 }, to: { r: 255, g: 255, b: 255 } }
    ]
  })
  .duration(1200)
  .easing(EasingFunction.EaseOut)
  .delay(300)                    // Wait 300ms before starting
  .speed(1.5)                    // Play at 1.5x speed
  .loopMode(LoopMode.Count, 2)   // Loop twice
  .autoReverse(true)             // Reverse on each loop
  .onStart((animation) => {
    console.log('Advanced animation started');
  })
  .onUpdate((animation, values) => {
    console.log('Animation progress:', animation.getProgress());
    console.log('Current values:', values);
  })
  .onLoop((animation, loopCount) => {
    console.log(`Loop ${loopCount} completed`);
  })
  .onComplete((animation) => {
    console.log('Advanced animation completed');
  })
  .build();

advancedAnimation.play();
```

## Convenience Functions

```typescript
// Pre-built animation functions for common effects

// Fade animations
const fadeInAnim = fadeIn('fade-in', 500);
const fadeOutAnim = fadeOut('fade-out', 300);

// Slide animations
const slideLeft = slideInLeft('slide-left', -100, 0, 20, 600);

// Bounce animation
const bounceAnim = bounce('bounce-effect', 1000);

// Pulse animation
const pulseAnim = pulse('pulse-effect', 800);

// Color transition
const colorChange = colorTransition('color-change', 
  { r: 255, g: 0, b: 0 },     // From red
  { r: 0, g: 0, b: 255 },     // To blue
  750
);

// Scale animation
const scaleUpAnim = scaleUp('scale-up', 0, 1.2, 400);

// Rotation animation
const rotateAnim = rotate('rotate-360', 0, 360, 2000);

// Start multiple animations
[fadeInAnim, bounceAnim, pulseAnim].forEach(anim => anim.play());
```

## Real-World Examples

### Loading Animation System

```typescript
import { Animation, AnimationBuilder, AnimationManager, EasingFunction, LoopMode } from 'reactive-tui';

class LoadingAnimationSystem {
  private manager: AnimationManager;
  private loadingAnimations: Map<string, Animation> = new Map();
  private isActive: boolean = false;
  
  constructor() {
    this.manager = new AnimationManager();
    this.setupLoadingAnimations();
  }
  
  private setupLoadingAnimations() {
    // Spinner animation
    const spinner = new AnimationBuilder('spinner')
      .animateProperty({ type: 'rotation', from: 0, to: 360 })
      .duration(1000)
      .easing(EasingFunction.Linear)
      .loopMode(LoopMode.Infinite)
      .onUpdate((animation, values) => {
        this.updateSpinner(values.value);
      })
      .build();
    
    // Pulsing dots
    const dots = new AnimationBuilder('loading-dots')
      .animateProperty({ type: 'opacity', from: 0.3, to: 1.0 })
      .duration(600)
      .easing(EasingFunction.EaseInOut)
      .loopMode(LoopMode.PingPong)
      .onUpdate((animation, values) => {
        this.updateDots(values.value);
      })
      .build();
    
    // Progress bar fill
    const progressBar = new AnimationBuilder('progress-fill')
      .animateProperty({ type: 'scale', from: 0, to: 1 })
      .duration(3000)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateProgressBar(values.value);
      })
      .onComplete((animation) => {
        this.onLoadingComplete();
      })
      .build();
    
    // Breathing effect
    const breathing = new AnimationBuilder('breathing')
      .animateProperty({ type: 'scale', from: 0.95, to: 1.05 })
      .duration(2000)
      .easing(EasingFunction.EaseInOut)
      .loopMode(LoopMode.PingPong)
      .onUpdate((animation, values) => {
        this.updateBreathing(values.value);
      })
      .build();
    
    this.loadingAnimations.set('spinner', spinner);
    this.loadingAnimations.set('dots', dots);
    this.loadingAnimations.set('progress', progressBar);
    this.loadingAnimations.set('breathing', breathing);
    
    // Add to manager
    this.loadingAnimations.forEach(anim => this.manager.addAnimation(anim));
  }
  
  startLoading(type: 'spinner' | 'dots' | 'progress' | 'breathing' = 'spinner') {
    if (this.isActive) this.stopLoading();
    
    this.isActive = true;
    const animation = this.loadingAnimations.get(type);
    
    if (animation) {
      animation.play();
      console.log(`Started ${type} loading animation`);
    }
    
    // Start update loop
    this.updateLoop();
  }
  
  stopLoading() {
    this.isActive = false;
    this.manager.stopAll();
    console.log('Stopped all loading animations');
  }
  
  setProgress(progress: number) {
    const progressAnim = this.loadingAnimations.get('progress');
    if (progressAnim) {
      progressAnim.seek(Math.max(0, Math.min(1, progress)));
    }
  }
  
  private updateLoop() {
    if (!this.isActive) return;
    
    this.manager.update();
    
    // Continue loop
    requestAnimationFrame(() => this.updateLoop());
  }
  
  private updateSpinner(rotation: number) {
    // Update spinner visual (rotation angle)
    const spinnerChars = ['|', '/', '-', '\\'];
    const charIndex = Math.floor((rotation / 90) % 4);
    console.log(`Loading ${spinnerChars[charIndex]}`);
  }
  
  private updateDots(opacity: number) {
    // Update loading dots opacity
    const dotCount = Math.floor(opacity * 3) + 1;
    const dots = '.'.repeat(dotCount);
    console.log(`Loading${dots}`);
  }
  
  private updateProgressBar(scale: number) {
    // Update progress bar fill
    const percentage = Math.round(scale * 100);
    const filled = Math.floor(scale * 20);
    const empty = 20 - filled;
    const bar = '█'.repeat(filled) + '░'.repeat(empty);
    console.log(`Progress: [${bar}] ${percentage}%`);
  }
  
  private updateBreathing(scale: number) {
    // Update breathing effect scale
    const size = Math.round(scale * 10);
    console.log(`Loading... (${'●'.repeat(size)})`);
  }
  
  private onLoadingComplete() {
    console.log('Loading completed!');
    this.isActive = false;
  }
  
  // Public methods for external control
  pauseLoading() {
    this.loadingAnimations.forEach(anim => anim.pause());
  }
  
  resumeLoading() {
    this.loadingAnimations.forEach(anim => {
      if (anim.getState() === 'paused') {
        anim.play();
      }
    });
  }
  
  getLoadingProgress(): number {
    const progressAnim = this.loadingAnimations.get('progress');
    return progressAnim ? progressAnim.getProgress() : 0;
  }
  
  isLoading(): boolean {
    return this.isActive;
  }
}

// Usage
const loadingSystem = new LoadingAnimationSystem();

// Start different loading animations
loadingSystem.startLoading('spinner');

// Simulate loading progress
setTimeout(() => loadingSystem.setProgress(0.3), 1000);
setTimeout(() => loadingSystem.setProgress(0.6), 2000);
setTimeout(() => loadingSystem.setProgress(1.0), 3000);

// Switch loading styles
setTimeout(() => {
  loadingSystem.stopLoading();
  loadingSystem.startLoading('dots');
}, 4000);
```

### UI Transition System

```typescript
interface TransitionConfig {
  duration: number;
  easing: EasingFunction;
  stagger?: number;
}

class UITransitionSystem {
  private manager: AnimationManager;
  private transitionQueue: Animation[] = [];
  private elements: Map<string, any> = new Map();
  
  constructor() {
    this.manager = new AnimationManager();
    this.startUpdateLoop();
  }
  
  // Register UI elements for animation
  registerElement(id: string, element: any) {
    this.elements.set(id, element);
  }
  
  // Page transition animations
  slidePageTransition(fromPage: string, toPage: string, direction: 'left' | 'right' = 'left') {
    const slideDistance = direction === 'left' ? -100 : 100;
    
    // Slide out current page
    const slideOut = new AnimationBuilder(`${fromPage}-out`)
      .animateProperty({
        type: 'position',
        from: { x: 0, y: 0 },
        to: { x: -slideDistance, y: 0 }
      })
      .duration(300)
      .easing(EasingFunction.EaseIn)
      .onUpdate((animation, values) => {
        this.updateElementPosition(fromPage, values.value);
      })
      .build();
    
    // Slide in new page
    const slideIn = new AnimationBuilder(`${toPage}-in`)
      .animateProperty({
        type: 'position',
        from: { x: slideDistance, y: 0 },
        to: { x: 0, y: 0 }
      })
      .duration(300)
      .easing(EasingFunction.EaseOut)
      .delay(150) // Start halfway through slide out
      .onUpdate((animation, values) => {
        this.updateElementPosition(toPage, values.value);
      })
      .onComplete(() => {
        console.log(`Page transition to ${toPage} completed`);
      })
      .build();
    
    // Start both animations
    slideOut.play();
    slideIn.play();
    
    this.manager.addAnimation(slideOut);
    this.manager.addAnimation(slideIn);
  }
  
  // Staggered list item animations
  animateListItems(itemIds: string[], config: TransitionConfig = {
    duration: 400,
    easing: EasingFunction.EaseOut,
    stagger: 100
  }) {
    itemIds.forEach((itemId, index) => {
      const delay = (config.stagger || 0) * index;
      
      const itemAnimation = new AnimationBuilder(`list-item-${index}`)
        .animateProperty({
          type: 'multiple',
          properties: [
            { type: 'opacity', from: 0, to: 1 },
            { type: 'position', from: { x: -20, y: 0 }, to: { x: 0, y: 0 } },
            { type: 'scale', from: 0.95, to: 1.0 }
          ]
        })
        .duration(config.duration)
        .delay(delay)
        .easing(config.easing)
        .onUpdate((animation, values) => {
          this.updateElementMultiple(itemId, values);
        })
        .build();
      
      itemAnimation.play();
      this.manager.addAnimation(itemAnimation);
    });
  }
  
  // Modal entrance animation
  showModal(modalId: string) {
    // Background fade
    const backdropFade = new AnimationBuilder('modal-backdrop')
      .animateProperty({ type: 'opacity', from: 0, to: 0.7 })
      .duration(200)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateBackdrop(values.value);
      })
      .build();
    
    // Modal scale and fade
    const modalEntrance = new AnimationBuilder(`${modalId}-entrance`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 0, to: 1 },
          { type: 'scale', from: 0.8, to: 1.0 },
          { type: 'position', from: { x: 0, y: -20 }, to: { x: 0, y: 0 } }
        ]
      })
      .duration(350)
      .delay(100)
      .easing(EasingFunction.Back)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(modalId, values);
      })
      .onComplete(() => {
        console.log('Modal entrance animation completed');
      })
      .build();
    
    backdropFade.play();
    modalEntrance.play();
    
    this.manager.addAnimation(backdropFade);
    this.manager.addAnimation(modalEntrance);
  }
  
  // Modal exit animation
  hideModal(modalId: string) {
    const modalExit = new AnimationBuilder(`${modalId}-exit`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 1, to: 0 },
          { type: 'scale', from: 1.0, to: 0.9 },
          { type: 'position', from: { x: 0, y: 0 }, to: { x: 0, y: 10 } }
        ]
      })
      .duration(250)
      .easing(EasingFunction.EaseIn)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(modalId, values);
      })
      .onComplete(() => {
        this.hideElement(modalId);
      })
      .build();
    
    const backdropFade = new AnimationBuilder('modal-backdrop-exit')
      .animateProperty({ type: 'opacity', from: 0.7, to: 0 })
      .duration(200)
      .delay(150)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateBackdrop(values.value);
      })
      .build();
    
    modalExit.play();
    backdropFade.play();
    
    this.manager.addAnimation(modalExit);
    this.manager.addAnimation(backdropFade);
  }
  
  // Button interaction animations
  animateButtonPress(buttonId: string) {
    const pressDown = new AnimationBuilder(`${buttonId}-press`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'scale', from: 1.0, to: 0.95 },
          { type: 'color', from: { r: 100, g: 100, b: 100 }, to: { r: 80, g: 80, b: 80 } }
        ]
      })
      .duration(100)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(buttonId, values);
      })
      .onComplete(() => {
        // Immediate release animation
        this.animateButtonRelease(buttonId);
      })
      .build();
    
    pressDown.play();
    this.manager.addAnimation(pressDown);
  }
  
  private animateButtonRelease(buttonId: string) {
    const release = new AnimationBuilder(`${buttonId}-release`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'scale', from: 0.95, to: 1.0 },
          { type: 'color', from: { r: 80, g: 80, b: 80 }, to: { r: 100, g: 100, b: 100 } }
        ]
      })
      .duration(150)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(buttonId, values);
      })
      .build();
    
    release.play();
    this.manager.addAnimation(release);
  }
  
  // Notification animations
  showNotification(notificationId: string, type: 'success' | 'error' | 'info' = 'info') {
    const colors = {
      success: { r: 72, g: 187, b: 120 },
      error: { r: 239, g: 68, b: 68 },
      info: { r: 59, g: 130, b: 246 }
    };
    
    const slideIn = new AnimationBuilder(`${notificationId}-slide-in`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'position', from: { x: 100, y: 0 }, to: { x: 0, y: 0 } },
          { type: 'opacity', from: 0, to: 1 },
          { type: 'color', from: { r: 200, g: 200, b: 200 }, to: colors[type] }
        ]
      })
      .duration(400)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(notificationId, values);
      })
      .onComplete(() => {
        // Auto-hide after 3 seconds
        setTimeout(() => this.hideNotification(notificationId), 3000);
      })
      .build();
    
    slideIn.play();
    this.manager.addAnimation(slideIn);
  }
  
  private hideNotification(notificationId: string) {
    const slideOut = new AnimationBuilder(`${notificationId}-slide-out`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'position', from: { x: 0, y: 0 }, to: { x: 100, y: 0 } },
          { type: 'opacity', from: 1, to: 0 }
        ]
      })
      .duration(300)
      .easing(EasingFunction.EaseIn)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(notificationId, values);
      })
      .onComplete(() => {
        this.hideElement(notificationId);
      })
      .build();
    
    slideOut.play();
    this.manager.addAnimation(slideOut);
  }
  
  // Hover animations
  animateHover(elementId: string, isEntering: boolean) {
    const targetScale = isEntering ? 1.05 : 1.0;
    const targetOpacity = isEntering ? 1.0 : 0.9;
    
    const hover = new AnimationBuilder(`${elementId}-hover-${isEntering ? 'in' : 'out'}`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'scale', from: isEntering ? 1.0 : 1.05, to: targetScale },
          { type: 'opacity', from: isEntering ? 0.9 : 1.0, to: targetOpacity }
        ]
      })
      .duration(200)
      .easing(EasingFunction.EaseOut)
      .onUpdate((animation, values) => {
        this.updateElementMultiple(elementId, values);
      })
      .build();
    
    hover.play();
    this.manager.addAnimation(hover);
  }
  
  // Helper methods
  private updateElementPosition(elementId: string, position: { x: number; y: number }) {
    const element = this.elements.get(elementId);
    if (element) {
      console.log(`Element ${elementId} position:`, position);
      // Update actual element position
    }
  }
  
  private updateElementMultiple(elementId: string, values: any) {
    const element = this.elements.get(elementId);
    if (element && values.values) {
      console.log(`Element ${elementId} multiple values:`, values.values);
      // Update multiple element properties
    }
  }
  
  private updateBackdrop(opacity: number) {
    console.log('Backdrop opacity:', opacity);
    // Update backdrop opacity
  }
  
  private hideElement(elementId: string) {
    console.log(`Hiding element: ${elementId}`);
    // Hide element completely
  }
  
  private startUpdateLoop() {
    const update = () => {
      this.manager.update();
      requestAnimationFrame(update);
    };
    update();
  }
  
  // Public utilities
  stopAllAnimations() {
    this.manager.stopAll();
  }
  
  getActiveAnimationCount(): number {
    return this.manager.getActiveCount();
  }
  
  cleanup() {
    this.manager.cleanupCompleted();
  }
}

// Usage
const uiTransitions = new UITransitionSystem();

// Register UI elements
uiTransitions.registerElement('main-page', { /* element data */ });
uiTransitions.registerElement('settings-page', { /* element data */ });

// Animate page transitions
uiTransitions.slidePageTransition('main-page', 'settings-page', 'left');

// Animate list items with stagger
const listItems = ['item-1', 'item-2', 'item-3', 'item-4', 'item-5'];
uiTransitions.animateListItems(listItems, {
  duration: 500,
  easing: EasingFunction.EaseOut,
  stagger: 150
});

// Show modal with animation
uiTransitions.showModal('confirmation-modal');

// Animate button interactions
uiTransitions.animateButtonPress('submit-button');

// Show notifications
uiTransitions.showNotification('success-notification', 'success');
uiTransitions.showNotification('error-notification', 'error');

// Hover effects
uiTransitions.animateHover('nav-item', true);  // Mouse enter
uiTransitions.animateHover('nav-item', false); // Mouse leave
```

### Interactive Widget Animator

```typescript
class InteractiveWidgetAnimator {
  private manager: AnimationManager;
  private widgets: Map<string, any> = new Map();
  private activeAnimations: Map<string, string[]> = new Map(); // widgetId -> animationIds
  
  constructor() {
    this.manager = new AnimationManager();
    this.startAnimationLoop();
  }
  
  // Register widgets for animation
  registerWidget(id: string, widget: any) {
    this.widgets.set(id, widget);
    this.activeAnimations.set(id, []);
  }
  
  // Attention-grabbing animations
  drawAttention(widgetId: string, type: 'pulse' | 'shake' | 'glow' | 'bounce' = 'pulse') {
    this.stopWidgetAnimations(widgetId);
    
    let animation: Animation;
    
    switch (type) {
      case 'pulse':
        animation = new AnimationBuilder(`${widgetId}-pulse`)
          .animateProperty({
            type: 'multiple',
            properties: [
              { type: 'scale', from: 1.0, to: 1.1 },
              { type: 'opacity', from: 1.0, to: 0.7 }
            ]
          })
          .duration(600)
          .easing(EasingFunction.EaseInOut)
          .loopMode(LoopMode.PingPong)
          .loopMode(LoopMode.Count, 3)
          .onUpdate((anim, values) => this.updateWidget(widgetId, values))
          .build();
        break;
        
      case 'shake':
        animation = new AnimationBuilder(`${widgetId}-shake`)
          .animateProperty({
            type: 'position',
            from: { x: -3, y: 0 },
            to: { x: 3, y: 0 }
          })
          .duration(100)
          .easing(EasingFunction.Linear)
          .loopMode(LoopMode.PingPong)
          .loopMode(LoopMode.Count, 6)
          .onUpdate((anim, values) => this.updateWidgetPosition(widgetId, values.value))
          .build();
        break;
        
      case 'glow':
        animation = new AnimationBuilder(`${widgetId}-glow`)
          .animateProperty({
            type: 'color',
            from: { r: 100, g: 100, b: 100 },
            to: { r: 255, g: 215, b: 0 } // Gold glow
          })
          .duration(800)
          .easing(EasingFunction.EaseInOut)
          .loopMode(LoopMode.PingPong)
          .loopMode(LoopMode.Count, 2)
          .onUpdate((anim, values) => this.updateWidgetColor(widgetId, values.value))
          .build();
        break;
        
      case 'bounce':
        animation = new AnimationBuilder(`${widgetId}-bounce`)
          .animateProperty({ type: 'scale', from: 1.0, to: 1.2 })
          .duration(400)
          .easing(EasingFunction.Bounce)
          .loopMode(LoopMode.Count, 2)
          .onUpdate((anim, values) => this.updateWidgetScale(widgetId, values.value))
          .build();
        break;
    }
    
    this.addWidgetAnimation(widgetId, animation);
    animation.play();
  }
  
  // State transition animations
  animateStateTransition(widgetId: string, fromState: string, toState: string) {
    const transitionMap = {
      'disabled-to-enabled': () => this.createEnableAnimation(widgetId),
      'enabled-to-disabled': () => this.createDisableAnimation(widgetId),
      'inactive-to-active': () => this.createActivateAnimation(widgetId),
      'active-to-inactive': () => this.createDeactivateAnimation(widgetId),
      'normal-to-hover': () => this.createHoverEnterAnimation(widgetId),
      'hover-to-normal': () => this.createHoverExitAnimation(widgetId),
      'normal-to-pressed': () => this.createPressAnimation(widgetId),
      'pressed-to-normal': () => this.createReleaseAnimation(widgetId)
    };
    
    const transitionKey = `${fromState}-to-${toState}`;
    const animationFactory = transitionMap[transitionKey];
    
    if (animationFactory) {
      this.stopWidgetAnimations(widgetId);
      const animation = animationFactory();
      this.addWidgetAnimation(widgetId, animation);
      animation.play();
    }
  }
  
  // Loading state animations
  showLoadingState(widgetId: string, style: 'spinner' | 'pulse' | 'dots' = 'spinner') {
    this.stopWidgetAnimations(widgetId);
    
    let loadingAnimation: Animation;
    
    switch (style) {
      case 'spinner':
        loadingAnimation = new AnimationBuilder(`${widgetId}-loading-spinner`)
          .animateProperty({ type: 'rotation', from: 0, to: 360 })
          .duration(1000)
          .easing(EasingFunction.Linear)
          .loopMode(LoopMode.Infinite)
          .onUpdate((anim, values) => this.updateWidgetRotation(widgetId, values.value))
          .build();
        break;
        
      case 'pulse':
        loadingAnimation = new AnimationBuilder(`${widgetId}-loading-pulse`)
          .animateProperty({ type: 'opacity', from: 0.5, to: 1.0 })
          .duration(800)
          .easing(EasingFunction.EaseInOut)
          .loopMode(LoopMode.PingPong)
          .onUpdate((anim, values) => this.updateWidgetOpacity(widgetId, values.value))
          .build();
        break;
        
      case 'dots':
        loadingAnimation = new AnimationBuilder(`${widgetId}-loading-dots`)
          .animateProperty({ type: 'custom', name: 'dotCount', from: 1, to: 3 })
          .duration(1200)
          .easing(EasingFunction.Linear)
          .loopMode(LoopMode.Infinite)
          .onUpdate((anim, values) => this.updateLoadingDots(widgetId, values.value))
          .build();
        break;
    }
    
    this.addWidgetAnimation(widgetId, loadingAnimation);
    loadingAnimation.play();
  }
  
  hideLoadingState(widgetId: string) {
    this.stopWidgetAnimations(widgetId);
    
    // Fade back to normal state
    const fadeIn = new AnimationBuilder(`${widgetId}-loading-end`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 0.5, to: 1.0 },
          { type: 'scale', from: 0.9, to: 1.0 }
        ]
      })
      .duration(300)
      .easing(EasingFunction.EaseOut)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .build();
    
    this.addWidgetAnimation(widgetId, fadeIn);
    fadeIn.play();
  }
  
  // Entrance/exit animations
  animateEntrance(widgetId: string, direction: 'top' | 'bottom' | 'left' | 'right' | 'center' = 'center') {
    const startPositions = {
      top: { x: 0, y: -50 },
      bottom: { x: 0, y: 50 },
      left: { x: -50, y: 0 },
      right: { x: 50, y: 0 },
      center: { x: 0, y: 0 }
    };
    
    const entrance = new AnimationBuilder(`${widgetId}-entrance`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 0, to: 1 },
          { type: 'scale', from: 0.8, to: 1.0 },
          { type: 'position', from: startPositions[direction], to: { x: 0, y: 0 } }
        ]
      })
      .duration(500)
      .easing(EasingFunction.EaseOut)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .onComplete(() => console.log(`Widget ${widgetId} entrance completed`))
      .build();
    
    this.addWidgetAnimation(widgetId, entrance);
    entrance.play();
  }
  
  animateExit(widgetId: string, direction: 'top' | 'bottom' | 'left' | 'right' | 'center' = 'center') {
    const endPositions = {
      top: { x: 0, y: -50 },
      bottom: { x: 0, y: 50 },
      left: { x: -50, y: 0 },
      right: { x: 50, y: 0 },
      center: { x: 0, y: 0 }
    };
    
    const exit = new AnimationBuilder(`${widgetId}-exit`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 1, to: 0 },
          { type: 'scale', from: 1.0, to: 0.8 },
          { type: 'position', from: { x: 0, y: 0 }, to: endPositions[direction] }
        ]
      })
      .duration(400)
      .easing(EasingFunction.EaseIn)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .onComplete(() => {
        console.log(`Widget ${widgetId} exit completed`);
        this.hideWidget(widgetId);
      })
      .build();
    
    this.addWidgetAnimation(widgetId, exit);
    exit.play();
  }
  
  // Animation factory methods
  private createEnableAnimation(widgetId: string): Animation {
    return new AnimationBuilder(`${widgetId}-enable`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 0.5, to: 1.0 },
          { type: 'color', from: { r: 150, g: 150, b: 150 }, to: { r: 255, g: 255, b: 255 } }
        ]
      })
      .duration(300)
      .easing(EasingFunction.EaseOut)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .build();
  }
  
  private createDisableAnimation(widgetId: string): Animation {
    return new AnimationBuilder(`${widgetId}-disable`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'opacity', from: 1.0, to: 0.5 },
          { type: 'color', from: { r: 255, g: 255, b: 255 }, to: { r: 150, g: 150, b: 150 } }
        ]
      })
      .duration(300)
      .easing(EasingFunction.EaseIn)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .build();
  }
  
  private createHoverEnterAnimation(widgetId: string): Animation {
    return new AnimationBuilder(`${widgetId}-hover-enter`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'scale', from: 1.0, to: 1.05 },
          { type: 'color', from: { r: 200, g: 200, b: 200 }, to: { r: 255, g: 255, b: 255 } }
        ]
      })
      .duration(200)
      .easing(EasingFunction.EaseOut)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .build();
  }
  
  private createPressAnimation(widgetId: string): Animation {
    return new AnimationBuilder(`${widgetId}-press`)
      .animateProperty({
        type: 'multiple',
        properties: [
          { type: 'scale', from: 1.0, to: 0.95 },
          { type: 'opacity', from: 1.0, to: 0.8 }
        ]
      })
      .duration(100)
      .easing(EasingFunction.EaseOut)
      .onUpdate((anim, values) => this.updateWidget(widgetId, values))
      .build();
  }
  
  // Widget update methods
  private updateWidget(widgetId: string, values: any) {
    const widget = this.widgets.get(widgetId);
    if (widget) {
      console.log(`Updating widget ${widgetId}:`, values);
      // Apply animation values to widget
    }
  }
  
  private updateWidgetPosition(widgetId: string, position: { x: number; y: number }) {
    console.log(`Widget ${widgetId} position:`, position);
  }
  
  private updateWidgetColor(widgetId: string, color: { r: number; g: number; b: number }) {
    console.log(`Widget ${widgetId} color:`, color);
  }
  
  private updateWidgetScale(widgetId: string, scale: number) {
    console.log(`Widget ${widgetId} scale:`, scale);
  }
  
  private updateWidgetOpacity(widgetId: string, opacity: number) {
    console.log(`Widget ${widgetId} opacity:`, opacity);
  }
  
  private updateWidgetRotation(widgetId: string, rotation: number) {
    console.log(`Widget ${widgetId} rotation:`, rotation);
  }
  
  private updateLoadingDots(widgetId: string, dotCount: number) {
    const dots = '.'.repeat(Math.floor(dotCount));
    console.log(`Widget ${widgetId} loading dots:`, dots);
  }
  
  private hideWidget(widgetId: string) {
    console.log(`Hiding widget: ${widgetId}`);
  }
  
  // Animation management
  private addWidgetAnimation(widgetId: string, animation: Animation) {
    const animationId = animation.getId();
    this.activeAnimations.get(widgetId)?.push(animationId);
    this.manager.addAnimation(animation);
  }
  
  private stopWidgetAnimations(widgetId: string) {
    const animations = this.activeAnimations.get(widgetId) || [];
    animations.forEach(animId => {
      const animation = this.manager.getAnimation(animId);
      if (animation) {
        animation.stop();
        this.manager.removeAnimation(animId);
      }
    });
    this.activeAnimations.set(widgetId, []);
  }
  
  private startAnimationLoop() {
    const update = () => {
      this.manager.update();
      requestAnimationFrame(update);
    };
    update();
  }
  
  // Public utilities
  stopAllWidgetAnimations() {
    this.manager.stopAll();
    this.activeAnimations.forEach((_, widgetId) => {
      this.activeAnimations.set(widgetId, []);
    });
  }
  
  isWidgetAnimating(widgetId: string): boolean {
    const animations = this.activeAnimations.get(widgetId) || [];
    return animations.length > 0;
  }
  
  getActiveAnimationCount(): number {
    return this.manager.getActiveCount();
  }
}

// Usage
const widgetAnimator = new InteractiveWidgetAnimator();

// Register widgets
widgetAnimator.registerWidget('button-1', { /* button widget */ });
widgetAnimator.registerWidget('input-1', { /* input widget */ });
widgetAnimator.registerWidget('modal-1', { /* modal widget */ });

// Draw attention to important elements
widgetAnimator.drawAttention('button-1', 'pulse');
widgetAnimator.drawAttention('input-1', 'shake');

// Animate state transitions
widgetAnimator.animateStateTransition('button-1', 'normal', 'hover');
widgetAnimator.animateStateTransition('button-1', 'hover', 'pressed');

// Show loading states
widgetAnimator.showLoadingState('button-1', 'spinner');
setTimeout(() => widgetAnimator.hideLoadingState('button-1'), 3000);

// Animate widget entrance/exit
widgetAnimator.animateEntrance('modal-1', 'center');
setTimeout(() => widgetAnimator.animateExit('modal-1', 'top'), 5000);
```

## Performance Considerations

```typescript
// Performance monitoring
const performanceAnimator = new AnimationManager();

// Monitor frame rate
let frameCount = 0;
let lastFpsTime = performance.now();

function updateAnimations() {
  const startTime = performance.now();
  
  performanceAnimator.update();
  
  const updateTime = performance.now() - startTime;
  
  // Monitor performance
  frameCount++;
  if (performance.now() - lastFpsTime >= 1000) {
    const fps = frameCount;
    console.log(`FPS: ${fps}, Update time: ${updateTime.toFixed(2)}ms`);
    
    if (fps < 30) {
      console.warn('Low frame rate detected - consider reducing animation complexity');
    }
    
    frameCount = 0;
    lastFpsTime = performance.now();
  }
  
  requestAnimationFrame(updateAnimations);
}

updateAnimations();

// Optimize animations for performance
const optimizedAnimation = new AnimationBuilder('optimized')
  .animateProperty({ type: 'opacity', from: 0, to: 1 })
  .duration(300)                    // Shorter durations
  .easing(EasingFunction.Linear)    // Simple easing functions
  .onUpdate((animation, values) => {
    // Batch DOM updates
    // Use requestAnimationFrame for visual updates
    // Minimize property calculations
  })
  .build();

// Cleanup completed animations
setInterval(() => {
  performanceAnimator.cleanupCompleted();
}, 5000);
```

## Best Practices

1. **Performance Optimization**
   - Use shorter durations for frequent animations
   - Prefer simple easing functions for real-time interactions
   - Clean up completed animations regularly
   - Monitor frame rates and adjust complexity

2. **User Experience**
   - Use appropriate easing for different contexts
   - Provide visual feedback for user interactions
   - Keep animations purposeful and not distracting
   - Allow users to disable animations if needed

3. **Animation Design**
   - Use consistent timing and easing across the interface
   - Layer animations for complex effects
   - Test animations at different speeds and devices
   - Provide fallbacks for reduced motion preferences

4. **Code Organization**
   - Use the builder pattern for complex animations
   - Group related animations in timelines
   - Create reusable animation functions
   - Separate animation logic from UI logic

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const animatedContainer = new ElementBuilderImpl('div')
  .class('animated-container')
  .child(
    new AnimationBuilder('container-fade')
      .animateProperty({ type: 'opacity', from: 0, to: 1 })
      .duration(500)
      .autoPlay(true)
      .build()
  )
  .build();
```

The Animation widget provides comprehensive animation capabilities with smooth property transitions, advanced easing functions, timeline management, and complete animation lifecycle control for creating dynamic, engaging terminal user interfaces.