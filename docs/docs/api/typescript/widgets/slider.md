# Slider Widget

Range sliders with single and dual-handle modes, supporting horizontal and vertical orientations with customizable steps and value display.

## Overview

The Slider widget provides interactive range selection with support for single-value and dual-handle range sliders, multiple orientations, stepped values, and comprehensive accessibility features.

```typescript
import { slider, SliderMode, SliderOrientation } from 'reactive-tui-ts'

const volumeSlider = slider({
  id: 'volume-control',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 100,
  value: 75,
  step: 5,
  label: 'Volume',
  showValues: true
})
```

## Types

### SliderMode

```typescript
export enum SliderMode {
  Single = 'single',
  Range = 'range'
}
```

### SliderOrientation

```typescript
export enum SliderOrientation {
  Horizontal = 'horizontal',
  Vertical = 'vertical'
}
```

### SliderConfig

```typescript
interface SliderConfig {
  id?: string
  mode: SliderMode
  orientation: SliderOrientation
  min: number
  max: number
  value: number
  rangeEnd?: number
  step?: number
  label?: string
  description?: string
  showValues?: boolean
  showPercentage?: boolean
  classes?: string[]
}
```

## Basic Usage

### Single-Value Slider

```typescript
import { slider, SliderMode, SliderOrientation } from 'reactive-tui-ts'

// Basic horizontal slider
const basicSlider = slider({
  id: 'basic-slider',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 100,
  value: 50,
  label: 'Basic Slider',
  showValues: true
})

// Stepped slider with specific increments
const steppedSlider = slider({
  id: 'stepped-slider',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 10,
  value: 5,
  step: 0.5,
  label: 'Rating',
  showValues: true,
  showPercentage: false
})
```

### Range Slider (Dual Handle)

```typescript
const priceRangeSlider = slider({
  id: 'price-range',
  mode: SliderMode.Range,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 1000,
  value: 100,      // Start of range
  rangeEnd: 500,   // End of range
  step: 10,
  label: 'Price Range',
  showValues: true,
  description: 'Select your budget range'
})
```

### Vertical Slider

```typescript
const verticalSlider = slider({
  id: 'vertical-control',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Vertical,
  min: -50,
  max: 50,
  value: 0,
  step: 1,
  label: 'Temperature',
  showValues: true,
  classes: ['temperature-control']
})
```

## Convenience Functions

### Horizontal Sliders

```typescript
import { horizontalSlider } from 'reactive-tui-ts'

const brightnessSlider = horizontalSlider({
  id: 'brightness',
  min: 0,
  max: 100,
  value: 80,
  step: 5,
  label: 'Screen Brightness',
  showValues: true
})

const opacitySlider = horizontalSlider({
  id: 'opacity',
  min: 0,
  max: 1,
  value: 0.8,
  step: 0.1,
  label: 'Opacity',
  classes: ['opacity-control']
})
```

### Vertical Sliders

```typescript
import { verticalSlider } from 'reactive-tui-ts'

const audioSlider = verticalSlider({
  id: 'audio-level',
  min: 0,
  max: 100,
  value: 60,
  step: 2,
  label: 'Audio Level',
  showValues: true,
  classes: ['audio-mixer']
})
```

### Range Sliders

```typescript
import { rangeSlider } from 'reactive-tui-ts'

const timeRangeSlider = rangeSlider({
  id: 'time-range',
  min: 0,
  max: 24,
  value: 9,        // Start time: 9 AM
  rangeEnd: 17,    // End time: 5 PM
  step: 0.5,       // 30-minute increments
  label: 'Working Hours',
  showValues: true
})

const ageRangeSlider = rangeSlider({
  id: 'age-range',
  min: 18,
  max: 65,
  value: 25,
  rangeEnd: 45,
  step: 1,
  label: 'Target Age Range',
  orientation: SliderOrientation.Horizontal
})
```

## Builder Pattern

### SliderBuilder

