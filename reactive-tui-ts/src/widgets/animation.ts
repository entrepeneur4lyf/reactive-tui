/**
 * Animation System Widget - TypeScript Implementation
 * 
 * A comprehensive animation system providing smooth transitions, easing functions,
 * and property animations for TUI widgets with frame-based timing and interpolation.
 * 
 * Features:
 * - Property animations (opacity, position, size, colors, custom properties)
 * - Advanced easing functions (linear, cubic bezier, bounce, elastic, etc.)
 * - Timeline management for sequential and parallel animations
 * - Frame-based timing with smooth 60fps animations
 * - Animation states (play, pause, stop, reverse, loop controls)
 * - Value interpolation between keyframes
 * - Performance optimization with dirty region tracking
 * - Event callbacks for animation lifecycle events
 */

export type AnimationId = string;
export type TimelineId = string;

export enum EasingFunction {
  Linear = 'linear',
  EaseIn = 'ease-in',
  EaseOut = 'ease-out',
  EaseInOut = 'ease-in-out',
  Bounce = 'bounce',
  Elastic = 'elastic',
  Back = 'back',
  Expo = 'expo',
  Circ = 'circ',
  Sine = 'sine',
  Quad = 'quad',
  Cubic = 'cubic',
  Quart = 'quart',
  Quint = 'quint'
}

export interface CubicBezierEasing {
  type: 'cubic-bezier';
  x1: number;
  y1: number;
  x2: number;
  y2: number;
}

export type EasingConfig = EasingFunction | CubicBezierEasing;

export interface ColorDefinition {
  r: number;
  g: number;
  b: number;
}

export interface AnimatedProperty {
  type: 'opacity' | 'position' | 'size' | 'color' | 'scale' | 'rotation' | 'custom' | 'multiple';
  from?: any;
  to?: any;
  name?: string;
  properties?: AnimatedProperty[];
}

export interface AnimatedValue {
  type: 'opacity' | 'position' | 'size' | 'color' | 'scale' | 'rotation' | 'custom' | 'multiple';
  value?: any;
  name?: string;
  values?: AnimatedValue[];
}

export enum AnimationState {
  Stopped = 'stopped',
  Playing = 'playing',
  Paused = 'paused',
  Completed = 'completed',
  Reversed = 'reversed'
}

export enum LoopMode {
  None = 'none',
  Infinite = 'infinite',
  Count = 'count',
  PingPong = 'ping-pong'
}

export interface AnimationConfig {
  duration: number; // milliseconds
  easing: EasingConfig;
  delay: number;
  loop_mode: LoopMode;
  loop_count?: number;
  reverse: boolean;
  speed: number;
  auto_play: boolean;
  auto_reverse: boolean;
}

export interface AnimationRuntimeState {
  state: AnimationState;
  current_time: number;
  loops_completed: number;
  is_reversed: boolean;
  current_values: AnimatedValue | null;
  progress: number;
}

export interface AnimationCallbacks {
  onStart?: (animation: Animation) => void;
  onUpdate?: (animation: Animation, values: AnimatedValue) => void;
  onComplete?: (animation: Animation) => void;
  onLoop?: (animation: Animation, loopCount: number) => void;
  onPause?: (animation: Animation) => void;
  onStop?: (animation: Animation) => void;
}

