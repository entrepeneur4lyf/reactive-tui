# Progress Widget

The Progress widget provides comprehensive progress indication functionality with multiple visual styles, animation types, builder patterns, and 15+ built-in spinner types. It integrates seamlessly with the TUI framework via FFI and supports both determinate and indeterminate progress states.

## Basic Usage

```typescript
import { 
  progress, 
  linearProgress, 
  circularProgress, 
  arcProgress, 
  spinner,
  ProgressBuilder,
  ProgressStyle, 
  ProgressState,
  SPINNER_TYPES 
} from 'reactive-tui';

// Basic linear progress
const basicProgress = progress({
  id: 'download-progress',
  style: ProgressStyle.Linear,
  value: 65,
  min: 0,
  max: 100,
  label: 'Downloading files...',
  showPercentage: true
});

// Convenience function
const fileUpload = linearProgress({
  id: 'file-upload',
  value: 75,
  label: 'Uploading...',
  showPercentage: true
});

// Spinner with custom type
const loadingSpinner = spinner({
  id: 'loading',
  label: 'Processing...',
  spinnerType: SPINNER_TYPES.dots
});
```

## Types

### ProgressStyle

```typescript
export enum ProgressStyle {
  Linear = 'linear',
  Circular = 'circular', 
  Arc = 'arc',
  Spinner = 'spinner'
}
```

### ProgressState

```typescript
export enum ProgressState {
  Determinate = 'determinate',
  Indeterminate = 'indeterminate'
}
```

## Configuration

### ProgressConfig

```typescript
interface ProgressConfig {
  id?: string;
  style: ProgressStyle;
  value?: number;
  min?: number;
  max?: number;
  label?: string;
  showPercentage?: boolean;
  showValue?: boolean;
  state?: ProgressState;
  spinnerType?: string;
  classes?: string[];
}
```

## Examples

### Linear Progress Bar

```typescript
import { progress, ProgressStyle } from 'reactive-tui'

const linearProgress = progress({
  id: 'linear-progress',
  style: ProgressStyle.Linear,
  value: 75,
  min: 0,
  max: 100,
  label: 'Processing...',
  showPercentage: true
})
```

### Circular Progress

```typescript
const circularProgress = progress({
  id: 'circular-progress',
  style: ProgressStyle.Circular,
  value: 50,
  min: 0,
  max: 100,
  label: 'Loading data'
})
```

### Arc Progress

```typescript
const arcProgress = progress({
  id: 'arc-progress',
  style: ProgressStyle.Arc,
  value: 85,
  min: 0,
  max: 100,
  showPercentage: true
})
```

### Indeterminate Progress

```typescript
const indeterminateProgress = progress({
  id: 'indeterminate-progress',
  style: ProgressStyle.Linear,
  state: ProgressState.Indeterminate,
  label: 'Please wait...'
})
```

### Spinner with Custom Type

```typescript
const customSpinner = progress({
  id: 'custom-spinner',
  style: ProgressStyle.Spinner,
  state: ProgressState.Indeterminate,
  spinnerType: 'dots',
  label: 'Loading...'
})
```

## Convenience Functions

The Progress widget provides 4 convenience functions for common progress patterns:

### linearProgress

Creates a linear progress bar:

```typescript
function linearProgress(props: {
  id?: string;
  value?: number;
  min?: number;
  max?: number;
  label?: string;
  showPercentage?: boolean;
  classes?: string[];
}): ElementBuilder
```

```typescript
import { linearProgress } from 'reactive-tui'

// File upload progress
const uploadProgress = linearProgress({
  id: 'file-upload',
  value: 75,
  min: 0,
  max: 100,
  label: 'Uploading document.pdf...',
  showPercentage: true,
  classes: ['upload-progress']
});

// Task completion progress
const taskProgress = linearProgress({
  id: 'task-completion',
  value: 8,
  min: 0,
  max: 10,
  label: 'Processing tasks: 8/10 completed',
  showPercentage: false
});
```

### circularProgress

Creates a circular progress indicator:

```typescript
function circularProgress(props: {
  id?: string;
  value?: number;
  min?: number;
  max?: number;
  label?: string;
  classes?: string[];
}): ElementBuilder
```

```typescript
import { circularProgress } from 'reactive-tui'

// Data loading progress
const dataLoading = circularProgress({
  id: 'data-loading',
  value: 65,
  min: 0,
  max: 100,
  label: 'Loading user data...',
  classes: ['data-loader']
});

// System health indicator
const systemHealth = circularProgress({
  id: 'system-health',
  value: 92,
  min: 0,
  max: 100,
  label: 'System Health: 92%'
});
```

### arcProgress

Creates an arc-style progress indicator:

```typescript
function arcProgress(props: {
  id?: string;
  value?: number;
  min?: number;
  max?: number;
  label?: string;
  classes?: string[];
}): ElementBuilder
```

```typescript
import { arcProgress } from 'reactive-tui'

// Completion arc
const completionArc = arcProgress({
  id: 'completion-arc',
  value: 85,
  min: 0,
  max: 100,
  label: 'Course Progress: 85%',
  classes: ['completion-indicator']
});

// Storage usage arc
const storageUsage = arcProgress({
  id: 'storage-usage',
  value: 1.2,
  min: 0,
  max: 2.0,
  label: 'Storage: 1.2GB / 2.0GB'
});
```

### spinner

Creates an indeterminate spinner:

```typescript
function spinner(props: {
  id?: string;
  label?: string;
  spinnerType?: string;
  classes?: string[];
}): ElementBuilder
```

```typescript
import { spinner, SPINNER_TYPES } from 'reactive-tui'

// Basic loading spinner
const basicSpinner = spinner({
  id: 'basic-loading',
  label: 'Loading...',
  spinnerType: SPINNER_TYPES.dots
});

// API request spinner
const apiSpinner = spinner({
  id: 'api-request',
  label: 'Fetching data from server...',
  spinnerType: SPINNER_TYPES.clock,
  classes: ['api-loader']
});

// Processing spinner with style
const processingSpinner = spinner({
  id: 'processing',
  label: 'Processing your request...',
  spinnerType: SPINNER_TYPES.aesthetic,
  classes: ['processing-indicator']
});
```

## Built-in Spinner Types

The Progress widget includes 15 built-in spinner animation types:

```typescript
export const SPINNER_TYPES = {
  dots: 'dots',                    // Classic braille dots animation
  dots2: 'dots2',                  // Alternative dots pattern
  line: 'line',                    // Horizontal line animation
  simpleDots: 'simple-dots',       // Simple dot sequence
  growVertical: 'grow-vertical',   // Vertical growing bars
  growHorizontal: 'grow-horizontal', // Horizontal growing bars
  clock: 'clock',                  // Clock face animation
  moon: 'moon',                    // Moon phases
  arrow: 'arrow',                  // Rotating arrow
  bouncingBar: 'bouncing-bar',     // Bouncing progress bar
  circleQuarters: 'circle-quarters', // Quarter circle rotation
  triangle: 'triangle',            // Rotating triangle
  hearts: 'hearts',                // Heart symbols
  weather: 'weather',              // Weather symbols
  aesthetic: 'aesthetic'           // Stylized aesthetic symbols
};
```

### Spinner Examples by Category

```typescript
import { spinner, SPINNER_TYPES } from 'reactive-tui'

// Technical spinners for system operations
const systemSpinners = [
  spinner({ 
    id: 'dots-spinner', 
    spinnerType: SPINNER_TYPES.dots, 
    label: 'System processing...' 
  }),
  spinner({ 
    id: 'line-spinner', 
    spinnerType: SPINNER_TYPES.line, 
    label: 'Loading modules...' 
  }),
  spinner({ 
    id: 'bars-spinner', 
    spinnerType: SPINNER_TYPES.growVertical, 
    label: 'Analyzing data...' 
  })
];

// Time-based spinners for scheduling operations
const timeSpinners = [
  spinner({ 
    id: 'clock-spinner', 
    spinnerType: SPINNER_TYPES.clock, 
    label: 'Scheduling tasks...' 
  }),
  spinner({ 
    id: 'moon-spinner', 
    spinnerType: SPINNER_TYPES.moon, 
    label: 'Night mode processing...' 
  })
];

// Visual/aesthetic spinners for user-facing operations
const aestheticSpinners = [
  spinner({ 
    id: 'hearts-spinner', 
    spinnerType: SPINNER_TYPES.hearts, 
    label: 'Saving favorites...' 
  }),
  spinner({ 
    id: 'aesthetic-spinner', 
    spinnerType: SPINNER_TYPES.aesthetic, 
    label: 'Applying theme...' 
  })
];

// Directional spinners for navigation/movement
const directionalSpinners = [
  spinner({ 
    id: 'arrow-spinner', 
    spinnerType: SPINNER_TYPES.arrow, 
    label: 'Navigating...' 
  }),
  spinner({ 
    id: 'triangle-spinner', 
    spinnerType: SPINNER_TYPES.triangle, 
    label: 'Moving forward...' 
  })
];

// Context-specific spinners
const contextSpinners = [
  spinner({ 
    id: 'weather-spinner', 
    spinnerType: SPINNER_TYPES.weather, 
    label: 'Fetching weather data...' 
  }),
  spinner({ 
    id: 'bouncing-spinner', 
    spinnerType: SPINNER_TYPES.bouncingBar, 
    label: 'Optimizing performance...' 
  })
];
```