```typescript
import { SliderBuilder, SliderMode, SliderOrientation } from 'reactive-tui-ts'

const complexSlider = SliderBuilder.create()
  .id('complex-slider')
  .horizontal()
  .range(0, 1000)
  .value(250)
  .step(25)
  .label('Budget Allocation')
  .description('Adjust your budget allocation')
  .showValues(true)
  .showPercentage(true)
  .classes(['budget-control', 'financial'])
  .build()
```

### Builder Methods

```typescript
// Create builder instance
const builder = SliderBuilder.create()

// Configuration methods
builder.id('my-slider')                          // Set slider ID
builder.mode(SliderMode.Single)                  // Set mode
builder.orientation(SliderOrientation.Horizontal) // Set orientation
builder.range(0, 100)                           // Set min/max range
builder.value(50)                               // Set current value
builder.rangeEnd(75)                            // Set range end (auto-sets Range mode)
builder.step(5)                                 // Set step increment
builder.label('My Slider')                      // Set label
builder.description('Slider description')       // Set description
builder.showValues(true)                        // Show numeric values
builder.showPercentage(false)                   // Show percentage
builder.classes(['custom', 'themed'])           // Add CSS classes

// Convenience methods
builder.horizontal()                            // Set horizontal orientation
builder.vertical()                              // Set vertical orientation
builder.single()                                // Set single-value mode
builder.dualRange(20, 80)                       // Set dual-handle range

// Build the slider
const slider = builder.build()
```

## Convenience Builder Functions

### Create Specific Slider Types

```typescript
import { 
  createSlider, 
  createHorizontalSlider, 
  createVerticalSlider, 
  createRangeSlider 
} from 'reactive-tui-ts'

// Generic slider builder
const genericSlider = createSlider()
  .id('generic')
  .range(0, 100)
  .value(50)
  .build()

// Pre-configured horizontal slider
const horizontalBuilder = createHorizontalSlider(0, 100, 75)
  .id('horizontal-preset')
  .step(5)
  .label('Horizontal Control')
  .build()

// Pre-configured vertical slider
const verticalBuilder = createVerticalSlider(-10, 10, 0)
  .id('vertical-preset')
  .step(0.5)
  .label('Vertical Control')
  .build()

// Pre-configured range slider
const rangeBuilder = createRangeSlider(0, 1000, 200, 800)
  .id('range-preset')
  .step(50)
  .label('Range Control')
  .build()
```

## Real-World Examples

### Audio Mixing Console

