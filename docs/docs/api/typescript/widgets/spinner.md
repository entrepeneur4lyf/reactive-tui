# Spinner Widget

The Spinner widget provides animated loading indicators with 30+ built-in animation types, custom animation support, and flexible label positioning for comprehensive loading state management in terminal applications.

## Basic Usage

```typescript
import { spinnerWidget, SpinnerType, SpinnerLabelPosition } from 'reactive-tui';

// Basic spinner
const loadingSpinner = spinnerWidget({
  id: 'loader',
  type: SpinnerType.Dots,
  label: 'Loading...',
  labelPosition: SpinnerLabelPosition.After,
  visible: true
});
```

## Configuration

### SpinnerConfig Interface

```typescript
interface SpinnerConfig {
  id?: string;                           // Unique identifier
  type?: SpinnerType;                    // Animation type (30+ options)
  customDefinition?: SpinnerDefinition;  // Custom animation frames
  label?: string;                        // Text label
  labelPosition?: SpinnerLabelPosition;  // Label placement
  spacing?: number;                      // Space between spinner and label
  prefix?: string;                       // Text before spinner
  suffix?: string;                       // Text after spinner
  showSpinner?: boolean;                 // Show/hide spinner animation
  visible?: boolean;                     // Widget visibility
  animationState?: SpinnerAnimationState; // Animation control
  currentFrame?: number;                 // Current animation frame
  classes?: string[];                    // CSS classes
}
```

### Spinner Types

#### Classic Braille Dots
```typescript
SpinnerType.Dots        // ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
SpinnerType.Dots2       // ⣾⣽⣻⢿⡿⣟⣯⣷
```

#### Simple Character Spinners
```typescript
SpinnerType.Line            // -\|/
SpinnerType.Pipe            // ┤┘┴└├┌┬┐
SpinnerType.SimpleDots      // .   ..  ... 
SpinnerType.SimpleDotsScrolling // .   ..  ...  ..   . 
```

#### Geometric Shapes
```typescript
SpinnerType.Arc             // ◜◠◝◞◡◟
SpinnerType.Circle          // ◡⊙◠
SpinnerType.CircleQuarters  // ◴◷◶◵
SpinnerType.CircleHalves    // ◐◓◑◒
SpinnerType.SquareCorners   // ◰◳◲◱
SpinnerType.Triangle        // ◢◣◤◥
```

#### Movement Animations
```typescript
SpinnerType.Bounce          // ⠁⠂⠄⠂
SpinnerType.BoxBounce       // ▖▘▝▗
SpinnerType.GrowVertical    // ▁▃▄▅▆▇▆▅▄▃
SpinnerType.GrowHorizontal  // ▏▎▍▌▋▊▉▊▋▌▍▎
SpinnerType.Balloon         //  .oO@* 
SpinnerType.Arrow           // ←↖↑↗→↘↓↙
SpinnerType.BouncingBar     // [    ] [=   ] [==  ] etc.
SpinnerType.BouncingBall    // ( ●    ) (  ●   ) etc.
```

#### Emoji Spinners
```typescript
SpinnerType.Hearts      // 💛💙💜💚❤️
SpinnerType.Clock       // 🕛🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚
SpinnerType.Earth       // 🌍🌎🌏
SpinnerType.Moon        // 🌑🌒🌓🌔🌕🌖🌗🌘
SpinnerType.Weather     // ☀️🌤⛅️🌥☁️🌧🌨⛈
SpinnerType.Smiley      // 😄😝
SpinnerType.Monkey      // 🙈🙉🙊
SpinnerType.Runner      // 🚶🏃
SpinnerType.Christmas   // 🌲🎄
```

### Label Positioning

```typescript
enum SpinnerLabelPosition {
  Before = 'before',  // Label Spinner
  After = 'after',    // Spinner Label
  Above = 'above',    // Label\nSpinner
  Below = 'below',    // Spinner\nLabel
  None = 'none'       // Spinner only
}
```

### Animation States

```typescript
enum SpinnerAnimationState {
  Running = 'running',   // Active animation
  Paused = 'paused',     // Paused animation
  Stopped = 'stopped'    // Stopped animation
}
```

## Basic Usage

### Simple Loading Spinner

```typescript
import { spinnerWidget, SpinnerType, SpinnerLabelPosition, SpinnerAnimationState } from 'reactive-tui-ts'

const basicSpinner = spinnerWidget({
  id: 'basic-spinner',
  type: SpinnerType.Dots,
  label: 'Loading...',
  labelPosition: SpinnerLabelPosition.After,
  animationState: SpinnerAnimationState.Running,
  visible: true
})

// Build the spinner element
const element = basicSpinner.build()
console.log('Spinner element:', element)
```

### Customized Spinner

```typescript
const customSpinner = spinnerWidget({
  id: 'custom-spinner',
  type: SpinnerType.Arc,
  label: 'Processing your request',
  labelPosition: SpinnerLabelPosition.Below,
  spacing: 2,
  prefix: '⟨ ',
  suffix: ' ⟩',
  animationState: SpinnerAnimationState.Running,
  visible: true,
  classes: ['processing-spinner', 'blue-theme']
})
```

## Spinner Types

### Classic Braille Dot Spinners

```typescript
// Classic dots spinner
const dotsSpinner = spinnerWidget({
  id: 'dots',
  type: SpinnerType.Dots,
  label: 'Loading data...'
})
// Frames: ⠋ ⠙ ⠹ ⠸ ⠼ ⠴ ⠦ ⠧ ⠇ ⠏

// Alternative dots spinner
const dots2Spinner = spinnerWidget({
  id: 'dots2',
  type: SpinnerType.Dots2,
  label: 'Processing...'
})
// Frames: ⣾ ⣽ ⣻ ⢿ ⡿ ⣟ ⣯ ⣷
```

### Simple Character Spinners

```typescript
// Line spinner
const lineSpinner = spinnerWidget({
  id: 'line',
  type: SpinnerType.Line,
  label: 'Working...'
})
// Frames: - \ | /

// Pipe spinner
const pipeSpinner = spinnerWidget({
  id: 'pipe',
  type: SpinnerType.Pipe,
  label: 'Connecting...'
})
// Frames: ┤ ┘ ┴ └ ├ ┌ ┬ ┐

// Simple dots
const simpleDotsSpinner = spinnerWidget({
  id: 'simple-dots',
  type: SpinnerType.SimpleDots,
  label: 'Please wait'
})
// Frames: .   ..  ... (empty)

// Scrolling dots
const scrollingDotsSpinner = spinnerWidget({
  id: 'scrolling-dots',
  type: SpinnerType.SimpleDotsScrolling,
  label: 'Loading files'
})
// Frames: .   ..  ... ..  .   (empty)
```