## ProgressBuilder Pattern

The ProgressBuilder class provides a fluent interface for creating complex progress configurations:

```typescript
class ProgressBuilder {
  static create(): ProgressBuilder
  id(id: string): this
  style(style: ProgressStyle): this
  value(value: number): this
  range(min: number, max: number): this
  label(label: string): this
  showPercentage(show?: boolean): this
  showValue(show?: boolean): this
  indeterminate(): this
  spinnerType(type: string): this
  classes(classes: string[]): this
  build(): ElementBuilder
}
```

### Builder Examples

```typescript
import { ProgressBuilder, ProgressStyle, SPINNER_TYPES } from 'reactive-tui'

// Complex linear progress with all options
const complexProgress = ProgressBuilder.create()
  .id('complex-progress')
  .style(ProgressStyle.Linear)
  .value(45)
  .range(0, 100)
  .label('Multi-step installation process')
  .showPercentage(true)
  .showValue(true)
  .classes(['installation-progress', 'highlighted'])
  .build();

// Circular progress for dashboard
const dashboardMetric = ProgressBuilder.create()
  .id('cpu-usage')
  .style(ProgressStyle.Circular)
  .value(35)
  .range(0, 100)
  .label('CPU Usage')
  .classes(['metric-indicator', 'cpu-metric'])
  .build();

// Indeterminate spinner with custom type
const customSpinner = ProgressBuilder.create()
  .id('custom-spinner')
  .style(ProgressStyle.Spinner)
  .indeterminate()
  .spinnerType(SPINNER_TYPES.weather)
  .label('Synchronizing weather data...')
  .classes(['weather-sync'])
  .build();

// Arc progress for completion tracking
const completionTracker = ProgressBuilder.create()
  .id('completion-tracker')
  .style(ProgressStyle.Arc)
  .value(8)
  .range(0, 10)
  .label('Steps completed')
  .showValue(true)
  .classes(['completion-arc'])
  .build();
```

## Factory Functions

The Progress widget includes 4 factory functions for quick builder creation:

### createProgress

```typescript
function createProgress(style?: ProgressStyle): ProgressBuilder
```

```typescript
import { createProgress, ProgressStyle } from 'reactive-tui'

// Generic progress with default linear style
const genericProgress = createProgress()
  .id('generic-progress')
  .value(50)
  .label('Processing...')
  .showPercentage(true)
  .build();

// Circular progress using factory
const circularFromFactory = createProgress(ProgressStyle.Circular)
  .id('circular-factory')
  .value(75)
  .label('Loading content...')
  .build();
```

### createLinearProgress

```typescript
function createLinearProgress(): ProgressBuilder
```

```typescript
import { createLinearProgress } from 'reactive-tui'

const linearBuilder = createLinearProgress()
  .id('linear-from-factory')
  .value(80)
  .label('Upload progress')
  .showPercentage(true)
  .classes(['upload-bar'])
  .build();
```

### createCircularProgress

```typescript
function createCircularProgress(): ProgressBuilder
```

```typescript
import { createCircularProgress } from 'reactive-tui'

const circularBuilder = createCircularProgress()
  .id('circular-from-factory')
  .value(65)
  .label('Data synchronization')
  .classes(['sync-indicator'])
  .build();
```

### createSpinner

```typescript
function createSpinner(type?: string): ProgressBuilder
```

```typescript
import { createSpinner, SPINNER_TYPES } from 'reactive-tui'

// Default spinner (dots)
const defaultSpinner = createSpinner()
  .id('default-spinner')
  .label('Loading...')
  .build();

// Custom spinner type
const heartSpinner = createSpinner(SPINNER_TYPES.hearts)
  .id('heart-spinner')
  .label('Sending love...')
  .classes(['love-indicator'])
  .build();

// Aesthetic spinner for premium features
const premiumSpinner = createSpinner(SPINNER_TYPES.aesthetic)
  .id('premium-spinner')
  .label('Activating premium features...')
  .classes(['premium-loader'])
  .build();
```

## Real-World Examples

### Advanced File Upload Manager