```typescript
import { SliderBuilder, SliderOrientation } from 'reactive-tui-ts'

class AudioMixingConsole {
  private channels: Map<string, any> = new Map()

  constructor() {
    this.setupChannels()
  }

  private setupChannels() {
    // Master volume (horizontal)
    this.channels.set('master', SliderBuilder.create()
      .id('master-volume')
      .horizontal()
      .range(0, 100)
      .value(75)
      .step(1)
      .label('Master Volume')
      .showValues(true)
      .classes(['master-control', 'volume-slider'])
      .build()
    )

    // Individual channel controls (vertical)
    const channelNames = ['Vocals', 'Guitar', 'Bass', 'Drums', 'Keys']
    channelNames.forEach((name, index) => {
      this.channels.set(`channel-${index}`, SliderBuilder.create()
        .id(`channel-${index}-volume`)
        .vertical()
        .range(0, 100)
        .value(60)
        .step(2)
        .label(name)
        .showValues(true)
        .classes(['channel-control', 'volume-slider'])
        .build()
      )
    })

    // EQ controls (horizontal)
    const eqBands = ['Low', 'Mid', 'High']
    eqBands.forEach((band, index) => {
      this.channels.set(`eq-${band.toLowerCase()}`, SliderBuilder.create()
        .id(`eq-${band.toLowerCase()}`)
        .horizontal()
        .range(-12, 12)
        .value(0)
        .step(0.5)
        .label(`${band} EQ`)
        .showValues(true)
        .classes(['eq-control', 'eq-slider'])
        .build()
      )
    })

    // Pan controls (horizontal, centered)
    channelNames.forEach((name, index) => {
      this.channels.set(`pan-${index}`, SliderBuilder.create()
        .id(`pan-${index}`)
        .horizontal()
        .range(-100, 100)
        .value(0)
        .step(5)
        .label(`${name} Pan`)
        .showValues(true)
        .classes(['pan-control', 'pan-slider'])
        .build()
      )
    })
  }

  getMasterVolume(): number {
    const masterSlider = this.channels.get('master')
    return parseInt(masterSlider?.getAttribute('value') || '0')
  }

  setChannelVolume(channelIndex: number, volume: number) {
    const channel = this.channels.get(`channel-${channelIndex}`)
    if (channel) {
      channel.setAttribute('value', volume.toString())
      console.log(`Channel ${channelIndex} volume set to ${volume}`)
    }
  }

  getChannelLevels(): Record<string, number> {
    const levels: Record<string, number> = {}
    
    for (const [key, slider] of this.channels) {
      if (key.startsWith('channel-')) {
        const value = parseInt(slider.getAttribute('value') || '0')
        levels[key] = value
      }
    }
    
    return levels
  }

  render() {
    return `
      <div class="audio-console">
        <div class="master-section">
          <h3>Master</h3>
          ${this.channels.get('master').build().toString()}
        </div>
        
        <div class="channel-section">
          <h3>Channels</h3>
          <div class="channel-strips">
            ${Array.from(this.channels.entries())
              .filter(([key]) => key.startsWith('channel-'))
              .map(([key, slider]) => `
                <div class="channel-strip">
                  ${slider.build().toString()}
                </div>
              `).join('')}
          </div>
        </div>
        
        <div class="eq-section">
          <h3>Master EQ</h3>
          <div class="eq-controls">
            ${['low', 'mid', 'high'].map(band => `
              <div class="eq-band">
                ${this.channels.get(`eq-${band}`).build().toString()}
              </div>
            `).join('')}
          </div>
        </div>
        
        <div class="pan-section">
          <h3>Pan Controls</h3>
          <div class="pan-controls">
            ${Array.from(this.channels.entries())
              .filter(([key]) => key.startsWith('pan-'))
              .map(([key, slider]) => `
                <div class="pan-control">
                  ${slider.build().toString()}
                </div>
              `).join('')}
          </div>
        </div>
      </div>
    `
  }
}

const audioConsole = new AudioMixingConsole()
console.log(audioConsole.render())
```

### Settings Dashboard