export class EasingHelper {
  static apply(easing: EasingConfig, t: number): number {
    t = Math.max(0, Math.min(1, t));

    if (typeof easing === 'object' && easing.type === 'cubic-bezier') {
      // Simplified cubic bezier approximation
      const { y1, y2 } = easing;
      const t2 = t * t;
      const t3 = t2 * t;
      return 3 * (1 - t) * (1 - t) * t * y1 + 3 * (1 - t) * t2 * y2 + t3;
    }

    switch (easing) {
      case EasingFunction.Linear:
        return t;
      case EasingFunction.EaseIn:
        return t * t;
      case EasingFunction.EaseOut:
        return t * (2 - t);
      case EasingFunction.EaseInOut:
        return t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
      case EasingFunction.Bounce:
        if (t < 1 / 2.75) {
          return 7.5625 * t * t;
        } else if (t < 2 / 2.75) {
          const t2 = t - 1.5 / 2.75;
          return 7.5625 * t2 * t2 + 0.75;
        } else if (t < 2.5 / 2.75) {
          const t2 = t - 2.25 / 2.75;
          return 7.5625 * t2 * t2 + 0.9375;
        } else {
          const t2 = t - 2.625 / 2.75;
          return 7.5625 * t2 * t2 + 0.984375;
        }
      case EasingFunction.Elastic:
        if (t === 0 || t === 1) return t;
        const p = 0.3;
        const s = p / 4;
        return -(Math.pow(2, 10 * (t - 1))) * Math.sin((t - 1 - s) * (2 * Math.PI) / p);
      case EasingFunction.Back:
        const c1 = 1.70158;
        const c3 = c1 + 1;
        return c3 * t * t * t - c1 * t * t;
      case EasingFunction.Expo:
        return t === 0 ? 0 : Math.pow(2, 10 * (t - 1));
      case EasingFunction.Circ:
        return 1 - Math.sqrt(1 - t * t);
      case EasingFunction.Sine:
        return 1 - Math.cos(t * Math.PI / 2);
      case EasingFunction.Quad:
        return t * t;
      case EasingFunction.Cubic:
        return t * t * t;
      case EasingFunction.Quart:
        return t * t * t * t;
      case EasingFunction.Quint:
        return t * t * t * t * t;
      default:
        return t;
    }
  }
}

export class PropertyInterpolator {
  static interpolate(property: AnimatedProperty, t: number): AnimatedValue {
    switch (property.type) {
      case 'opacity':
        return {
          type: 'opacity',
          value: property.from + (property.to - property.from) * t
        };
      case 'position':
        return {
          type: 'position',
          value: {
            x: property.from.x + (property.to.x - property.from.x) * t,
            y: property.from.y + (property.to.y - property.from.y) * t
          }
        };
      case 'size':
        return {
          type: 'size',
          value: {
            width: property.from.width + (property.to.width - property.from.width) * t,
            height: property.from.height + (property.to.height - property.from.height) * t
          }
        };
      case 'color':
        return {
          type: 'color',
          value: {
            r: Math.round(property.from.r + (property.to.r - property.from.r) * t),
            g: Math.round(property.from.g + (property.to.g - property.from.g) * t),
            b: Math.round(property.from.b + (property.to.b - property.from.b) * t)
          }
        };
      case 'scale':
        return {
          type: 'scale',
          value: property.from + (property.to - property.from) * t
        };
      case 'rotation':
        return {
          type: 'rotation',
          value: property.from + (property.to - property.from) * t
        };
      case 'custom':
        return {
          type: 'custom',
          name: property.name,
          value: property.from + (property.to - property.from) * t
        };
      case 'multiple':
        return {
          type: 'multiple',
          values: property.properties?.map(prop => PropertyInterpolator.interpolate(prop, t)) || []
        };
      default:
        return { type: 'opacity', value: 0 };
    }
  }
}

export class Animation {
  private id: AnimationId;
  private property: AnimatedProperty;
  private config: AnimationConfig;
  private state: AnimationRuntimeState;
  private callbacks: AnimationCallbacks;
  private startTime: number | null = null;
  private lastFrameTime: number | null = null;

  constructor(
    id: AnimationId,
    property: AnimatedProperty,
    config: Partial<AnimationConfig> = {},
    callbacks: AnimationCallbacks = {}
  ) {
    this.id = id;
    this.property = property;
    this.callbacks = callbacks;

    this.config = {
      duration: 500,
      easing: EasingFunction.EaseInOut,
      delay: 0,
      loop_mode: LoopMode.None,
      reverse: false,
      speed: 1.0,
      auto_play: false,
      auto_reverse: false,
      ...config
    };

    this.state = {
      state: AnimationState.Stopped,
      current_time: 0,
      loops_completed: 0,
      is_reversed: false,
      current_values: null,
      progress: 0
    };

    if (this.config.auto_play) {
      this.play();
    }
  }