```typescript
import { 
  linearProgress, 
  spinner, 
  createLinearProgress,
  ProgressBuilder,
  ProgressStyle,
  SPINNER_TYPES 
} from 'reactive-tui'

class FileUploadManager {
  private uploads: Map<string, any> = new Map();
  private totalBytes: number = 0;
  private uploadedBytes: number = 0;
  private overallProgress: any;

  constructor() {
    this.setupOverallProgress();
  }

  private setupOverallProgress() {
    this.overallProgress = createLinearProgress()
      .id('overall-upload-progress')
      .value(0)
      .range(0, 100)
      .label('Upload Queue: 0 files')
      .showPercentage(true)
      .showValue(false)
      .classes(['overall-progress', 'upload-manager'])
      .build();
  }

  startUpload(fileId: string, fileName: string, fileSize: number): any {
    this.totalBytes += fileSize;
    
    const fileProgress = linearProgress({
      id: `upload-${fileId}`,
      value: 0,
      min: 0,
      max: 100,
      label: `Preparing ${fileName}...`,
      showPercentage: true,
      classes: ['file-upload-progress']
    });

    this.uploads.set(fileId, {
      progress: fileProgress,
      fileName,
      fileSize,
      uploadedBytes: 0,
      status: 'preparing'
    });

    this.updateOverallProgress();
    return fileProgress;
  }

  updateFileProgress(fileId: string, uploadedBytes: number) {
    const upload = this.uploads.get(fileId);
    if (!upload) return;

    const previousBytes = upload.uploadedBytes;
    upload.uploadedBytes = uploadedBytes;
    upload.status = uploadedBytes >= upload.fileSize ? 'completed' : 'uploading';
    
    // Update global uploaded bytes
    this.uploadedBytes += (uploadedBytes - previousBytes);
    
    // Update file progress
    const percentage = Math.round((uploadedBytes / upload.fileSize) * 100);
    upload.progress.attr('value', percentage.toString());
    
    const status = upload.status === 'completed' ? '✅ Completed' : 'Uploading';
    const bytesText = `${this.formatBytes(uploadedBytes)} / ${this.formatBytes(upload.fileSize)}`;
    upload.progress.attr('label', `${status}: ${upload.fileName} (${bytesText})`);
    
    this.updateOverallProgress();
  }

  private updateOverallProgress() {
    const totalFiles = this.uploads.size;
    const completedFiles = Array.from(this.uploads.values())
      .filter(upload => upload.status === 'completed').length;
    
    const overallPercentage = this.totalBytes > 0 
      ? Math.round((this.uploadedBytes / this.totalBytes) * 100)
      : 0;
    
    this.overallProgress.attr('value', overallPercentage.toString());
    this.overallProgress.attr('label', 
      `Upload Queue: ${completedFiles}/${totalFiles} files completed (${this.formatBytes(this.uploadedBytes)} / ${this.formatBytes(this.totalBytes)})`);
  }

  showProcessingSpinner(fileId: string, operation: string) {
    const upload = this.uploads.get(fileId);
    if (!upload) return;

    const processingSpinner = spinner({
      id: `processing-${fileId}`,
      label: `${operation} ${upload.fileName}...`,
      spinnerType: SPINNER_TYPES.dots2,
      classes: ['file-processing']
    });

    upload.processingSpinner = processingSpinner;
    return processingSpinner;
  }

  hideProcessingSpinner(fileId: string) {
    const upload = this.uploads.get(fileId);
    if (upload?.processingSpinner) {
      upload.processingSpinner.attr('style', 'display: none');
      delete upload.processingSpinner;
    }
  }

  private formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  getUploadStats() {
    return {
      totalFiles: this.uploads.size,
      completedFiles: Array.from(this.uploads.values())
        .filter(upload => upload.status === 'completed').length,
      totalBytes: this.totalBytes,
      uploadedBytes: this.uploadedBytes,
      overallPercentage: this.totalBytes > 0 
        ? Math.round((this.uploadedBytes / this.totalBytes) * 100) 
        : 0
    };
  }

  clearCompleted() {
    const completed = Array.from(this.uploads.entries())
      .filter(([_, upload]) => upload.status === 'completed');
    
    completed.forEach(([fileId, upload]) => {
      this.totalBytes -= upload.fileSize;
      this.uploadedBytes -= upload.uploadedBytes;
      this.uploads.delete(fileId);
    });
    
    this.updateOverallProgress();
  }
}

// Usage
const uploadManager = new FileUploadManager();

// Start uploads
const progress1 = uploadManager.startUpload('file1', 'document.pdf', 2048576);
const progress2 = uploadManager.startUpload('file2', 'image.jpg', 1024000);

// Simulate upload progress
let uploaded1 = 0;
let uploaded2 = 0;

const interval = setInterval(() => {
  uploaded1 += 102400; // 100KB chunks
  uploaded2 += 51200;  // 50KB chunks
  
  if (uploaded1 <= 2048576) {
    uploadManager.updateFileProgress('file1', uploaded1);
  }
  
  if (uploaded2 <= 1024000) {
    uploadManager.updateFileProgress('file2', uploaded2);
  }
  
  if (uploaded1 > 2048576 && uploaded2 > 1024000) {
    clearInterval(interval);
    console.log('All uploads completed:', uploadManager.getUploadStats());
  }
}, 100);
```

### Application Installation Wizard