```typescript
import { SliderBuilder, SliderMode, SliderOrientation, rangeSlider } from 'reactive-tui-ts'

class SettingsDashboard {
  private settings: Map<string, any> = new Map()

  constructor() {
    this.setupSettings()
  }

  private setupSettings() {
    // Display settings
    this.settings.set('brightness', SliderBuilder.create()
      .id('display-brightness')
      .horizontal()
      .range(10, 100)
      .value(80)
      .step(5)
      .label('Screen Brightness')
      .description('Adjust screen brightness level')
      .showPercentage(true)
      .classes(['display-setting'])
      .build()
    )

    this.settings.set('contrast', SliderBuilder.create()
      .id('display-contrast')
      .horizontal()
      .range(50, 150)
      .value(100)
      .step(5)
      .label('Contrast')
      .showPercentage(true)
      .classes(['display-setting'])
      .build()
    )

    // Audio settings
    this.settings.set('volume', SliderBuilder.create()
      .id('system-volume')
      .horizontal()
      .range(0, 100)
      .value(70)
      .step(2)
      .label('System Volume')
      .showValues(true)
      .classes(['audio-setting'])
      .build()
    )

    this.settings.set('bass', SliderBuilder.create()
      .id('audio-bass')
      .horizontal()
      .range(-10, 10)
      .value(0)
      .step(1)
      .label('Bass')
      .showValues(true)
      .classes(['audio-setting', 'eq-control'])
      .build()
    )

    this.settings.set('treble', SliderBuilder.create()
      .id('audio-treble')
      .horizontal()
      .range(-10, 10)
      .value(0)
      .step(1)
      .label('Treble')
      .showValues(true)
      .classes(['audio-setting', 'eq-control'])
      .build()
    )

    // Performance settings
    this.settings.set('cpu-limit', SliderBuilder.create()
      .id('cpu-usage-limit')
      .horizontal()
      .range(10, 100)
      .value(80)
      .step(5)
      .label('CPU Usage Limit')
      .showPercentage(true)
      .classes(['performance-setting'])
      .build()
    )

    this.settings.set('memory-limit', SliderBuilder.create()
      .id('memory-usage-limit')
      .horizontal()
      .range(1, 16)
      .value(8)
      .step(0.5)
      .label('Memory Limit (GB)')
      .showValues(true)
      .classes(['performance-setting'])
      .build()
    )

    // Time range settings
    this.settings.set('active-hours', rangeSlider({
      id: 'active-hours-range',
      min: 0,
      max: 24,
      value: 8,       // 8 AM start
      rangeEnd: 18,   // 6 PM end
      step: 0.5,
      label: 'Active Hours',
      showValues: true,
      classes: ['time-setting']
    }))

    this.settings.set('notification-quiet', rangeSlider({
      id: 'quiet-hours-range',
      min: 0,
      max: 24,
      value: 22,      // 10 PM start
      rangeEnd: 7,    // 7 AM end
      step: 0.5,
      label: 'Quiet Hours (No Notifications)',
      showValues: true,
      classes: ['time-setting']
    }))

    // Temperature control (vertical slider)
    this.settings.set('target-temp', SliderBuilder.create()
      .id('target-temperature')
      .vertical()
      .range(60, 80)
      .value(72)
      .step(0.5)
      .label('Target Temperature (°F)')
      .showValues(true)
      .classes(['climate-setting'])
      .build()
    )
  }

  getSettingValue(settingKey: string): number | { start: number; end: number } | null {
    const setting = this.settings.get(settingKey)
    if (!setting) return null

    const value = parseFloat(setting.getAttribute('value') || '0')
    const rangeEnd = setting.getAttribute('range-end')
    
    if (rangeEnd) {
      return {
        start: value,
        end: parseFloat(rangeEnd)
      }
    }
    
    return value
  }

  updateSetting(settingKey: string, value: number | { start: number; end: number }) {
    const setting = this.settings.get(settingKey)
    if (!setting) return

    if (typeof value === 'object') {
      // Range slider
      setting.setAttribute('value', value.start.toString())
      setting.setAttribute('range-end', value.end.toString())
    } else {
      // Single value slider
      setting.setAttribute('value', value.toString())
    }

    console.log(`Updated ${settingKey}:`, value)
    this.applySettingChange(settingKey, value)
  }

  private applySettingChange(settingKey: string, value: number | { start: number; end: number }) {
    switch (settingKey) {
      case 'brightness':
        console.log(`Screen brightness set to ${value}%`)
        // Apply brightness change
        break
      
      case 'volume':
        console.log(`System volume set to ${value}`)
        // Apply volume change
        break
      
      case 'active-hours':
        if (typeof value === 'object') {
          console.log(`Active hours: ${value.start}:00 - ${value.end}:00`)
        }
        break
      
      case 'target-temp':
        console.log(`Target temperature set to ${value}°F`)
        // Apply temperature change
        break
      
      default:
        console.log(`Setting ${settingKey} updated to:`, value)
    }
  }

  getAllSettings(): Record<string, any> {
    const allSettings: Record<string, any> = {}
    
    for (const [key] of this.settings) {
      allSettings[key] = this.getSettingValue(key)
    }
    
    return allSettings
  }

  resetToDefaults() {
    console.log('Resetting all settings to defaults...')
    
    this.updateSetting('brightness', 80)
    this.updateSetting('contrast', 100)
    this.updateSetting('volume', 70)
    this.updateSetting('bass', 0)
    this.updateSetting('treble', 0)
    this.updateSetting('cpu-limit', 80)
    this.updateSetting('memory-limit', 8)
    this.updateSetting('active-hours', { start: 8, end: 18 })
    this.updateSetting('notification-quiet', { start: 22, end: 7 })
    this.updateSetting('target-temp', 72)
  }

  render() {
    return `
      <div class="settings-dashboard">
        <h1>System Settings</h1>
        
        <div class="settings-section display-settings">
          <h2>Display</h2>
          <div class="setting-group">
            ${this.settings.get('brightness').build().toString()}
          </div>
          <div class="setting-group">
            ${this.settings.get('contrast').build().toString()}
          </div>
        </div>
        
        <div class="settings-section audio-settings">
          <h2>Audio</h2>
          <div class="setting-group">
            ${this.settings.get('volume').build().toString()}
          </div>
          <div class="eq-group">
            <h3>Equalizer</h3>
            <div class="eq-controls">
              ${this.settings.get('bass').build().toString()}
              ${this.settings.get('treble').build().toString()}
            </div>
          </div>
        </div>
        
        <div class="settings-section performance-settings">
          <h2>Performance</h2>
          <div class="setting-group">
            ${this.settings.get('cpu-limit').build().toString()}
          </div>
          <div class="setting-group">
            ${this.settings.get('memory-limit').build().toString()}
          </div>
        </div>
        
        <div class="settings-section time-settings">
          <h2>Schedule</h2>
          <div class="setting-group">
            ${this.settings.get('active-hours').build().toString()}
          </div>
          <div class="setting-group">
            ${this.settings.get('notification-quiet').build().toString()}
          </div>
        </div>
        
        <div class="settings-section climate-settings">
          <h2>Climate Control</h2>
          <div class="vertical-control">
            ${this.settings.get('target-temp').build().toString()}
          </div>
        </div>
        
        <div class="settings-actions">
          <button onclick="this.resetToDefaults()">Reset to Defaults</button>
          <button onclick="console.log(this.getAllSettings())">Show All Settings</button>
        </div>
      </div>
    `
  }
}

const settingsDashboard = new SettingsDashboard()
console.log(settingsDashboard.render())

// Example usage
setTimeout(() => {
  settingsDashboard.updateSetting('brightness', 90)
  settingsDashboard.updateSetting('active-hours', { start: 9, end: 17 })
  console.log('Updated settings:', settingsDashboard.getAllSettings())
}, 2000)
```

