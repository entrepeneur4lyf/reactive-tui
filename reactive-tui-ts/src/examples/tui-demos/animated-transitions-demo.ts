#!/usr/bin/env bun
/**
 * Animated Screen Transitions Demo
 * 
 * Shows smooth transitions between screens
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

type TransitionType = 'none' | 'fade' | 'slide-left' | 'slide-right' | 'slide-up' | 'slide-down';

interface Screen {
  id: string;
  title: string;
  color: string;
  content: string[];
}

class AnimatedScreenManager implements Component {
  private screens: Screen[] = [
    {
      id: 'welcome',
      title: '🎉 Welcome Screen',
      color: 'bg-blue-800',
      content: [
        'Welcome to the Animated Transitions Demo!',
        '',
        'This demo showcases smooth screen transitions:',
        '• Fade transitions',
        '• Slide animations (all directions)',
        '• Progress indicators',
        '• Smooth navigation'
      ]
    },
    {
      id: 'features',
      title: '✨ Features',
      color: 'bg-green-800',
      content: [
        'Key Features:',
        '',
        '→ Multiple transition types',
        '→ Customizable duration',
        '→ Hardware accelerated',
        '→ Smooth 60 FPS animations',
        '→ Navigation history',
        '→ Keyboard shortcuts'
      ]
    },
    {
      id: 'controls',
      title: '🎮 Controls',
      color: 'bg-purple-800',
      content: [
        'Navigation Controls:',
        '',
        '[→] Next screen (slide left)',
        '[←] Previous screen (slide right)',
        '[↑] Jump up (slide up)',
        '[↓] Jump down (slide down)',
        '[F] Fade transition',
        '[Space] Random transition',
        '[1-5] Direct navigation'
      ]
    },
    {
      id: 'performance',
      title: '⚡ Performance',
      color: 'bg-orange-800',
      content: [
        'Performance Metrics:',
        '',
        'Frame Rate: 60 FPS',
        'Render Time: <16ms',
        'Memory Usage: 42MB',
        'CPU Usage: 12%',
        '',
        'Optimized for smooth transitions!'
      ]
    },
    {
      id: 'about',
      title: 'ℹ️ About',
      color: 'bg-red-800',
      content: [
        'About This Demo:',
        '',
        'Built with TUI Framework',
        'TypeScript + Bun',
        'Hardware Accelerated',
        'Cross-Platform',
        '',
        'Press [Q] to quit'
      ]
    }
  ];

  private currentIndex = 0;
  private nextIndex = 0;
  private transition: TransitionType = 'none';
  private transitionProgress = 0;
  private transitionDuration = 300; // ms
  private transitionStartTime = 0;
  private history: number[] = [];

  private animationFrame?: NodeJS.Timeout;

  constructor() {
    this.startAnimation();
  }

  private startAnimation() {
    const animate = () => {
      if (this.transition !== 'none') {
        const elapsed = Date.now() - this.transitionStartTime;
        this.transitionProgress = Math.min(elapsed / this.transitionDuration, 1);
        
        if (this.transitionProgress >= 1) {
          this.currentIndex = this.nextIndex;
          this.transition = 'none';
          this.transitionProgress = 0;
        }
      }
      
      this.animationFrame = setTimeout(animate, 16); // ~60 FPS
    };
    animate();
  }

  private startTransition(toIndex: number, type: TransitionType) {
    if (this.transition !== 'none' || toIndex === this.currentIndex) return;
    
    this.history.push(this.currentIndex);
    this.nextIndex = toIndex;
    this.transition = type;
    this.transitionProgress = 0;
    this.transitionStartTime = Date.now();
  }

  handleKeyPress(key: KeyEvent): boolean {
    // Direct navigation
    if (key.data.key >= '1' && key.data.key <= '5') {
      const index = parseInt(key.data.key) - 1;
      if (index < this.screens.length) {
        this.startTransition(index, 'fade');
        return true;
      }
    }

    switch (key.data.key) {
      case 'ArrowRight':
        this.startTransition(
          (this.currentIndex + 1) % this.screens.length,
          'slide-left'
        );
        return true;
        
      case 'ArrowLeft':
        this.startTransition(
          (this.currentIndex - 1 + this.screens.length) % this.screens.length,
          'slide-right'
        );
        return true;
        
      case 'ArrowUp':
        this.startTransition(
          (this.currentIndex - 1 + this.screens.length) % this.screens.length,
          'slide-down'
        );
        return true;
        
      case 'ArrowDown':
        this.startTransition(
          (this.currentIndex + 1) % this.screens.length,
          'slide-up'
        );
        return true;
        
      case 'f':
      case 'F':
        this.startTransition(
          (this.currentIndex + 1) % this.screens.length,
          'fade'
        );
        return true;
        
      case ' ':
        const transitions: TransitionType[] = ['fade', 'slide-left', 'slide-right', 'slide-up', 'slide-down'];
        const randomTransition = transitions[Math.floor(Math.random() * transitions.length)];
        this.startTransition(
          (this.currentIndex + 1) % this.screens.length,
          randomTransition
        );
        return true;
        
      case 'Escape':
        const prev = this.history.pop();
        if (prev !== undefined) {
          this.startTransition(prev, 'fade');
        }
        return true;
        
      case 'q':
      case 'Q':
        if (this.animationFrame) clearTimeout(this.animationFrame);
        process.exit(0);
        
      default:
        return false;
    }
  }

  private easeInOut(t: number): number {
    return t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
  }

  private renderScreen(screen: Screen, opacity: number = 1, transform: string = '') {
    const opacityClass = opacity < 1 ? `opacity-${Math.floor(opacity * 100)}` : '';
    
    return div({ class: `absolute inset-0 ${screen.color} ${transform} ${opacityClass}` })
      .children([
        // Header
        div({ class: 'text-white text-2xl font-bold p-4 border-b border-black/20' })
          .child(text(screen.title)),
        
        // Content
        div({ class: 'p-8' })
          .children(screen.content.map(line => 
            div({ class: 'text-white text-lg mb-2' })
              .child(text(line))
          ))
      ]);
  }

  render(): Element {
    const currentScreen = this.screens[this.currentIndex];
    const nextScreen = this.screens[this.nextIndex];
    const progress = this.easeInOut(this.transitionProgress);

    return div({ class: 'relative h-full bg-gray-900 overflow-hidden' })
      .children([
        // Screens container
        div({ class: 'relative h-full' })
          .children(this.transition === 'none' ? 
            [this.renderScreen(currentScreen).build()] :
            (() => {
              switch (this.transition) {
                case 'fade':
                  return [
                    this.renderScreen(currentScreen, 1 - progress).build(),
                    this.renderScreen(nextScreen, progress).build()
                  ];
                  
                case 'slide-left':
                  return [
                    this.renderScreen(currentScreen, 1, `translate-x-${Math.floor(-100 * progress)}`).build(),
                    this.renderScreen(nextScreen, 1, `translate-x-${Math.floor(100 * (1 - progress))}`).build()
                  ];
                  
                case 'slide-right':
                  return [
                    this.renderScreen(currentScreen, 1, `translate-x-${Math.floor(100 * progress)}`).build(),
                    this.renderScreen(nextScreen, 1, `translate-x-${Math.floor(-100 * (1 - progress))}`).build()
                  ];
                  
                case 'slide-up':
                  return [
                    this.renderScreen(currentScreen, 1, `translate-y-${Math.floor(-100 * progress)}`).build(),
                    this.renderScreen(nextScreen, 1, `translate-y-${Math.floor(100 * (1 - progress))}`).build()
                  ];
                  
                case 'slide-down':
                  return [
                    this.renderScreen(currentScreen, 1, `translate-y-${Math.floor(100 * progress)}`).build(),
                    this.renderScreen(nextScreen, 1, `translate-y-${Math.floor(-100 * (1 - progress))}`).build()
                  ];
                  
                default:
                  return [this.renderScreen(currentScreen).build()];
              }
            })()
          ),
        
        // Navigation dots
        div({ class: 'absolute bottom-4 left-0 right-0 flex justify-center gap-2' })
          .children(this.screens.map((_, i) => 
            div({ 
              class: `w-3 h-3 rounded-full transition-all ${
                i === (this.transition === 'none' ? this.currentIndex : this.nextIndex) ?
                'bg-white scale-125' : 'bg-white/30'
              }` 
            })
          )),
        
        // Transition indicator
        ...(this.transition !== 'none' ? [
          div({ class: 'absolute top-4 right-4 bg-black/50 text-white px-3 py-1 rounded' })
            .child(text(`${this.transition} ${Math.floor(this.transitionProgress * 100)}%`))
        ] : []),
        
        // Controls hint
        div({ class: 'absolute bottom-12 left-0 right-0 text-center text-white/50' })
          .child(text('[←→↑↓] Navigate | [F] Fade | [Space] Random | [1-5] Jump | [Q] Quit'))
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new AnimatedScreenManager().render()
  });

  await app.run();
}

main().catch(console.error);