```typescript
import { 
  ProgressBuilder, 
  linearProgress, 
  spinner, 
  circularProgress,
  ProgressStyle, 
  SPINNER_TYPES 
} from 'reactive-tui'

interface InstallationStep {
  id: string;
  name: string;
  description: string;
  estimatedDuration: number;
  hasSubProgress?: boolean;
}

class InstallationWizard {
  private steps: InstallationStep[];
  private currentStepIndex: number = 0;
  private overallProgress: any;
  private currentStepProgress: any;
  private statusSpinner: any;
  private isInstalling: boolean = false;

  constructor(steps: InstallationStep[]) {
    this.steps = steps;
    this.setupProgressIndicators();
  }

  private setupProgressIndicators() {
    // Overall installation progress
    this.overallProgress = ProgressBuilder.create()
      .id('overall-installation-progress')
      .style(ProgressStyle.Linear)
      .value(0)
      .range(0, 100)
      .label('Installation Progress')
      .showPercentage(true)
      .showValue(false)
      .classes(['overall-progress', 'installation-wizard'])
      .build();

    // Current step progress
    this.currentStepProgress = linearProgress({
      id: 'current-step-progress',
      value: 0,
      min: 0,
      max: 100,
      label: 'Ready to install',
      showPercentage: true,
      classes: ['step-progress']
    });

    // Status spinner for indeterminate operations
    this.statusSpinner = spinner({
      id: 'status-spinner',
      label: 'Preparing installation...',
      spinnerType: SPINNER_TYPES.dots,
      classes: ['status-spinner']
    });
  }

  async startInstallation(): Promise<void> {
    if (this.isInstalling) return;
    
    this.isInstalling = true;
    this.showStatusSpinner('Initializing installation...');
    
    try {
      for (let i = 0; i < this.steps.length; i++) {
        this.currentStepIndex = i;
        await this.executeStep(this.steps[i]);
        this.updateOverallProgress();
      }
      
      this.completeInstallation();
    } catch (error) {
      this.handleInstallationError(error);
    } finally {
      this.isInstalling = false;
    }
  }

  private async executeStep(step: InstallationStep): Promise<void> {
    this.updateStepProgress(step, 0);
    this.showStatusSpinner(`Executing: ${step.name}...`);
    
    if (step.hasSubProgress) {
      // Simulate step with sub-progress
      for (let progress = 0; progress <= 100; progress += 5) {
        await this.delay(step.estimatedDuration / 20);
        this.updateStepProgress(step, progress);
      }
    } else {
      // Indeterminate step - just show spinner
      await this.delay(step.estimatedDuration);
      this.updateStepProgress(step, 100);
    }
    
    this.hideStatusSpinner();
  }

  private updateStepProgress(step: InstallationStep, progress: number) {
    this.currentStepProgress.attr('value', progress.toString());
    this.currentStepProgress.attr('label', 
      `Step ${this.currentStepIndex + 1}/${this.steps.length}: ${step.name} - ${step.description}`);
  }

  private updateOverallProgress() {
    const overallPercentage = Math.round(
      ((this.currentStepIndex + 1) / this.steps.length) * 100
    );
    
    this.overallProgress.attr('value', overallPercentage.toString());
    this.overallProgress.attr('label', 
      `Installation Progress: ${this.currentStepIndex + 1}/${this.steps.length} steps completed`);
  }

  private showStatusSpinner(message: string) {
    this.statusSpinner.attr('label', message);
    this.statusSpinner.attr('style', 'display: inline-block');
  }

  private hideStatusSpinner() {
    this.statusSpinner.attr('style', 'display: none');
  }

  private completeInstallation() {
    this.overallProgress.attr('value', '100');
    this.overallProgress.attr('label', '✅ Installation completed successfully!');
    
    this.currentStepProgress.attr('value', '100');
    this.currentStepProgress.attr('label', '✅ All components installed and configured');
    
    this.hideStatusSpinner();
  }

  private handleInstallationError(error: any) {
    console.error('Installation failed:', error);
    
    this.overallProgress.attr('label', '❌ Installation failed');
    this.currentStepProgress.attr('label', `❌ Error in step: ${this.steps[this.currentStepIndex].name}`);
    
    // Show error spinner
    this.statusSpinner.attr('label', 'Installation failed - check logs');
    this.statusSpinner.attr('spinnerType', SPINNER_TYPES.triangle);
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  pauseInstallation() {
    this.showStatusSpinner('Installation paused...');
    // Implementation would pause the current step
  }

  resumeInstallation() {
    this.hideStatusSpinner();
    // Implementation would resume the current step
  }

  cancelInstallation() {
    this.isInstalling = false;
    this.overallProgress.attr('label', '⚠️ Installation cancelled');
    this.currentStepProgress.attr('label', 'Installation cancelled by user');
    this.hideStatusSpinner();
  }

  getInstallationStatus() {
    return {
      isInstalling: this.isInstalling,
      currentStep: this.currentStepIndex + 1,
      totalSteps: this.steps.length,
      currentStepName: this.steps[this.currentStepIndex]?.name || 'None',
      overallProgress: Math.round(((this.currentStepIndex + 1) / this.steps.length) * 100)
    };
  }
}

// Usage
const installationSteps: InstallationStep[] = [
  {
    id: 'download',
    name: 'Download Components',
    description: 'Downloading required files from server',
    estimatedDuration: 3000,
    hasSubProgress: true
  },
  {
    id: 'extract',
    name: 'Extract Archive',
    description: 'Extracting downloaded archive',
    estimatedDuration: 2000,
    hasSubProgress: true
  },
  {
    id: 'install',
    name: 'Install Components',
    description: 'Installing application components',
    estimatedDuration: 4000,
    hasSubProgress: true
  },
  {
    id: 'configure',
    name: 'Configure Settings',
    description: 'Applying configuration settings',
    estimatedDuration: 1500,
    hasSubProgress: false
  },
  {
    id: 'finalize',
    name: 'Finalize Installation',
    description: 'Completing installation and cleanup',
    estimatedDuration: 1000,
    hasSubProgress: false
  }
];

const wizard = new InstallationWizard(installationSteps);

// Start installation
wizard.startInstallation().then(() => {
  console.log('Installation wizard completed');
}).catch(error => {
  console.error('Installation wizard failed:', error);
});
```