### Geometric Shape Spinners

```typescript
// Arc spinner
const arcSpinner = spinnerWidget({
  id: 'arc',
  type: SpinnerType.Arc,
  label: 'Initializing...'
})
// Frames: ◜ ◠ ◝ ◞ ◡ ◟

// Circle spinner
const circleSpinner = spinnerWidget({
  id: 'circle',
  type: SpinnerType.Circle,
  label: 'Synchronizing...'
})
// Frames: ◡ ⊙ ◠

// Circle quarters
const circleQuartersSpinner = spinnerWidget({
  id: 'circle-quarters',
  type: SpinnerType.CircleQuarters,
  label: 'Building...'
})
// Frames: ◴ ◷ ◶ ◵

// Circle halves
const circleHalvesSpinner = spinnerWidget({
  id: 'circle-halves',
  type: SpinnerType.CircleHalves,
  label: 'Compiling...'
})
// Frames: ◐ ◓ ◑ ◒

// Square corners
const squareCornersSpinner = spinnerWidget({
  id: 'square-corners',
  type: SpinnerType.SquareCorners,
  label: 'Optimizing...'
})
// Frames: ◰ ◳ ◲ ◱

// Triangle
const triangleSpinner = spinnerWidget({
  id: 'triangle',
  type: SpinnerType.Triangle,
  label: 'Rendering...'
})
// Frames: ◢ ◣ ◤ ◥
```

### Movement and Growth Animations

```typescript
// Bouncing bar progress
const bouncingBarSpinner = spinnerWidget({
  id: 'bouncing-bar',
  type: SpinnerType.BouncingBar,
  label: 'Downloading...',
  labelPosition: SpinnerLabelPosition.Above
})
// Frames: [    ] [=   ] [==  ] [=== ] [====] [ ===] [  ==] [   =] ...

// Bouncing ball
const bouncingBallSpinner = spinnerWidget({
  id: 'bouncing-ball',
  type: SpinnerType.BouncingBall,
  label: 'Uploading...',
  labelPosition: SpinnerLabelPosition.Below
})
// Frames: ( ●    ) (  ●   ) (   ●  ) (    ● ) (     ●) ...

// Vertical growth
const growVerticalSpinner = spinnerWidget({
  id: 'grow-vertical',
  type: SpinnerType.GrowVertical,
  label: 'Expanding...'
})
// Frames: ▁ ▃ ▄ ▅ ▆ ▇ ▆ ▅ ▄ ▃

// Horizontal growth
const growHorizontalSpinner = spinnerWidget({
  id: 'grow-horizontal',
  type: SpinnerType.GrowHorizontal,
  label: 'Loading content...'
})
// Frames: ▏ ▎ ▍ ▌ ▋ ▊ ▉ ▊ ▋ ▌ ▍ ▎

// Balloon animation
const balloonSpinner = spinnerWidget({
  id: 'balloon',
  type: SpinnerType.Balloon,
  label: 'Inflating...'
})
// Frames: (space) . o O @ * (space)

// Arrow rotation
const arrowSpinner = spinnerWidget({
  id: 'arrow',
  type: SpinnerType.Arrow,
  label: 'Navigating...'
})
// Frames: ← ↖ ↑ ↗ → ↘ ↓ ↙
```

### Emoji Spinners

```typescript
// Hearts spinner
const heartsSpinner = spinnerWidget({
  id: 'hearts',
  type: SpinnerType.Hearts,
  label: 'Processing with love...',
  classes: ['emoji-spinner']
})
// Frames: 💛 💙 💜 💚 ❤️

// Clock spinner
const clockSpinner = spinnerWidget({
  id: 'clock',
  type: SpinnerType.Clock,
  label: 'Time-based processing...'
})
// Frames: 🕛 🕐 🕑 🕒 🕓 🕔 🕕 🕖 🕗 🕘 🕙 🕚

// Earth spinner
const earthSpinner = spinnerWidget({
  id: 'earth',
  type: SpinnerType.Earth,
  label: 'Global synchronization...'
})
// Frames: 🌍 🌎 🌏

// Moon phases
const moonSpinner = spinnerWidget({
  id: 'moon',
  type: SpinnerType.Moon,
  label: 'Lunar calculations...'
})
// Frames: 🌑 🌒 🌓 🌔 🌕 🌖 🌗 🌘

// Weather animation
const weatherSpinner = spinnerWidget({
  id: 'weather',
  type: SpinnerType.Weather,
  label: 'Weather data processing...'
})
// Frames: ☀️ ☀️ ☀️ 🌤 ⛅️ 🌥 ☁️ 🌧 🌨 🌧 🌨 ...

// Smiley faces
const smileySpinner = spinnerWidget({
  id: 'smiley',
  type: SpinnerType.Smiley,
  label: 'Happy processing...'
})
// Frames: 😄 😝

// Monkey animation
const monkeySpinner = spinnerWidget({
  id: 'monkey',
  type: SpinnerType.Monkey,
  label: 'Monkey business...'
})
// Frames: 🙈 🙈 🙉 🙊

// Runner animation
const runnerSpinner = spinnerWidget({
  id: 'runner',
  type: SpinnerType.Runner,
  label: 'Running tasks...'
})
// Frames: 🚶 🏃

// Christmas theme
const christmasSpinner = spinnerWidget({
  id: 'christmas',
  type: SpinnerType.Christmas,
  label: 'Holiday processing...'
})
// Frames: 🌲 🎄
```

## Label Positioning

### Before and After Labels

```typescript
// Label before spinner
const beforeLabelSpinner = spinnerWidget({
  id: 'before-label',
  type: SpinnerType.Dots,
  label: 'Status:',
  labelPosition: SpinnerLabelPosition.Before,
  spacing: 1
})
// Output: "Status: ⠋"

// Label after spinner (default)
const afterLabelSpinner = spinnerWidget({
  id: 'after-label',
  type: SpinnerType.Arc,
  label: 'Loading data...',
  labelPosition: SpinnerLabelPosition.After,
  spacing: 2
})
// Output: "◜  Loading data..."
```

### Above and Below Labels

```typescript
// Label above spinner
const aboveLabelSpinner = spinnerWidget({
  id: 'above-label',
  type: SpinnerType.BouncingBar,
  label: 'Download Progress',
  labelPosition: SpinnerLabelPosition.Above
})
// Output:
// "Download Progress"
// "[==  ]"

// Label below spinner
const belowLabelSpinner = spinnerWidget({
  id: 'below-label',
  type: SpinnerType.CircleHalves,
  label: 'Please wait while we process your request',
  labelPosition: SpinnerLabelPosition.Below
})
// Output:
// "◐"
// "Please wait while we process your request"
```