## CSS Styling

```css
/* Slider base styles */
.slider {
  display: inline-block;
  position: relative;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

/* Horizontal sliders */
.slider-horizontal {
  width: 200px;
  height: 20px;
}

.slider-horizontal .slider-track {
  width: 100%;
  height: 4px;
  background: #e2e8f0;
  border-radius: 2px;
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}

.slider-horizontal .slider-fill {
  height: 100%;
  background: #3b82f6;
  border-radius: 2px;
  transition: width 0.2s ease;
}

.slider-horizontal .slider-handle {
  width: 16px;
  height: 16px;
  background: #ffffff;
  border: 2px solid #3b82f6;
  border-radius: 50%;
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  cursor: pointer;
  transition: all 0.2s ease;
}

.slider-horizontal .slider-handle:hover {
  border-width: 3px;
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
}

.slider-horizontal .slider-handle:focus {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

/* Vertical sliders */
.slider-vertical {
  width: 20px;
  height: 200px;
}

.slider-vertical .slider-track {
  width: 4px;
  height: 100%;
  background: #e2e8f0;
  border-radius: 2px;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.slider-vertical .slider-fill {
  width: 100%;
  background: #3b82f6;
  border-radius: 2px;
  position: absolute;
  bottom: 0;
  transition: height 0.2s ease;
}

.slider-vertical .slider-handle {
  width: 16px;
  height: 16px;
  background: #ffffff;
  border: 2px solid #3b82f6;
  border-radius: 50%;
  position: absolute;
  left: 50%;
  transform: translate(-50%, 50%);
  cursor: pointer;
  transition: all 0.2s ease;
}

/* Range sliders */
.slider-range .slider-fill {
  position: absolute;
  left: var(--range-start);
  width: calc(var(--range-end) - var(--range-start));
}

.slider-range .slider-handle:first-of-type {
  left: var(--range-start);
}

.slider-range .slider-handle:last-of-type {
  left: var(--range-end);
}

/* Value display */
.slider-values {
  display: flex;
  justify-content: space-between;
  margin-top: 0.5rem;
  font-size: 0.875rem;
  color: #6b7280;
}

.slider-vertical .slider-values {
  flex-direction: column;
  margin-top: 0;
  margin-left: 1.5rem;
  height: 100%;
  justify-content: space-between;
}

.slider-current-value {
  position: absolute;
  top: -1.5rem;
  left: 50%;
  transform: translateX(-50%);
  background: #374151;
  color: white;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
  white-space: nowrap;
}

.slider-vertical .slider-current-value {
  top: 50%;
  left: -3rem;
  transform: translateY(-50%);
}

/* Label and description */
.slider-label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.slider-description {
  font-size: 0.875rem;
  color: #6b7280;
  margin-bottom: 0.5rem;
}

/* Specialized slider styles */
.volume-slider .slider-fill {
  background: linear-gradient(to right, #10b981, #f59e0b, #ef4444);
}

.temperature-control .slider-fill {
  background: linear-gradient(to top, #3b82f6, #ef4444);
}

.audio-mixer {
  margin: 0.5rem;
}

.audio-mixer .slider-handle {
  border-color: #10b981;
}

.opacity-control .slider-fill {
  background: linear-gradient(to right, transparent, #000000);
}

/* EQ controls */
.eq-control .slider-track {
  background: linear-gradient(to right, #ef4444, #6b7280, #10b981);
}

.eq-control .slider-handle {
  border-color: #6b7280;
}

/* Pan controls */
.pan-slider .slider-track {
  background: linear-gradient(to right, #8b5cf6, #6b7280, #06b6d4);
}

.pan-control .slider-fill {
  background: transparent;
}

.pan-control .slider-handle {
  border-color: #6b7280;
  background: #f3f4f6;
}

/* Disabled state */
.slider[disabled] {
  opacity: 0.6;
  cursor: not-allowed;
}

.slider[disabled] .slider-handle {
  cursor: not-allowed;
  border-color: #9ca3af;
}

.slider[disabled] .slider-fill {
  background: #9ca3af;
}

/* Focus and hover states */
.slider:focus-within {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
  border-radius: 4px;
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .slider-track {
    border: 1px solid #000000;
  }
  
  .slider-handle {
    border-width: 3px;
    border-color: #000000;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .slider-track {
    background: #4b5563;
  }
  
  .slider-handle {
    background: #1f2937;
    border-color: #60a5fa;
  }
  
  .slider-label {
    color: #e5e7eb;
  }
  
  .slider-description {
    color: #9ca3af;
  }
  
  .slider-values {
    color: #9ca3af;
  }
  
  .slider-current-value {
    background: #1f2937;
    color: #e5e7eb;
  }
}

/* Animation for smooth interactions */
.slider-handle {
  transition: transform 0.1s ease, box-shadow 0.2s ease;
}

.slider-handle:active {
  transform: translate(-50%, -50%) scale(1.1);
  box-shadow: 0 4px 8px rgba(59, 130, 246, 0.4);
}

.slider-vertical .slider-handle:active {
  transform: translate(-50%, 50%) scale(1.1);
}

/* Responsive design */
@media (max-width: 768px) {
  .slider-horizontal {
    width: 150px;
  }
  
  .slider-vertical {
    height: 150px;
  }
  
  .slider-handle {
    width: 20px;
    height: 20px;
  }
  
  .slider-current-value {
    font-size: 0.875rem;
    padding: 0.375rem 0.75rem;
  }
}
```