### System Performance Dashboard

```typescript
import { 
  circularProgress, 
  arcProgress, 
  linearProgress, 
  createCircularProgress,
  ProgressStyle 
} from 'reactive-tui'

class SystemPerformanceDashboard {
  private metrics: Map<string, any> = new Map();
  private updateInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.setupMetrics();
    this.startMonitoring();
  }

  private setupMetrics() {
    // CPU Usage - Circular progress
    const cpuMetric = circularProgress({
      id: 'cpu-usage',
      value: 0,
      min: 0,
      max: 100,
      label: 'CPU Usage: 0%',
      classes: ['metric-indicator', 'cpu-metric']
    });

    // Memory Usage - Linear progress
    const memoryMetric = linearProgress({
      id: 'memory-usage',
      value: 0,
      min: 0,
      max: 100,
      label: 'Memory Usage',
      showPercentage: true,
      classes: ['metric-indicator', 'memory-metric']
    });

    // Disk Usage - Arc progress
    const diskMetric = arcProgress({
      id: 'disk-usage',
      value: 0,
      min: 0,
      max: 100,
      label: 'Disk Usage',
      classes: ['metric-indicator', 'disk-metric']
    });

    // Network Throughput - Circular progress
    const networkMetric = createCircularProgress()
      .id('network-throughput')
      .value(0)
      .range(0, 1000) // Mbps
      .label('Network: 0 Mbps')
      .classes(['metric-indicator', 'network-metric'])
      .build();

    // System Load - Linear progress with custom range
    const loadMetric = linearProgress({
      id: 'system-load',
      value: 0,
      min: 0,
      max: 8, // 8-core system
      label: 'System Load: 0.00',
      showPercentage: false,
      classes: ['metric-indicator', 'load-metric']
    });

    this.metrics.set('cpu', cpuMetric);
    this.metrics.set('memory', memoryMetric);
    this.metrics.set('disk', diskMetric);
    this.metrics.set('network', networkMetric);
    this.metrics.set('load', loadMetric);
  }

  private startMonitoring() {
    this.updateInterval = setInterval(() => {
      this.updateMetrics();
    }, 1000);
  }

  private updateMetrics() {
    // Simulate real system metrics
    const cpuUsage = this.getRandomValue(0, 100);
    const memoryUsage = this.getRandomValue(30, 85);
    const diskUsage = this.getRandomValue(45, 75);
    const networkThroughput = this.getRandomValue(0, 500);
    const systemLoad = this.getRandomValue(0, 4);

    // Update CPU metric
    const cpuMetric = this.metrics.get('cpu');
    cpuMetric.attr('value', cpuUsage.toString());
    cpuMetric.attr('label', `CPU Usage: ${cpuUsage.toFixed(1)}%`);
    this.updateMetricClass(cpuMetric, cpuUsage, [70, 90]);

    // Update Memory metric
    const memoryMetric = this.metrics.get('memory');
    memoryMetric.attr('value', memoryUsage.toString());
    memoryMetric.attr('label', `Memory Usage: ${this.formatBytes(memoryUsage * 1024 * 1024 * 80)} / ${this.formatBytes(8 * 1024 * 1024 * 1024)}`);
    this.updateMetricClass(memoryMetric, memoryUsage, [75, 90]);

    // Update Disk metric
    const diskMetric = this.metrics.get('disk');
    diskMetric.attr('value', diskUsage.toString());
    diskMetric.attr('label', `Disk Usage: ${diskUsage.toFixed(1)}%`);
    this.updateMetricClass(diskMetric, diskUsage, [80, 95]);

    // Update Network metric
    const networkMetric = this.metrics.get('network');
    networkMetric.attr('value', networkThroughput.toString());
    networkMetric.attr('label', `Network: ${networkThroughput.toFixed(1)} Mbps`);

    // Update System Load metric
    const loadMetric = this.metrics.get('load');
    const loadPercentage = (systemLoad / 8) * 100;
    loadMetric.attr('value', loadPercentage.toString());
    loadMetric.attr('label', `System Load: ${systemLoad.toFixed(2)}`);
    this.updateMetricClass(loadMetric, loadPercentage, [60, 80]);
  }

  private updateMetricClass(metric: any, value: number, thresholds: [number, number]) {
    const [warning, critical] = thresholds;
    
    // Remove existing status classes
    const currentClasses = metric.attr('class') || '';
    const baseClasses = currentClasses.replace(/\s*(metric-normal|metric-warning|metric-critical)\s*/g, ' ').trim();
    
    let statusClass = 'metric-normal';
    if (value >= critical) {
      statusClass = 'metric-critical';
    } else if (value >= warning) {
      statusClass = 'metric-warning';
    }
    
    metric.attr('class', `${baseClasses} ${statusClass}`);
  }

  private getRandomValue(min: number, max: number): number {
    return Math.random() * (max - min) + min;
  }

  private formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  stopMonitoring() {
    if (this.updateInterval) {
      clearInterval(this.updateInterval);
      this.updateInterval = null;
    }
  }

  getMetrics() {
    return Array.from(this.metrics.values());
  }

  getMetricById(id: string) {
    return this.metrics.get(id);
  }

  setMetricThresholds(metricId: string, warning: number, critical: number) {
    // Implementation would store and use custom thresholds
    console.log(`Updated thresholds for ${metricId}: warning=${warning}, critical=${critical}`);
  }
}

// Usage
const dashboard = new SystemPerformanceDashboard();

// The dashboard will automatically update metrics every second
// To stop monitoring:
// dashboard.stopMonitoring();

// Get all metrics for rendering:
const allMetrics = dashboard.getMetrics();

// Get specific metric:
const cpuMetric = dashboard.getMetricById('cpu');
```

