#!/usr/bin/env bun
/**
 * Slider Widget Demo - TypeScript
 * 
 * Demonstrates various slider configurations and features
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  slider, horizontalSlider, verticalSlider, rangeSlider,
  SliderOrientation, SliderMode
} from '../../packages/tui-bun/src/widgets/slider';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface AudioSettings {
  volume: number;
  bass: number;
  treble: number;
  balance: number;
}

interface DisplaySettings {
  brightness: number;
  contrast: number;
  saturation: number;
  temperature: number;
}

class SliderDemo implements Component {
  private audioSettings: AudioSettings = {
    volume: 75,
    bass: 50,
    treble: 50,
    balance: 50
  };
  
  private displaySettings: DisplaySettings = {
    brightness: 80,
    contrast: 50,
    saturation: 60,
    temperature: 6500
  };
  
  private rgbColor = { r: 128, g: 128, b: 128 };
  private opacity = 100;
  private zoom = 100;
  private playbackSpeed = 1.0;
  private selectedSlider = 0;
  private priceRange = { min: 100, max: 500 };

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        this.selectedSlider = (this.selectedSlider + 1) % 10;
        return true;
      
      case 'ArrowLeft':
        this.adjustCurrentSlider(-5);
        return true;
      
      case 'ArrowRight':
        this.adjustCurrentSlider(5);
        return true;
      
      case 'ArrowUp':
        this.adjustCurrentSlider(10);
        return true;
      
      case 'ArrowDown':
        this.adjustCurrentSlider(-10);
        return true;
      
      case 'r':
        // Reset all sliders
        this.audioSettings = { volume: 75, bass: 50, treble: 50, balance: 50 };
        this.displaySettings = { brightness: 80, contrast: 50, saturation: 60, temperature: 6500 };
        this.rgbColor = { r: 128, g: 128, b: 128 };
        this.opacity = 100;
        this.zoom = 100;
        this.playbackSpeed = 1.0;
        this.priceRange = { min: 100, max: 500 };
        return true;
      
      case 'm':
        // Mute/unmute
        this.audioSettings.volume = this.audioSettings.volume > 0 ? 0 : 75;
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  adjustCurrentSlider(delta: number) {
    switch (this.selectedSlider) {
      case 0:
        this.audioSettings.volume = Math.max(0, Math.min(100, this.audioSettings.volume + delta));
        break;
      case 1:
        this.audioSettings.bass = Math.max(0, Math.min(100, this.audioSettings.bass + delta));
        break;
      case 2:
        this.displaySettings.brightness = Math.max(0, Math.min(100, this.displaySettings.brightness + delta));
        break;
      case 3:
        this.zoom = Math.max(50, Math.min(200, this.zoom + delta * 2));
        break;
      case 4:
        this.playbackSpeed = Math.max(0.25, Math.min(4, this.playbackSpeed + delta * 0.05));
        break;
      case 5:
        this.rgbColor.r = Math.max(0, Math.min(255, this.rgbColor.r + delta * 2.55));
        break;
      case 6:
        this.rgbColor.g = Math.max(0, Math.min(255, this.rgbColor.g + delta * 2.55));
        break;
      case 7:
        this.rgbColor.b = Math.max(0, Math.min(255, this.rgbColor.b + delta * 2.55));
        break;
      case 8:
        this.opacity = Math.max(0, Math.min(100, this.opacity + delta));
        break;
      case 9:
        const range = this.priceRange.max - this.priceRange.min;
        this.priceRange.min = Math.max(0, Math.min(900, this.priceRange.min + delta));
        this.priceRange.max = Math.max(this.priceRange.min + 10, Math.min(1000, this.priceRange.max + delta));
        break;
    }
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('🎚️ Slider Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text('Interactive value controls with sliders', { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Audio Controls
            div({ class: 'mb-8' })
              .children([
                text('Audio Controls', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-4' })
                  .children([
                    div()
                      .children([
                        text(`🔊 Volume: ${this.audioSettings.volume}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'volume',
                          min: 0,
                          max: 100,
                          value: this.audioSettings.volume,
                          showValues: true,
                          classes: this.selectedSlider === 0 ? ['slider-focused'] : []
                        })
                      ]),
                    div()
                      .children([
                        text(`🎵 Bass: ${this.audioSettings.bass}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'bass',
                          min: 0,
                          max: 100,
                          value: this.audioSettings.bass,
                          showValues: true,
                          classes: this.selectedSlider === 1 ? ['slider-focused'] : []
                        })
                      ]),
                    div()
                      .children([
                        text(`🎶 Treble: ${this.audioSettings.treble}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'treble',
                          min: 0,
                          max: 100,
                          value: this.audioSettings.treble,
                          showValues: true
                        })
                      ]),
                    div()
                      .children([
                        text(`⚖️ Balance: ${this.audioSettings.balance === 50 ? 'Center' : this.audioSettings.balance < 50 ? `L${50-this.audioSettings.balance}` : `R${this.audioSettings.balance-50}`}`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'balance',
                          min: 0,
                          max: 100,
                          value: this.audioSettings.balance,
                          showValues: true
                        })
                      ])
                  ])
              ]),
            
            // Display Settings
            div({ class: 'mb-8' })
              .children([
                text('Display Settings', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-2 gap-4' })
                  .children([
                    div()
                      .children([
                        text(`☀️ Brightness: ${this.displaySettings.brightness}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'brightness',
                          min: 0,
                          max: 100,
                          value: this.displaySettings.brightness,
                          showValues: true,
                          classes: this.selectedSlider === 2 ? ['slider-focused'] : []
                        })
                      ]),
                    div()
                      .children([
                        text(`◐ Contrast: ${this.displaySettings.contrast}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'contrast',
                          min: 0,
                          max: 100,
                          value: this.displaySettings.contrast,
                          showValues: true
                        })
                      ]),
                    div()
                      .children([
                        text(`🎨 Saturation: ${this.displaySettings.saturation}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'saturation',
                          min: 0,
                          max: 100,
                          value: this.displaySettings.saturation,
                          showValues: true
                        })
                      ]),
                    div()
                      .children([
                        text(`🌡️ Temperature: ${this.displaySettings.temperature}K`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'temperature',
                          min: 3000,
                          max: 10000,
                          value: this.displaySettings.temperature,
                          step: 100,
                          showValues: true
                        })
                      ])
                  ])
              ]),
            
            // Color Picker
            div({ class: 'mb-8' })
              .children([
                text('RGB Color Picker', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    div()
                      .children([
                        text(`🔴 Red: ${this.rgbColor.r}`, { class: 'text-sm text-red-400 mb-1' }),
                        horizontalSlider({
                          id: 'red',
                          min: 0,
                          max: 255,
                          value: this.rgbColor.r,
                          showValues: true,
                          classes: ['slider-red', ...(this.selectedSlider === 5 ? ['slider-focused'] : [])]
                        })
                      ]),
                    div()
                      .children([
                        text(`🟢 Green: ${this.rgbColor.g}`, { class: 'text-sm text-green-400 mb-1' }),
                        horizontalSlider({
                          id: 'green',
                          min: 0,
                          max: 255,
                          value: this.rgbColor.g,
                          showValues: true,
                          classes: ['slider-green', ...(this.selectedSlider === 6 ? ['slider-focused'] : [])]
                        })
                      ]),
                    div()
                      .children([
                        text(`🔵 Blue: ${this.rgbColor.b}`, { class: 'text-sm text-blue-400 mb-1' }),
                        horizontalSlider({
                          id: 'blue',
                          min: 0,
                          max: 255,
                          value: this.rgbColor.b,
                          showValues: true,
                          classes: ['slider-blue', ...(this.selectedSlider === 7 ? ['slider-focused'] : [])]
                        })
                      ]),
                    div({ class: 'flex items-center gap-4' })
                      .children([
                        div({ 
                          class: 'w-20 h-20 rounded border-2 border-gray-600',
                          style: `background-color: rgb(${this.rgbColor.r}, ${this.rgbColor.g}, ${this.rgbColor.b})`
                        }),
                        text(`RGB(${this.rgbColor.r}, ${this.rgbColor.g}, ${this.rgbColor.b})`, 
                          { class: 'font-mono' })
                      ])
                  ])
              ]),
            
            // Range Slider
            div({ class: 'mb-8' })
              .children([
                text('Price Range Filter', { class: 'text-xl font-bold mb-4' }),
                div()
                  .children([
                    text(`Price Range: $${this.priceRange.min} - $${this.priceRange.max}`, { class: 'text-sm text-gray-400 mb-1' }),
                    rangeSlider({
                      id: 'price-range',
                      min: 0,
                      max: 1000,
                      value: this.priceRange.min,
                      rangeEnd: this.priceRange.max,
                      showValues: true,
                      classes: this.selectedSlider === 9 ? ['slider-focused'] : []
                    })
                  ])
              ]),
            
            // Special Controls
            div({ class: 'mb-8' })
              .children([
                text('Special Controls', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-2 gap-6' })
                  .children([
                    div()
                      .children([
                        text(`🔍 Zoom: ${this.zoom}%`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'zoom',
                          min: 50,
                          max: 200,
                          value: this.zoom,
                          step: 10,
                          showValues: true,
                          classes: this.selectedSlider === 3 ? ['slider-focused'] : []
                        })
                      ]),
                    div()
                      .children([
                        text(`▶️ Playback Speed: ${this.playbackSpeed}x`, { class: 'text-sm text-gray-400 mb-1' }),
                        horizontalSlider({
                          id: 'playback',
                          min: 0.25,
                          max: 4,
                          value: this.playbackSpeed,
                          step: 0.25,
                          showValues: true,
                          classes: this.selectedSlider === 4 ? ['slider-focused'] : []
                        })
                      ])
                  ])
              ]),
            
            // Opacity Control
            div({ class: 'mb-8' })
              .children([
                text('Opacity Control', { class: 'text-xl font-bold mb-4' }),
                div()
                  .children([
                    text(`Opacity: ${this.opacity}%`, { class: 'text-sm text-gray-400 mb-1' }),
                    horizontalSlider({
                      id: 'opacity',
                      min: 0,
                      max: 100,
                      value: this.opacity,
                      showValues: true,
                      classes: this.selectedSlider === 8 ? ['slider-focused'] : []
                    })
                  ]),
                div({ 
                  class: 'mt-4 p-4 rounded transition-opacity bg-blue-500',
                  style: `opacity: ${this.opacity / 100}`
                })
                  .child(text('This box opacity changes with the slider'))
              ]),
            
            // Vertical Sliders Demo
            div({ class: 'mb-8' })
              .children([
                text('Vertical Sliders', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-8' })
                  .children([
                    div({ class: 'flex flex-col items-center' })
                      .children([
                        verticalSlider({
                          id: 'vert-1',
                          min: 0,
                          max: 100,
                          value: 75,
                          showValues: true
                        }),
                        text('Volume', { class: 'text-sm text-gray-400 mt-2' })
                      ]),
                    div({ class: 'flex flex-col items-center' })
                      .children([
                        verticalSlider({
                          id: 'vert-2',
                          min: 0,
                          max: 100,
                          value: 50,
                          showValues: true
                        }),
                        text('Balance', { class: 'text-sm text-gray-400 mt-2' })
                      ]),
                    div({ class: 'flex flex-col items-center' })
                      .children([
                        verticalSlider({
                          id: 'vert-3',
                          min: 0,
                          max: 100,
                          value: 25,
                          showValues: true
                        }),
                        text('Gain', { class: 'text-sm text-gray-400 mt-2' })
                      ])
                  ])
              ]),
            
            // Disabled Slider
            div({ class: 'mb-8' })
              .children([
                text('Disabled State', { class: 'text-xl font-bold mb-4' }),
                slider({
                  id: 'disabled',
                  mode: SliderMode.Single,
                  orientation: SliderOrientation.Horizontal,
                  min: 0,
                  max: 100,
                  value: 30,
                  showValues: true,
                  classes: ['slider-disabled']
                })
              ])
          ]),
        
        // Footer with status
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            div({ class: 'grid grid-cols-2 gap-4 text-sm text-gray-400 mb-2' })
              .children([
                text(`Volume: ${this.audioSettings.volume}% | Bass: ${this.audioSettings.bass}%`),
                text(`Brightness: ${this.displaySettings.brightness}% | Zoom: ${this.zoom}%`)
              ]),
            text('[Tab] Navigate | [←→] Adjust ±5 | [↑↓] Adjust ±10 | [M] Mute | [R] Reset | [Q] Quit', 
              { class: 'text-center text-sm text-gray-500' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new SliderDemo().render(),
    // Uses full terminal by default
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}