### No Label

```typescript
// Spinner without label
const noLabelSpinner = spinnerWidget({
  id: 'no-label',
  type: SpinnerType.Dots,
  labelPosition: SpinnerLabelPosition.None
})
// Output: "⠋" (just the spinner)
```

## Custom Spinner Definitions

### Custom Frame Sequences

```typescript
// Create custom spinner with specific frame sequence
const customFrameSpinner = spinnerWidget({
  id: 'custom-frames',
  customDefinition: {
    frames: ['▰▱▱▱', '▰▰▱▱', '▰▰▰▱', '▰▰▰▰', '▱▰▰▰', '▱▱▰▰', '▱▱▱▰', '▱▱▱▱'],
    interval: 100,
    name: 'custom-progress'
  },
  label: 'Custom loading...',
  labelPosition: SpinnerLabelPosition.After
})

// Binary-style custom spinner
const binarySpinner = spinnerWidget({
  id: 'binary',
  customDefinition: {
    frames: ['010010', '001100', '100101', '111010', '111101', '010111', '101011', '111000'],
    interval: 80,
    name: 'binary'
  },
  label: 'Computing binary data...',
  labelPosition: SpinnerLabelPosition.After,
  classes: ['binary-spinner']
})

// DNA helix custom spinner
const dnaSpinner = spinnerWidget({
  id: 'dna',
  customDefinition: {
    frames: ['🧬   ', ' 🧬  ', '  🧬 ', '   🧬', '  🧬 ', ' 🧬  '],
    interval: 150,
    name: 'dna-helix'
  },
  label: 'Analyzing genome...',
  labelPosition: SpinnerLabelPosition.Before
})

// Matrix-style custom spinner
const matrixSpinner = spinnerWidget({
  id: 'matrix',
  customDefinition: {
    frames: [
      '█▓▒░',
      '▓▒░█',
      '▒░█▓',
      '░█▓▒'
    ],
    interval: 120,
    name: 'matrix'
  },
  label: 'Entering the Matrix...',
  labelPosition: SpinnerLabelPosition.After,
  classes: ['matrix-theme']
})
```

### Animation Control

```typescript
// Spinner with different animation states
const controlledSpinner = spinnerWidget({
  id: 'controlled',
  type: SpinnerType.Circle,
  label: 'Controlled animation',
  animationState: SpinnerAnimationState.Running,
  currentFrame: 0
})

// Start animation
const runningSpinner = spinnerWidget({
  id: 'running',
  type: SpinnerType.Dots,
  label: 'Running...',
  animationState: SpinnerAnimationState.Running
})

// Pause animation
const pausedSpinner = spinnerWidget({
  id: 'paused',
  type: SpinnerType.Arc,
  label: 'Paused',
  animationState: SpinnerAnimationState.Paused
})

// Stop animation
const stoppedSpinner = spinnerWidget({
  id: 'stopped',
  type: SpinnerType.Line,
  label: 'Stopped',
  animationState: SpinnerAnimationState.Stopped
})
```

## Pre-built Spinner Functions

### Loading Spinner

```typescript
import { createLoadingSpinner } from 'reactive-tui-ts'

const loadingSpinner = createLoadingSpinner({
  id: 'loading',
  label: 'Loading application...',
  type: SpinnerType.Dots,
  classes: ['app-loading']
})
// Pre-configured with running animation and after label position
```

### Processing Spinner

```typescript
import { createProcessingSpinner } from 'reactive-tui-ts'

const processingSpinner = createProcessingSpinner('processing', 'Processing data...')
// Uses Arc spinner type with running animation
```

### Saving Spinner

```typescript
import { createSavingSpinner } from 'reactive-tui-ts'

const savingSpinner = createSavingSpinner('saving', 'Saving changes...')
// Uses CircleHalves spinner type with running animation
```

### Custom Spinner

```typescript
import { createCustomSpinner } from 'reactive-tui-ts'

const customSpinner = createCustomSpinner({
  id: 'custom',
  type: SpinnerType.BouncingBar,
  label: 'Custom operation...',
  labelPosition: SpinnerLabelPosition.Above,
  prefix: '[ ',
  suffix: ' ]'
})
// Output:
// "Custom operation..."
// "[ [==  ] ]"
```

### Emoji Spinner

```typescript
import { createEmojiSpinner } from 'reactive-tui-ts'

const emojiSpinner = createEmojiSpinner({
  id: 'emoji',
  type: SpinnerType.Hearts,
  label: 'Processing with love...'
})
// Uses emoji animation with special styling
```

### Minimal Spinner

```typescript
import { createMinimalSpinner } from 'reactive-tui-ts'

const minimalSpinner = createMinimalSpinner('minimal', SpinnerType.Dots)
// Just the spinner animation without any labels
```

### Progress Spinner

```typescript
import { createProgressSpinner } from 'reactive-tui-ts'

const progressSpinner = createProgressSpinner('progress', 'Progress')
// Custom progress bar animation with label before
```

### Binary Spinner

```typescript
import { createBinarySpinner } from 'reactive-tui-ts'

const binarySpinner = createBinarySpinner('binary', 'Computing...')
// Binary-style animation for technical operations
```

## Real-World Examples

### Application Loading System