## Accessibility Features

The Slider widget includes comprehensive accessibility support:

```typescript
const accessibleSlider = slider({
  id: 'accessible-volume',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 100,
  value: 75,
  step: 5,
  label: 'System Volume',
  description: 'Adjust the system-wide volume level',
  showValues: true
})

// Automatically includes:
// - role="slider"
// - aria-valuemin, aria-valuemax, aria-valuenow
// - aria-orientation
// - aria-valuetext for range sliders
// - aria-description for additional context
// - Proper keyboard navigation (arrow keys, Page Up/Down, Home/End)
```

## Best Practices

### 1. Choose Appropriate Ranges and Steps

```typescript
// ✅ Good - meaningful ranges with appropriate steps
const volumeSlider = slider({
  id: 'volume',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 100,
  value: 75,
  step: 5, // 5% increments make sense for volume
  showPercentage: true
})

const temperatureSlider = slider({
  id: 'temperature',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 60,
  max: 80,
  value: 72,
  step: 0.5, // Half-degree precision for temperature
  showValues: true
})
```

### 2. Use Range Sliders for Intervals

```typescript
// ✅ Good - range slider for time intervals
const workingHours = rangeSlider({
  id: 'working-hours',
  min: 0,
  max: 24,
  value: 9,     // 9 AM start
  rangeEnd: 17, // 5 PM end
  step: 0.5,    // 30-minute increments
  label: 'Working Hours',
  showValues: true
})
```