  // Playback controls
  play(): void {
    this.state.state = AnimationState.Playing;
    this.lastFrameTime = performance.now();
    this.startTime = performance.now();
    
    this.callbacks.onStart?.(this);
  }

  pause(): void {
    this.state.state = AnimationState.Paused;
    this.callbacks.onPause?.(this);
  }

  stop(): void {
    this.state.state = AnimationState.Stopped;
    this.state.current_time = 0;
    this.state.progress = 0;
    this.state.loops_completed = 0;
    this.state.is_reversed = false;
    this.state.current_values = null;
    this.lastFrameTime = null;
    this.startTime = null;
    
    this.callbacks.onStop?.(this);
  }

  reverse(): void {
    this.state.is_reversed = !this.state.is_reversed;
    if (this.state.state === AnimationState.Playing) {
      this.state.state = AnimationState.Reversed;
    }
  }

  // Animation update (call in main loop)
  update(deltaTime: number): boolean {
    if (this.state.state !== AnimationState.Playing) {
      return false;
    }

    // Handle delay
    if (this.startTime && (performance.now() - this.startTime) < this.config.delay) {
      return false;
    }

    // Update time
    const adjustedDelta = deltaTime * this.config.speed;
    this.state.current_time += adjustedDelta;

    // Calculate progress
    const totalDuration = this.config.duration;
    const rawProgress = totalDuration > 0 ? Math.min(this.state.current_time / totalDuration, 1.0) : 1.0;

    // Apply direction
    const progress = (this.state.is_reversed || this.config.reverse) ? 1.0 - rawProgress : rawProgress;

    // Apply easing
    const easedProgress = EasingHelper.apply(this.config.easing, progress);
    
    // Update state
    this.state.progress = easedProgress;
    this.state.current_values = PropertyInterpolator.interpolate(this.property, easedProgress);

    // Trigger update callback
    if (this.state.current_values) {
      this.callbacks.onUpdate?.(this, this.state.current_values);
    }

    // Check for completion
    if (rawProgress >= 1.0) {
      this.handleAnimationComplete();
    }

    return true;
  }

  private handleAnimationComplete(): void {
    switch (this.config.loop_mode) {
      case LoopMode.None:
        this.state.state = AnimationState.Completed;
        this.callbacks.onComplete?.(this);
        break;
      case LoopMode.Infinite:
        this.restartAnimation();
        break;
      case LoopMode.Count:
        this.state.loops_completed++;
        if (this.config.loop_count && this.state.loops_completed < this.config.loop_count) {
          this.restartAnimation();
        } else {
          this.state.state = AnimationState.Completed;
          this.callbacks.onComplete?.(this);
        }
        break;
      case LoopMode.PingPong:
        this.state.is_reversed = !this.state.is_reversed;
        this.state.current_time = 0;
        this.state.loops_completed++;
        this.callbacks.onLoop?.(this, this.state.loops_completed);
        break;
    }
  }

  private restartAnimation(): void {
    this.state.current_time = 0;
    this.state.progress = 0;
    this.state.loops_completed++;
    
    if (this.config.auto_reverse) {
      this.state.is_reversed = !this.state.is_reversed;
    }
    
    this.callbacks.onLoop?.(this, this.state.loops_completed);
  }

  // Getters
  getId(): AnimationId {
    return this.id;
  }

  getState(): AnimationState {
    return this.state.state;
  }

  getProgress(): number {
    return this.state.progress;
  }

  getCurrentValues(): AnimatedValue | null {
    return this.state.current_values;
  }