## CSS Styling

```css
/* Base progress styles */
.progress {
  display: inline-block;
  position: relative;
  font-family: inherit;
}

/* Linear progress bar */
.progress-linear {
  width: 100%;
  height: 8px;
  background-color: #e2e8f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-linear .progress-bar {
  height: 100%;
  background-color: #3b82f6;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-linear.progress-indeterminate .progress-bar {
  animation: indeterminate-linear 2s infinite linear;
}

@keyframes indeterminate-linear {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

/* Circular progress */
.progress-circular {
  width: 40px;
  height: 40px;
  position: relative;
}

.progress-circular svg {
  width: 100%;
  height: 100%;
  transform: rotate(-90deg);
}

.progress-circular .progress-circle-bg {
  fill: none;
  stroke: #e2e8f0;
  stroke-width: 2;
}

.progress-circular .progress-circle-fg {
  fill: none;
  stroke: #3b82f6;
  stroke-width: 2;
  stroke-linecap: round;
  transition: stroke-dashoffset 0.3s ease;
}

.progress-circular.progress-indeterminate .progress-circle-fg {
  animation: indeterminate-circular 1.5s infinite ease-in-out;
}

@keyframes indeterminate-circular {
  0% {
    stroke-dasharray: 0 150;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 75 150;
    stroke-dashoffset: -25;
  }
  100% {
    stroke-dasharray: 0 150;
    stroke-dashoffset: -100;
  }
}

/* Arc progress */
.progress-arc {
  width: 60px;
  height: 30px;
  position: relative;
}

.progress-arc svg {
  width: 100%;
  height: 100%;
}

/* Spinner styles */
.progress-spinner {
  display: inline-block;
  width: 20px;
  height: 20px;
}

.progress-spinner.spinner-dots::before {
  content: '⠋';
  animation: spinner-dots 1s infinite;
}

@keyframes spinner-dots {
  0% { content: '⠋'; }
  12.5% { content: '⠙'; }
  25% { content: '⠹'; }
  37.5% { content: '⠸'; }
  50% { content: '⠼'; }
  62.5% { content: '⠴'; }
  75% { content: '⠦'; }
  87.5% { content: '⠧'; }
  100% { content: '⠇'; }
}

.progress-spinner.spinner-clock::before {
  content: '🕐';
  animation: spinner-clock 2s infinite;
}

@keyframes spinner-clock {
  0% { content: '🕐'; }
  8.33% { content: '🕑'; }
  16.66% { content: '🕒'; }
  25% { content: '🕓'; }
  33.33% { content: '🕔'; }
  41.66% { content: '🕕'; }
  50% { content: '🕖'; }
  58.33% { content: '🕗'; }
  66.66% { content: '🕘'; }
  75% { content: '🕙'; }
  83.33% { content: '🕚'; }
  91.66% { content: '🕛'; }
  100% { content: '🕐'; }
}

/* Progress labels */
.progress-label {
  font-size: 14px;
  color: #374151;
  margin-bottom: 4px;
}

.progress-percentage {
  font-size: 12px;
  color: #6b7280;
  margin-left: 8px;
}

.progress-value {
  font-size: 12px;
  color: #6b7280;
  margin-left: 4px;
}

/* Size variants */
.progress-sm .progress-linear {
  height: 4px;
}

.progress-lg .progress-linear {
  height: 12px;
}

.progress-sm .progress-circular {
  width: 24px;
  height: 24px;
}

.progress-lg .progress-circular {
  width: 60px;
  height: 60px;
}

/* Color variants */
.progress-success .progress-bar,
.progress-success .progress-circle-fg {
  background-color: #10b981;
  stroke: #10b981;
}

.progress-warning .progress-bar,
.progress-warning .progress-circle-fg {
  background-color: #f59e0b;
  stroke: #f59e0b;
}

.progress-error .progress-bar,
.progress-error .progress-circle-fg {
  background-color: #ef4444;
  stroke: #ef4444;
}

/* Metric status classes */
.metric-normal .progress-bar,
.metric-normal .progress-circle-fg {
  background-color: #10b981;
  stroke: #10b981;
}

.metric-warning .progress-bar,
.metric-warning .progress-circle-fg {
  background-color: #f59e0b;
  stroke: #f59e0b;
}

.metric-critical .progress-bar,
.metric-critical .progress-circle-fg {
  background-color: #ef4444;
  stroke: #ef4444;
}
```

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';
import { linearProgress, spinner, SPINNER_TYPES } from 'reactive-tui'