### 3. Provide Clear Labels and Value Display

```typescript
// ✅ Good - descriptive labels with value display
const complexSlider = slider({
  id: 'memory-allocation',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 1,
  max: 32,
  value: 8,
  step: 0.5,
  label: 'Memory Allocation',
  description: 'Amount of RAM to allocate to the application (GB)',
  showValues: true,
  classes: ['memory-control']
})
```

### 4. Use Vertical Orientation Appropriately

```typescript
// ✅ Good - vertical for controls that represent vertical concepts
const elevatorFloor = verticalSlider({
  id: 'elevator-floor',
  min: 1,
  max: 50,
  value: 1,
  step: 1,
  label: 'Floor Selection',
  showValues: true,
  classes: ['elevator-control']
})

const audioLevel = verticalSlider({
  id: 'audio-level-meter',
  min: -60,
  max: 0,
  value: -20,
  step: 1,
  label: 'Audio Level (dB)',
  showValues: true,
  classes: ['audio-meter']
})
```

### 5. Implement Proper Step Values

```typescript
// ✅ Good - appropriate step values for different use cases
const precisionSlider = slider({
  id: 'precision-control',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 1,
  value: 0.5,
  step: 0.01, // High precision for decimal values
  label: 'Precision Value',
  showValues: true
})

const discreteSlider = slider({
  id: 'discrete-levels',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 1,
  max: 10,
  value: 5,
  step: 1, // Integer steps only
  label: 'Quality Level',
  showValues: true
})
```

## Related Widgets

- **[Input](./input)** - Numeric input fields as alternative to sliders
- **[Progress](./progress)** - Progress indicators using similar visual patterns
- **[Radio](./radio)** - Discrete value selection alternative
- **[Switch](./switch)** - Binary on/off controls

## Examples

- **[Basic Sliders](../../examples/basic/slider-basic)** - Simple slider implementations
- **[Range Sliders](../../examples/basic/slider-range)** - Dual-handle range examples
- **[Audio Console](../../examples/advanced/audio-mixer)** - Professional audio mixing interface
- **[Settings Dashboard](../../examples/apps/settings-sliders)** - Comprehensive settings interface
- **[Data Visualization](../../examples/advanced/data-range)** - Interactive data range selection

The Slider widget provides comprehensive range selection functionality with support for single and dual-handle modes, multiple orientations, and extensive customization options for building sophisticated control interfaces.