```typescript
import { 
  spinnerWidget, 
  SpinnerType, 
  SpinnerLabelPosition, 
  SpinnerAnimationState,
  createLoadingSpinner,
  createProcessingSpinner 
} from 'reactive-tui-ts'

class ApplicationLoader {
  private spinners: Map<string, any> = new Map()
  private currentPhase: string = ''
  private phases: LoadingPhase[] = []

  constructor() {
    this.setupLoadingPhases()
  }

  private setupLoadingPhases() {
    this.phases = [
      {
        id: 'init',
        label: 'Initializing application...',
        spinner: SpinnerType.Dots,
        duration: 1000
      },
      {
        id: 'config',
        label: 'Loading configuration...',
        spinner: SpinnerType.Arc,
        duration: 1500
      },
      {
        id: 'assets',
        label: 'Loading assets...',
        spinner: SpinnerType.BouncingBar,
        duration: 2000
      },
      {
        id: 'database',
        label: 'Connecting to database...',
        spinner: SpinnerType.CircleHalves,
        duration: 1200
      },
      {
        id: 'auth',
        label: 'Authenticating user...',
        spinner: SpinnerType.GrowHorizontal,
        duration: 800
      },
      {
        id: 'final',
        label: 'Finalizing setup...',
        spinner: SpinnerType.Star,
        duration: 600
      }
    ]
  }

  async startLoading(): Promise<void> {
    console.log('🚀 Starting application loading sequence...\n')

    for (const phase of this.phases) {
      await this.executeLoadingPhase(phase)
    }

    this.showCompletionMessage()
  }

  private async executeLoadingPhase(phase: LoadingPhase): Promise<void> {
    this.currentPhase = phase.id

    // Create spinner for this phase
    const spinner = createLoadingSpinner({
      id: phase.id,
      label: phase.label,
      type: phase.spinner,
      classes: [`loading-${phase.id}`]
    })

    this.spinners.set(phase.id, spinner)

    // Show phase start
    console.log(`📍 Phase: ${phase.label}`)
    this.renderSpinner(spinner)

    // Simulate loading work
    await this.simulateWork(phase.duration)

    // Show phase completion
    this.showPhaseComplete(phase)
    console.log('') // Empty line for spacing
  }

  private renderSpinner(spinner: any) {
    const element = spinner.build()
    console.log(`   ${element.content || 'Loading...'}`)
  }

  private async simulateWork(duration: number): Promise<void> {
    return new Promise(resolve => {
      let elapsed = 0
      const interval = 100

      const timer = setInterval(() => {
        elapsed += interval

        if (elapsed >= duration) {
          clearInterval(timer)
          resolve()
        } else {
          // Update spinner frame (in real implementation)
          // this.updateSpinnerFrame(this.currentPhase)
        }
      }, interval)
    })
  }

  private showPhaseComplete(phase: LoadingPhase) {
    console.log(`   ✅ ${phase.label.replace('...', '')} completed`)
  }

  private showCompletionMessage() {
    console.log('🎉 Application loaded successfully!')
    console.log('')
    
    // Show summary
    const totalTime = this.phases.reduce((sum, phase) => sum + phase.duration, 0)
    console.log(`📊 Loading Summary:`)
    console.log(`   Total phases: ${this.phases.length}`)
    console.log(`   Total time: ${totalTime}ms`)
    console.log(`   Average phase time: ${Math.round(totalTime / this.phases.length)}ms`)
  }

  // Get specific spinner for updates
  getSpinner(phaseId: string): any {
    return this.spinners.get(phaseId)
  }

  // Create multi-phase progress display
  createProgressDisplay(): any[] {
    return this.phases.map((phase, index) => {
      const isActive = phase.id === this.currentPhase
      const isComplete = this.phases.findIndex(p => p.id === this.currentPhase) > index
      
      let spinnerType = SpinnerType.SimpleDots
      let label = phase.label

      if (isComplete) {
        spinnerType = SpinnerType.Toggle3
        label = label.replace('...', ' ✓')
      } else if (isActive) {
        spinnerType = phase.spinner
      }

      return spinnerWidget({
        id: `progress-${phase.id}`,
        type: spinnerType,
        label,
        labelPosition: SpinnerLabelPosition.After,
        animationState: isActive ? SpinnerAnimationState.Running : SpinnerAnimationState.Stopped,
        visible: true,
        classes: [
          'progress-item',
          isActive ? 'active' : '',
          isComplete ? 'complete' : 'pending'
        ].filter(Boolean)
      })
    })
  }
}

interface LoadingPhase {
  id: string
  label: string
  spinner: SpinnerType
  duration: number
}

// Usage
const appLoader = new ApplicationLoader()

// Start loading sequence
appLoader.startLoading().then(() => {
  console.log('Application ready for use!')
})

// Create progress display component
const progressDisplay = appLoader.createProgressDisplay()
console.log('Progress display created with', progressDisplay.length, 'phases')
```

### File Operations Progress