  isPlaying(): boolean {
    return this.state.state === AnimationState.Playing || this.state.state === AnimationState.Reversed;
  }

  isCompleted(): boolean {
    return this.state.state === AnimationState.Completed;
  }

  setSpeed(speed: number): void {
    this.config.speed = Math.max(0, speed);
  }

  seek(progress: number): void {
    progress = Math.max(0, Math.min(1, progress));
    const targetTime = this.config.duration * progress;

    this.state.current_time = targetTime;
    this.state.progress = progress;
    this.state.current_values = PropertyInterpolator.interpolate(this.property, progress);
  }
}

export class AnimationBuilder {
  private id: AnimationId;
  private property: AnimatedProperty | null = null;
  private config: Partial<AnimationConfig> = {};
  private callbacks: AnimationCallbacks = {};

  constructor(id: AnimationId) {
    this.id = id;
  }

  animateProperty(property: AnimatedProperty): this {
    this.property = property;
    return this;
  }

  duration(duration: number): this {
    this.config.duration = duration;
    return this;
  }

  easing(easing: EasingConfig): this {
    this.config.easing = easing;
    return this;
  }

  delay(delay: number): this {
    this.config.delay = delay;
    return this;
  }

  loopMode(mode: LoopMode, count?: number): this {
    this.config.loop_mode = mode;
    if (count !== undefined) {
      this.config.loop_count = count;
    }
    return this;
  }

  speed(speed: number): this {
    this.config.speed = speed;
    return this;
  }

  autoPlay(autoPlay: boolean): this {
    this.config.auto_play = autoPlay;
    return this;
  }

  autoReverse(autoReverse: boolean): this {
    this.config.auto_reverse = autoReverse;
    return this;
  }

  onStart(callback: (animation: Animation) => void): this {
    this.callbacks.onStart = callback;
    return this;
  }

  onUpdate(callback: (animation: Animation, values: AnimatedValue) => void): this {
    this.callbacks.onUpdate = callback;
    return this;
  }

  onComplete(callback: (animation: Animation) => void): this {
    this.callbacks.onComplete = callback;
    return this;
  }

  onLoop(callback: (animation: Animation, loopCount: number) => void): this {
    this.callbacks.onLoop = callback;
    return this;
  }

  build(): Animation {
    const property = this.property || { type: 'opacity', from: 0, to: 1 };
    return new Animation(this.id, property, this.config, this.callbacks);
  }
}

export class AnimationTimeline {
  private id: TimelineId;
  private animations: Animation[] = [];
  private state: AnimationState = AnimationState.Stopped;
  private sequential: boolean;
  private currentIndex: number = 0;

  constructor(id: TimelineId, sequential: boolean = false) {
    this.id = id;
    this.sequential = sequential;
  }

  addAnimation(animation: Animation): void {
    this.animations.push(animation);
  }

  play(): void {
    this.state = AnimationState.Playing;
    
    if (this.sequential) {
      if (this.animations.length > 0) {
        this.currentIndex = 0;
        this.animations[0].play();
      }
    } else {
      this.animations.forEach(animation => animation.play());
    }
  }

  update(deltaTime: number): boolean {
    if (this.state !== AnimationState.Playing) {
      return false;
    }

    if (this.sequential) {
      if (this.currentIndex < this.animations.length) {
        const currentAnimation = this.animations[this.currentIndex];
        
        if (!currentAnimation.update(deltaTime) && currentAnimation.isCompleted()) {
          this.currentIndex++;
          
          if (this.currentIndex < this.animations.length) {
            this.animations[this.currentIndex].play();
          } else {
            this.state = AnimationState.Completed;
            return false;
          }
        }
      }
    } else {
      let anyPlaying = false;
      for (const animation of this.animations) {
        if (animation.update(deltaTime)) {
          anyPlaying = true;
        }
      }
      
      if (!anyPlaying) {
        this.state = AnimationState.Completed;
        return false;
      }
    }

    return true;
  }