const progressContainer = new ElementBuilderImpl('div')
  .class('progress-container')
  .child(
    linearProgress({
      id: 'main-progress',
      value: 50,
      label: 'Main Progress',
      showPercentage: true
    })
  )
  .child(
    spinner({
      id: 'loading-spinner',
      label: 'Loading...',
      spinnerType: SPINNER_TYPES.dots
    })
  )
  .build();
```

## Best Practices

1. **Progress Type Selection**
   - Use linear progress for file operations and step-by-step processes
   - Use circular progress for system metrics and compact spaces
   - Use arc progress for completion indicators and gauges
   - Use spinners for indeterminate operations

2. **User Experience**
   - Always provide meaningful labels describing the operation
   - Show percentage for determinate progress when helpful
   - Use appropriate spinner types for context (technical vs. aesthetic)
   - Update progress smoothly to avoid jarring jumps

3. **Performance**
   - Update progress indicators efficiently (debounce rapid updates)
   - Use indeterminate spinners for unknown duration operations
   - Clean up intervals and timers when components are destroyed

4. **Accessibility**
   - Provide ARIA labels for screen readers
   - Announce progress changes for important operations
   - Use sufficient color contrast for progress indicators

## Accessibility

The Progress widget includes comprehensive accessibility features:

- ARIA attributes for screen readers (`role="progressbar"`, `aria-valuenow`, `aria-valuemin`, `aria-valuemax`)
- Progress value announcements via ARIA live regions
- Semantic HTML structure with proper labeling
- High contrast mode support
- Screen reader friendly spinner descriptions

```typescript
const accessibleProgress = progress({
  id: 'accessible-progress',
  style: ProgressStyle.Linear,
  value: 50,
  min: 0,
  max: 100,
  label: 'File upload progress: 50% complete',
  showPercentage: true,
  // Automatically includes:
  // - role="progressbar"
  // - aria-valuenow="50"
  // - aria-valuemin="0"
  // - aria-valuemax="100"
  // - aria-label="File upload progress: 50% complete"
  // - aria-live="polite" for progress updates
});

const accessibleSpinner = spinner({
  id: 'accessible-spinner',
  label: 'Loading user data, please wait',
  spinnerType: SPINNER_TYPES.dots,
  // Automatically includes:
  // - role="status"
  // - aria-label="Loading user data, please wait"
  // - aria-live="polite"
  // - aria-busy="true"
});
```

The Progress widget provides comprehensive progress indication functionality with 4 convenience functions, ProgressBuilder pattern, 15+ spinner types, factory functions, and extensive customization options for building engaging user interfaces with proper visual feedback.