```typescript
import { 
  spinnerWidget, 
  SpinnerType, 
  SpinnerLabelPosition, 
  createCustomSpinner 
} from 'reactive-tui-ts'

class FileOperationsManager {
  private activeOperations: Map<string, FileOperation> = new Map()
  private operationCounter: number = 0

  // Start file upload with progress spinner
  startFileUpload(fileName: string, fileSize: number): string {
    const operationId = `upload-${++this.operationCounter}`
    
    const operation: FileOperation = {
      id: operationId,
      type: 'upload',
      fileName,
      fileSize,
      progress: 0,
      startTime: Date.now(),
      spinner: this.createUploadSpinner(fileName)
    }

    this.activeOperations.set(operationId, operation)
    this.startProgressSimulation(operationId)
    
    return operationId
  }

  // Start file download with progress spinner
  startFileDownload(fileName: string, fileSize: number): string {
    const operationId = `download-${++this.operationCounter}`
    
    const operation: FileOperation = {
      id: operationId,
      type: 'download',
      fileName,
      fileSize,
      progress: 0,
      startTime: Date.now(),
      spinner: this.createDownloadSpinner(fileName)
    }

    this.activeOperations.set(operationId, operation)
    this.startProgressSimulation(operationId)
    
    return operationId
  }

  // Start file compression
  startFileCompression(fileName: string): string {
    const operationId = `compress-${++this.operationCounter}`
    
    const operation: FileOperation = {
      id: operationId,
      type: 'compress',
      fileName,
      fileSize: 0,
      progress: 0,
      startTime: Date.now(),
      spinner: this.createCompressionSpinner(fileName)
    }

    this.activeOperations.set(operationId, operation)
    this.startProgressSimulation(operationId)
    
    return operationId
  }

  private createUploadSpinner(fileName: string): any {
    return createCustomSpinner({
      id: `upload-spinner-${fileName}`,
      type: SpinnerType.BouncingBar,
      label: `Uploading ${fileName}`,
      labelPosition: SpinnerLabelPosition.Above,
      prefix: '📤 ',
      suffix: ' 0%'
    })
  }

  private createDownloadSpinner(fileName: string): any {
    return createCustomSpinner({
      id: `download-spinner-${fileName}`,
      type: SpinnerType.GrowHorizontal,
      label: `Downloading ${fileName}`,
      labelPosition: SpinnerLabelPosition.Above,
      prefix: '📥 ',
      suffix: ' 0%'
    })
  }

  private createCompressionSpinner(fileName: string): any {
    return spinnerWidget({
      id: `compress-spinner-${fileName}`,
      customDefinition: {
        frames: ['📦   ', ' 📦  ', '  📦 ', '   📦', '  📦 ', ' 📦  '],
        interval: 120,
        name: 'compression'
      },
      label: `Compressing ${fileName}`,
      labelPosition: SpinnerLabelPosition.After,
      spacing: 1
    })
  }

  private startProgressSimulation(operationId: string) {
    const operation = this.activeOperations.get(operationId)
    if (!operation) return

    const duration = this.getOperationDuration(operation)
    const updateInterval = 100

    const timer = setInterval(() => {
      const elapsed = Date.now() - operation.startTime
      const progress = Math.min(100, (elapsed / duration) * 100)
      
      operation.progress = progress
      this.updateOperationDisplay(operation)

      if (progress >= 100) {
        clearInterval(timer)
        this.completeOperation(operationId)
      }
    }, updateInterval)
  }

  private getOperationDuration(operation: FileOperation): number {
    switch (operation.type) {
      case 'upload':
        return Math.max(2000, operation.fileSize * 0.01) // Simulate based on file size
      case 'download':
        return Math.max(1500, operation.fileSize * 0.008)
      case 'compress':
        return Math.max(3000, operation.fileSize * 0.02)
      default:
        return 2000
    }
  }

  private updateOperationDisplay(operation: FileOperation) {
    const percentage = Math.round(operation.progress)
    const progressBar = this.createProgressBar(percentage)
    const speed = this.calculateSpeed(operation)
    const eta = this.calculateETA(operation)

    console.clear()
    console.log(`\n${operation.type.toUpperCase()} OPERATION`)
    console.log(`File: ${operation.fileName}`)
    console.log(`Progress: ${progressBar} ${percentage}%`)
    
    if (operation.fileSize > 0) {
      console.log(`Size: ${this.formatFileSize(operation.fileSize)}`)
      console.log(`Speed: ${speed}`)
      console.log(`ETA: ${eta}`)
    }
    
    // Render spinner
    const element = operation.spinner.build()
    console.log(`\n${element.content || 'Processing...'}`)
  }

  private createProgressBar(percentage: number): string {
    const width = 30
    const filled = Math.round((percentage / 100) * width)
    const empty = width - filled
    
    return '█'.repeat(filled) + '░'.repeat(empty)
  }

  private calculateSpeed(operation: FileOperation): string {
    if (operation.fileSize === 0) return 'N/A'
    
    const elapsed = (Date.now() - operation.startTime) / 1000
    const bytesProcessed = (operation.progress / 100) * operation.fileSize
    const speed = bytesProcessed / elapsed
    
    return `${this.formatFileSize(speed)}/s`
  }

  private calculateETA(operation: FileOperation): string {
    if (operation.progress === 0) return 'Calculating...'
    
    const elapsed = Date.now() - operation.startTime
    const remaining = (elapsed / operation.progress) * (100 - operation.progress)
    
    return this.formatDuration(remaining)
  }

  private formatFileSize(bytes: number): string {
    const units = ['B', 'KB', 'MB', 'GB']
    let size = bytes
    let unitIndex = 0

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }

    return `${size.toFixed(1)} ${units[unitIndex]}`
  }

  private formatDuration(ms: number): string {
    const seconds = Math.floor(ms / 1000) % 60
    const minutes = Math.floor(ms / (1000 * 60)) % 60
    const hours = Math.floor(ms / (1000 * 60 * 60))

    if (hours > 0) {
      return `${hours}h ${minutes}m ${seconds}s`
    } else if (minutes > 0) {
      return `${minutes}m ${seconds}s`
    } else {
      return `${seconds}s`
    }
  }

  private completeOperation(operationId: string) {
    const operation = this.activeOperations.get(operationId)
    if (!operation) return

    const duration = Date.now() - operation.startTime
    
    console.clear()
    console.log(`\n✅ ${operation.type.toUpperCase()} COMPLETED`)
    console.log(`File: ${operation.fileName}`)
    console.log(`Duration: ${this.formatDuration(duration)}`)
    
    if (operation.fileSize > 0) {
      const avgSpeed = operation.fileSize / (duration / 1000)
      console.log(`Average speed: ${this.formatFileSize(avgSpeed)}/s`)
    }

    this.activeOperations.delete(operationId)
  }

  // Get all active operations
  getActiveOperations(): FileOperation[] {
    return Array.from(this.activeOperations.values())
  }

  // Cancel operation
  cancelOperation(operationId: string): boolean {
    const operation = this.activeOperations.get(operationId)
    if (operation) {
      console.log(`\n❌ ${operation.type.toUpperCase()} CANCELLED`)
      console.log(`File: ${operation.fileName}`)
      
      this.activeOperations.delete(operationId)
      return true
    }
    return false
  }

  // Create summary display
  createSummaryDisplay(): any {
    const operations = this.getActiveOperations()
    
    if (operations.length === 0) {
      return spinnerWidget({
        id: 'no-operations',
        type: SpinnerType.Toggle3,
        label: 'No active operations',
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Stopped
      })
    }

    const summaryText = operations.map(op => {
      const percentage = Math.round(op.progress)
      return `${op.fileName}: ${percentage}%`
    }).join(' | ')

    return spinnerWidget({
      id: 'operations-summary',
      type: SpinnerType.Dots,
      label: `Active: ${summaryText}`,
      labelPosition: SpinnerLabelPosition.After,
      animationState: SpinnerAnimationState.Running
    })
  }
}

interface FileOperation {
  id: string
  type: 'upload' | 'download' | 'compress'
  fileName: string
  fileSize: number
  progress: number
  startTime: number
  spinner: any
}

// Usage
const fileOps = new FileOperationsManager()

// Start multiple file operations
const uploadId = fileOps.startFileUpload('document.pdf', 1024 * 1024 * 5) // 5MB
const downloadId = fileOps.startFileDownload('video.mp4', 1024 * 1024 * 50) // 50MB
const compressId = fileOps.startFileCompression('archive.zip')

// Create summary display
const summary = fileOps.createSummaryDisplay()
console.log('File operations summary:', summary.build().content)

// Demo cancellation after 3 seconds
setTimeout(() => {
  fileOps.cancelOperation(uploadId)
}, 3000)
```

### Status Indicator System