  stop(): void {
    this.state = AnimationState.Stopped;
    this.currentIndex = 0;
    this.animations.forEach(animation => animation.stop());
  }

  getId(): TimelineId {
    return this.id;
  }

  getState(): AnimationState {
    return this.state;
  }
}

export class AnimationManager {
  private animations: Map<AnimationId, Animation> = new Map();
  private timelines: Map<TimelineId, AnimationTimeline> = new Map();
  private lastUpdate: number = performance.now();

  addAnimation(animation: Animation): void {
    this.animations.set(animation.getId(), animation);
  }

  addTimeline(timeline: AnimationTimeline): void {
    this.timelines.set(timeline.getId(), timeline);
  }

  update(): void {
    const now = performance.now();
    const deltaTime = now - this.lastUpdate;
    this.lastUpdate = now;

    // Update standalone animations
    for (const [id, animation] of this.animations) {
      animation.update(deltaTime);
      if (animation.isCompleted() && animation.getState() === AnimationState.Completed) {
        this.animations.delete(id);
      }
    }

    // Update timelines
    for (const timeline of this.timelines.values()) {
      timeline.update(deltaTime);
    }
  }

  getAnimation(id: AnimationId): Animation | undefined {
    return this.animations.get(id);
  }

  removeAnimation(id: AnimationId): boolean {
    return this.animations.delete(id);
  }

  cleanupCompleted(): void {
    for (const [id, animation] of this.animations) {
      if (animation.isCompleted()) {
        this.animations.delete(id);
      }
    }
    
    for (const [id, timeline] of this.timelines) {
      if (timeline.getState() === AnimationState.Completed) {
        this.timelines.delete(id);
      }
    }
  }

  getActiveCount(): number {
    return this.animations.size + this.timelines.size;
  }

  stopAll(): void {
    this.animations.forEach(animation => animation.stop());
    this.timelines.forEach(timeline => timeline.stop());
  }
}

// Convenience functions for common animations

export function fadeIn(id: AnimationId, duration: number = 500): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'opacity', from: 0, to: 1 })
    .duration(duration)
    .easing(EasingFunction.EaseOut)
    .build();
}

export function fadeOut(id: AnimationId, duration: number = 500): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'opacity', from: 1, to: 0 })
    .duration(duration)
    .easing(EasingFunction.EaseIn)
    .build();
}

export function slideInLeft(id: AnimationId, fromX: number, toX: number, y: number, duration: number = 500): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'position', from: { x: fromX, y }, to: { x: toX, y } })
    .duration(duration)
    .easing(EasingFunction.EaseOut)
    .build();
}

export function bounce(id: AnimationId, duration: number = 1000): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'scale', from: 1, to: 1.2 })
    .duration(duration)
    .easing(EasingFunction.Bounce)
    .loopMode(LoopMode.PingPong)
    .build();
}

export function pulse(id: AnimationId, duration: number = 1000): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'opacity', from: 1, to: 0.5 })
    .duration(duration)
    .easing(EasingFunction.EaseInOut)
    .loopMode(LoopMode.PingPong)
    .build();
}

export function colorTransition(id: AnimationId, fromColor: ColorDefinition, toColor: ColorDefinition, duration: number = 500): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'color', from: fromColor, to: toColor })
    .duration(duration)
    .easing(EasingFunction.EaseInOut)
    .build();
}

export function scaleUp(id: AnimationId, fromScale: number = 0, toScale: number = 1, duration: number = 300): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'scale', from: fromScale, to: toScale })
    .duration(duration)
    .easing(EasingFunction.Back)
    .build();
}

export function rotate(id: AnimationId, fromAngle: number = 0, toAngle: number = 360, duration: number = 1000): Animation {
  return new AnimationBuilder(id)
    .animateProperty({ type: 'rotation', from: fromAngle, to: toAngle })
    .duration(duration)
    .easing(EasingFunction.Linear)
    .build();
}