```typescript
import { 
  spinnerWidget, 
  SpinnerType, 
  SpinnerLabelPosition, 
  SpinnerAnimationState 
} from 'reactive-tui-ts'

class StatusIndicatorSystem {
  private indicators: Map<string, StatusIndicator> = new Map()
  private systemStatus: SystemStatus = 'initializing'

  constructor() {
    this.setupStatusIndicators()
  }

  private setupStatusIndicators() {
    // System services status
    this.addIndicator('database', {
      name: 'Database Connection',
      status: 'connecting',
      spinner: SpinnerType.Dots,
      healthCheck: () => this.checkDatabaseHealth()
    })

    this.addIndicator('api', {
      name: 'API Server',
      status: 'starting',
      spinner: SpinnerType.Arc,
      healthCheck: () => this.checkApiHealth()
    })

    this.addIndicator('cache', {
      name: 'Cache Service',
      status: 'initializing',
      spinner: SpinnerType.Circle,
      healthCheck: () => this.checkCacheHealth()
    })

    this.addIndicator('auth', {
      name: 'Authentication Service',
      status: 'loading',
      spinner: SpinnerType.CircleHalves,
      healthCheck: () => this.checkAuthHealth()
    })

    this.addIndicator('storage', {
      name: 'File Storage',
      status: 'checking',
      spinner: SpinnerType.GrowVertical,
      healthCheck: () => this.checkStorageHealth()
    })
  }

  private addIndicator(id: string, config: StatusIndicatorConfig) {
    const indicator: StatusIndicator = {
      id,
      ...config,
      lastCheck: Date.now(),
      spinner: this.createStatusSpinner(id, config)
    }

    this.indicators.set(id, indicator)
  }

  private createStatusSpinner(id: string, config: StatusIndicatorConfig): any {
    return spinnerWidget({
      id: `status-${id}`,
      type: config.spinner,
      label: `${config.name}: ${this.getStatusLabel(config.status)}`,
      labelPosition: SpinnerLabelPosition.After,
      animationState: this.getAnimationState(config.status),
      prefix: this.getStatusIcon(config.status) + ' ',
      spacing: 1
    })
  }

  private getStatusIcon(status: ServiceStatus): string {
    const icons: Record<ServiceStatus, string> = {
      'initializing': '🔄',
      'starting': '🚀',
      'connecting': '🔗',
      'loading': '📥',
      'checking': '🔍',
      'healthy': '✅',
      'warning': '⚠️',
      'error': '❌',
      'stopped': '⏹️',
      'maintenance': '🔧'
    }
    return icons[status]
  }

  private getStatusLabel(status: ServiceStatus): string {
    const labels: Record<ServiceStatus, string> = {
      'initializing': 'Initializing...',
      'starting': 'Starting up...',
      'connecting': 'Connecting...',
      'loading': 'Loading...',
      'checking': 'Health check...',
      'healthy': 'Online',
      'warning': 'Warning',
      'error': 'Error',
      'stopped': 'Stopped',
      'maintenance': 'Maintenance'
    }
    return labels[status]
  }

  private getAnimationState(status: ServiceStatus): SpinnerAnimationState {
    const inProgress: ServiceStatus[] = ['initializing', 'starting', 'connecting', 'loading', 'checking']
    return inProgress.includes(status) ? SpinnerAnimationState.Running : SpinnerAnimationState.Stopped
  }

  // Health check simulations
  private async checkDatabaseHealth(): Promise<ServiceStatus> {
    await this.delay(1000 + Math.random() * 2000)
    return Math.random() > 0.1 ? 'healthy' : 'error'
  }

  private async checkApiHealth(): Promise<ServiceStatus> {
    await this.delay(800 + Math.random() * 1500)
    const rand = Math.random()
    if (rand > 0.8) return 'error'
    if (rand > 0.6) return 'warning'
    return 'healthy'
  }

  private async checkCacheHealth(): Promise<ServiceStatus> {
    await this.delay(500 + Math.random() * 1000)
    return Math.random() > 0.05 ? 'healthy' : 'warning'
  }

  private async checkAuthHealth(): Promise<ServiceStatus> {
    await this.delay(1200 + Math.random() * 1800)
    return Math.random() > 0.15 ? 'healthy' : 'error'
  }

  private async checkStorageHealth(): Promise<ServiceStatus> {
    await this.delay(600 + Math.random() * 1200)
    const rand = Math.random()
    if (rand > 0.9) return 'error'
    if (rand > 0.7) return 'warning'
    return 'healthy'
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms))
  }

  // Start all health checks
  async startHealthChecks(): Promise<void> {
    console.log('🏥 Starting system health checks...\n')

    const checkPromises = Array.from(this.indicators.entries()).map(async ([id, indicator]) => {
      console.log(`Starting health check for ${indicator.name}...`)
      this.renderIndicator(indicator)

      try {
        const newStatus = await indicator.healthCheck()
        this.updateIndicatorStatus(id, newStatus)
      } catch (error) {
        this.updateIndicatorStatus(id, 'error')
      }
    })

    await Promise.all(checkPromises)
    this.updateSystemStatus()
  }

  private updateIndicatorStatus(id: string, status: ServiceStatus) {
    const indicator = this.indicators.get(id)
    if (indicator) {
      indicator.status = status
      indicator.lastCheck = Date.now()
      indicator.spinner = this.createStatusSpinner(id, indicator)
      
      console.log(`${this.getStatusIcon(status)} ${indicator.name}: ${this.getStatusLabel(status)}`)
    }
  }

  private renderIndicator(indicator: StatusIndicator) {
    const element = indicator.spinner.build()
    console.log(`   ${element.content}`)
  }

  private updateSystemStatus() {
    const indicators = Array.from(this.indicators.values())
    const hasErrors = indicators.some(i => i.status === 'error')
    const hasWarnings = indicators.some(i => i.status === 'warning')
    const allHealthy = indicators.every(i => i.status === 'healthy')

    if (hasErrors) {
      this.systemStatus = 'error'
    } else if (hasWarnings) {
      this.systemStatus = 'warning'
    } else if (allHealthy) {
      this.systemStatus = 'healthy'
    } else {
      this.systemStatus = 'starting'
    }

    this.displaySystemSummary()
  }

  private displaySystemSummary() {
    console.log('\n' + '='.repeat(50))
    console.log('📊 SYSTEM STATUS SUMMARY')
    console.log('='.repeat(50))

    const indicators = Array.from(this.indicators.values())
    indicators.forEach(indicator => {
      const statusDisplay = `${this.getStatusIcon(indicator.status)} ${indicator.name}`;
      const statusLabel = this.getStatusLabel(indicator.status);
      const timeSinceCheck = Date.now() - indicator.lastCheck;
      
      console.log(`${statusDisplay.padEnd(30)} ${statusLabel.padEnd(15)} (${timeSinceCheck}ms ago)`)
    })

    console.log('-'.repeat(50))
    
    const overallIcon = this.getStatusIcon(this.systemStatus)
    const overallLabel = this.getStatusLabel(this.systemStatus)
    console.log(`${overallIcon} Overall System Status: ${overallLabel}`)
    
    const healthyCount = indicators.filter(i => i.status === 'healthy').length
    const totalCount = indicators.length
    const healthPercentage = Math.round((healthyCount / totalCount) * 100)
    
    console.log(`📈 Health Score: ${healthyCount}/${totalCount} services (${healthPercentage}%)`)
    console.log('='.repeat(50))
  }

  // Create status dashboard spinner
  createDashboardSpinner(): any {
    const indicators = Array.from(this.indicators.values())
    const statusCounts = {
      healthy: indicators.filter(i => i.status === 'healthy').length,
      warning: indicators.filter(i => i.status === 'warning').length,
      error: indicators.filter(i => i.status === 'error').length,
      other: indicators.filter(i => !['healthy', 'warning', 'error'].includes(i.status)).length
    }

    const statusText = [
      statusCounts.healthy > 0 ? `✅ ${statusCounts.healthy}` : '',
      statusCounts.warning > 0 ? `⚠️ ${statusCounts.warning}` : '',
      statusCounts.error > 0 ? `❌ ${statusCounts.error}` : '',
      statusCounts.other > 0 ? `🔄 ${statusCounts.other}` : ''
    ].filter(Boolean).join(' ')

    let spinnerType = SpinnerType.Dots
    if (this.systemStatus === 'error') spinnerType = SpinnerType.Toggle2
    else if (this.systemStatus === 'warning') spinnerType = SpinnerType.Triangle
    else if (this.systemStatus === 'healthy') spinnerType = SpinnerType.Toggle3

    return spinnerWidget({
      id: 'system-dashboard',
      type: spinnerType,
      label: `System Status: ${statusText}`,
      labelPosition: SpinnerLabelPosition.After,
      animationState: this.systemStatus === 'healthy' ? SpinnerAnimationState.Stopped : SpinnerAnimationState.Running,
      prefix: this.getStatusIcon(this.systemStatus) + ' '
    })
  }

  // Continuous monitoring
  startContinuousMonitoring(intervalMs: number = 30000) {
    console.log(`🔄 Starting continuous monitoring (every ${intervalMs/1000}s)`)
    
    setInterval(async () => {
      console.log('\n🔄 Running periodic health checks...')
      await this.startHealthChecks()
    }, intervalMs)
  }

  // Get indicator by ID
  getIndicator(id: string): StatusIndicator | undefined {
    return this.indicators.get(id)
  }

  // Get system status
  getSystemStatus(): SystemStatus {
    return this.systemStatus
  }
}

// Types
type ServiceStatus = 'initializing' | 'starting' | 'connecting' | 'loading' | 'checking' | 'healthy' | 'warning' | 'error' | 'stopped' | 'maintenance'
type SystemStatus = 'initializing' | 'starting' | 'healthy' | 'warning' | 'error' | 'maintenance'

interface StatusIndicatorConfig {
  name: string
  status: ServiceStatus
  spinner: SpinnerType
  healthCheck: () => Promise<ServiceStatus>
}

interface StatusIndicator extends StatusIndicatorConfig {
  id: string
  lastCheck: number
  spinner: any
}

// Usage
const statusSystem = new StatusIndicatorSystem()

// Start initial health checks
statusSystem.startHealthChecks().then(() => {
  console.log('\n✅ Initial health checks completed')
  
  // Create dashboard spinner
  const dashboardSpinner = statusSystem.createDashboardSpinner()
  console.log('\n📊 Dashboard Status:')
  console.log(dashboardSpinner.build().content)
  
  // Start continuous monitoring
  statusSystem.startContinuousMonitoring(10000) // Every 10 seconds
})
```

## CSS Styling

```css
/* Spinner base styles */
.spinner {
  display: inline-block;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  line-height: 1;
  white-space: pre;
  vertical-align: middle;
}

/* Animation states */
.spinner-running {
  animation: spin 1s linear infinite;
}

.spinner-paused {
  animation-play-state: paused;
}

.spinner-stopped {
  animation: none;
}

.spinner-hidden {
  display: none;
}

/* Spinner types */
.spinner[data-spinner-type="dots"] {
  font-size: 1rem;
  color: #3b82f6;
}

.spinner[data-spinner-type="arc"] {
  font-size: 1.125rem;
  color: #10b981;
}

.spinner[data-spinner-type="circle"] {
  font-size: 1rem;
  color: #f59e0b;
}

.spinner[data-spinner-type="bouncingBar"] {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: #8b5cf6;
}

.spinner[data-spinner-type="bouncingBall"] {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: #06b6d4;
}

/* Emoji spinners */
.spinner-emoji {
  font-size: 1.25rem;
  filter: none;
}

.spinner[data-spinner-type="hearts"] {
  animation-duration: 0.5s;
}

.spinner[data-spinner-type="clock"] {
  animation-duration: 1.2s;
}

.spinner[data-spinner-type="earth"] {
  animation-duration: 0.54s;
}

.spinner[data-spinner-type="moon"] {
  animation-duration: 0.64s;
}

.spinner[data-spinner-type="weather"] {
  animation-duration: 2.3s;
}

/* Minimal spinner */
.spinner-minimal {
  margin: 0;
  padding: 0;
  font-size: 0.875rem;
}

/* Progress spinner */
.spinner-progress {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: #3b82f6;
  background: linear-gradient(90deg, #3b82f6 0%, #1d4ed8 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

/* Binary spinner */
.spinner-binary {
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  color: #10b981;
  text-shadow: 0 0 5px rgba(16, 185, 129, 0.5);
}

/* Custom animations */
@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

@keyframes bounce {
  0%, 20%, 50%, 80%, 100% {
    transform: translateY(0);
  }
  40% {
    transform: translateY(-10px);
  }
  60% {
    transform: translateY(-5px);
  }
}

/* Loading contexts */
.loading-spinner {
  margin: 0.5rem;
  padding: 0.5rem;
  border-radius: 0.25rem;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.processing-spinner {
  color: #f59e0b;
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.2);
  border-radius: 0.25rem;
  padding: 0.5rem;
}

.saving-spinner {
  color: #10b981;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 0.25rem;
  padding: 0.5rem;
}

/* Status-based colors */
.spinner.status-success {
  color: #10b981;
}

.spinner.status-warning {
  color: #f59e0b;
}

.spinner.status-error {
  color: #ef4444;
}

.spinner.status-info {
  color: #3b82f6;
}

/* Size variants */
.spinner-xs {
  font-size: 0.75rem;
}

.spinner-sm {
  font-size: 0.875rem;
}

.spinner-md {
  font-size: 1rem;
}

.spinner-lg {
  font-size: 1.125rem;
}

.spinner-xl {
  font-size: 1.25rem;
}

/* Dark theme */
.spinner.theme-dark {
  color: #f3f4f6;
}

.spinner.theme-dark.status-success {
  color: #34d399;
}

.spinner.theme-dark.status-warning {
  color: #fbbf24;
}

.spinner.theme-dark.status-error {
  color: #f87171;
}

.spinner.theme-dark.status-info {
  color: #60a5fa;
}

/* Light theme */
.spinner.theme-light {
  color: #374151;
}

.spinner.theme-light.status-success {
  color: #059669;
}

.spinner.theme-light.status-warning {
  color: #d97706;
}

.spinner.theme-light.status-error {
  color: #dc2626;
}

.spinner.theme-light.status-info {
  color: #2563eb;
}

/* Accessibility */
.spinner[role="status"] {
  position: relative;
}

.spinner[role="status"]:focus {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
  border-radius: 0.25rem;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .spinner-running {
    animation: none;
  }
  
  .spinner-running::after {
    content: '⏳';
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .spinner {
    filter: contrast(2);
  }
  
  .loading-spinner,
  .processing-spinner,
  .saving-spinner {
    border-width: 2px;
    background: transparent;
  }
}

/* Print styles */
@media print {
  .spinner {
    display: none;
  }
  
  .spinner::after {
    content: '[Loading...]';
    display: inline;
    color: #000000;
  }
}

/* Container layouts */
.spinner-container {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.spinner-container.vertical {
  flex-direction: column;
  text-align: center;
}

.spinner-container.horizontal {
  flex-direction: row;
  align-items: center;
}

/* Full-screen overlay */
.spinner-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.spinner-overlay .spinner {
  font-size: 2rem;
  color: #ffffff;
}

.spinner-overlay .spinner-label {
  color: #ffffff;
  font-size: 1.125rem;
  margin-top: 1rem;
  text-align: center;
}
```

## Best Practices

### 1. Choose Appropriate Spinner Types

```typescript
// ✅ Good - appropriate spinners for different contexts
const dataLoadingSpinner = spinnerWidget({
  id: 'data-loading',
  type: SpinnerType.Dots,        // Classic for general loading
  label: 'Loading data...'
})

const progressSpinner = spinnerWidget({
  id: 'progress',
  type: SpinnerType.BouncingBar, // Visual progress indication
  label: 'Processing files...'
})

const savingSpinner = spinnerWidget({
  id: 'saving',
  type: SpinnerType.CircleHalves, // Continuous action
  label: 'Saving changes...'
})

// ❌ Poor - inappropriate spinner choices
const subtleTaskSpinner = spinnerWidget({
  id: 'subtle',
  type: SpinnerType.BouncingBall, // Too distracting for background tasks
  label: 'Background sync...'
})
```

### 2. Meaningful Labels

```typescript
// ✅ Good - descriptive, actionable labels
const descriptiveSpinner = spinnerWidget({
  id: 'descriptive',
  type: SpinnerType.Arc,
  label: 'Connecting to database...',
  labelPosition: SpinnerLabelPosition.After
})

const progressiveSpinner = spinnerWidget({
  id: 'progressive',
  type: SpinnerType.GrowHorizontal,
  label: 'Uploading image (2 of 5)...',
  labelPosition: SpinnerLabelPosition.Above
})

// ❌ Poor - vague or unhelpful labels
const vagueSpinner = spinnerWidget({
  id: 'vague',
  type: SpinnerType.Dots,
  label: 'Please wait...',     // Too generic
})

const noContextSpinner = spinnerWidget({
  id: 'no-context',
  type: SpinnerType.Line,
  label: 'Loading...',         // No indication of what's loading
})
```

### 3. Proper Animation Control

```typescript
// ✅ Good - controlled animation lifecycle
const controlledSpinner = spinnerWidget({
  id: 'controlled',
  type: SpinnerType.Dots,
  label: 'Processing...',
  animationState: SpinnerAnimationState.Running
})

// Stop animation when complete
const stoppedSpinner = spinnerWidget({
  id: 'stopped',
  type: SpinnerType.Toggle3,
  label: 'Complete ✓',
  animationState: SpinnerAnimationState.Stopped
})

// ❌ Poor - always running animations
const alwaysRunningSpinner = spinnerWidget({
  id: 'always-running',
  type: SpinnerType.Arc,
  label: 'Task completed',     // Misleading - still animating
  animationState: SpinnerAnimationState.Running
})
```

### 4. Accessibility Considerations

```typescript
// ✅ Good - accessible spinner with proper ARIA attributes
const accessibleSpinner = spinnerWidget({
  id: 'accessible',
  type: SpinnerType.Dots,
  label: 'Loading user profile data',
  labelPosition: SpinnerLabelPosition.After
})
// Automatically includes role="status", aria-live="polite", aria-label

// ✅ Good - screen reader friendly
const screenReaderSpinner = spinnerWidget({
  id: 'screen-reader',
  type: SpinnerType.SimpleDots,
  label: 'Saving document - this may take a few moments',
  labelPosition: SpinnerLabelPosition.After
})

// ❌ Poor - no context for screen readers
const inaccessibleSpinner = spinnerWidget({
  id: 'inaccessible',
  type: SpinnerType.Emoji,
  labelPosition: SpinnerLabelPosition.None  // No label for context
})
```

### 5. Performance Considerations

```typescript
// ✅ Good - appropriate intervals for performance
const efficientSpinner = spinnerWidget({
  id: 'efficient',
  customDefinition: {
    frames: ['⠋', '⠙', '⠹', '⠸'],
    interval: 100,              // Reasonable interval
    name: 'efficient'
  },
  label: 'Optimized loading...'
})

// ✅ Good - minimal animation for background tasks
const backgroundSpinner = spinnerWidget({
  id: 'background',
  type: SpinnerType.Toggle,     // Simple, low-frequency animation
  label: 'Background sync',
  classes: ['background-task']
})

// ❌ Poor - too frequent updates
const excessiveSpinner = spinnerWidget({
  id: 'excessive',
  customDefinition: {
    frames: ['1', '2', '3', '4', '5', '6', '7', '8'],
    interval: 10,               // Too frequent, performance impact
    name: 'excessive'
  }
})
```

## Related Widgets

- **[Button](./button)** - Loading states in interactive buttons
- **[Modal](./modal)** - Loading overlays in modal dialogs
- **[Input](./input)** - Processing states in form inputs
- **[Toast](./toast)** - Progress notifications with spinners

## Examples

- **[Basic Spinners](../../examples/basic/spinner-basic)** - Simple loading indicators
- **[Loading States](../../examples/advanced/loading-states)** - Application loading sequences
- **[Progress Indicators](../../examples/advanced/progress-spinners)** - File operation progress
- **[Status Dashboard](../../examples/apps/status-dashboard)** - System health monitoring

The Spinner widget provides comprehensive loading indication with 30+ built-in animations, custom frame sequences, and accessibility features, making it essential for creating engaging user experiences during asynchronous operations.