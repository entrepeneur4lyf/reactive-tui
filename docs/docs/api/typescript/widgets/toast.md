# Toast Widget

The Toast widget provides non-intrusive notification messages with multiple variants, positioning options, and automatic dismissal for effective user feedback in terminal applications.

## Basic Usage

```typescript
import { toast, ToastVariant, ToastPosition } from 'reactive-tui';

// Basic toast notification
const notification = toast({
  message: 'Operation completed successfully',
  variant: ToastVariant.Success,
  position: ToastPosition.TopRight,
  duration: 3000
});
```

## Configuration

### ToastConfig Interface

```typescript
interface ToastConfig {
  id?: string;                    // Unique identifier
  message: string;                // Toast message text
  variant: ToastVariant;          // Visual style and semantic meaning
  position?: ToastPosition;       // Screen positioning
  duration?: number;              // Auto-dismiss time (ms), 0 = persistent
  dismissible?: boolean;          // Allow manual dismissal
  showProgress?: boolean;         // Show progress bar during duration
  showIcon?: boolean;            // Display variant icon
  showCloseButton?: boolean;     // Show close button
  classes?: string[];            // Additional CSS classes
}
```

### Toast Variants

```typescript
enum ToastVariant {
  Info = 'info',        // Informational messages
  Success = 'success',  // Success confirmations
  Warning = 'warning',  // Warning alerts
  Error = 'error'       // Error notifications
}
```

### Toast Positioning

```typescript
enum ToastPosition {
  TopLeft = 'top-left',
  TopCenter = 'top-center',
  TopRight = 'top-right',
  BottomLeft = 'bottom-left',
  BottomCenter = 'bottom-center',
  BottomRight = 'bottom-right'
}
```

## Toast Variants

### Info Toast

```typescript
import { infoToast, ToastPosition } from 'reactive-tui';

// infoToast accepts specific props (not full ToastConfig)
interface InfoToastProps {
  id?: string;
  message: string;
  position?: ToastPosition;
  duration?: number;
  dismissible?: boolean;
  classes?: string[];
}

// Simple info toast
const info = infoToast({
  message: 'New update available',
  position: ToastPosition.TopCenter,
  duration: 4000
});

// Persistent info toast
const persistentInfo = infoToast({
  message: 'System maintenance scheduled',
  duration: 0,
  dismissible: true
});
```

### Success Toast

```typescript
import { successToast } from 'reactive-tui';

// Success confirmation
const success = successToast({
  message: 'File saved successfully',
  position: ToastPosition.BottomRight,
  duration: 2000,
  showProgress: true
});

// Success with custom styling
const styledSuccess = successToast({
  message: 'Profile updated',
  classes: ['custom-success', 'fade-in']
});
```

### Warning Toast

```typescript
import { warningToast } from 'reactive-tui';

// Warning notification
const warning = warningToast({
  message: 'Disk space running low',
  position: ToastPosition.TopLeft,
  duration: 8000,
  showCloseButton: true
});

// Critical warning
const criticalWarning = warningToast({
  message: 'Connection unstable - changes may not be saved',
  duration: 0,
  dismissible: true,
  classes: ['critical-warning']
});
```

### Error Toast

```typescript
import { errorToast } from 'reactive-tui';

// Error notification
const error = errorToast({
  message: 'Failed to connect to server',
  position: ToastPosition.TopCenter,
  duration: 0,
  dismissible: true
});

// Validation error
const validationError = errorToast({
  message: 'Please fill in all required fields',
  duration: 5000,
  showIcon: true
});
```

## Builder Pattern

```typescript
import { ToastBuilder } from 'reactive-tui';

// ToastBuilder defaults:
// - variant: ToastVariant.Info
// - position: ToastPosition.TopRight  
// - duration: 5000ms
// - dismissible: true
// - showProgress: false
// - showIcon: true
// - showCloseButton: true

// Complex toast with builder pattern
const complexToast = ToastBuilder.create()
  .message('Upload completed with warnings')
  .warning()
  .topRight()
  .duration(6000)
  .showProgress(true)
  .showCloseButton(true)
  .classes(['upload-toast', 'warning-style'])
  .build();

// Chained builder methods
const chainedToast = ToastBuilder.create()
  .message('Critical system error detected')
  .error()
  .bottomCenter()
  .persistent()
  .dismissible(true)
  .build();

// Using defaults (Info variant, TopRight position, 5000ms duration)
const defaultToast = ToastBuilder.create()
  .message('Using default settings')
  .build();
```

## Convenience Functions

```typescript
import { 
  createToast, 
  createInfoToast, 
  createSuccessToast, 
  createWarningToast, 
  createErrorToast 
} from 'reactive-tui';

// Quick toast creation - these return ToastBuilder instances
const quickInfo = createInfoToast('System initialized').topRight().build();
const quickSuccess = createSuccessToast('Operation complete').bottomRight().build();
const quickWarning = createWarningToast('Low memory').persistent().build();
const quickError = createErrorToast('Connection failed').topCenter().build();

// Generic toast with variant (defaults to Info if not specified)
const genericToast = createToast('Custom message', ToastVariant.Info)
  .duration(3000)
  .showProgress(true)
  .build();

// Default variant is Info
const defaultToast = createToast('Default info message').build();
```

## Advanced Features

### Progress Indication

```typescript
// Toast with progress bar
const progressToast = ToastBuilder.create()
  .message('Uploading files...')
  .info()
  .duration(10000)
  .showProgress(true)
  .showCloseButton(false)
  .build();

// Processing toast with progress
const processingToast = successToast({
  message: 'Processing 50 items',
  duration: 15000,
  showProgress: true,
  dismissible: false
});
```

### Persistent Notifications

```typescript
// Persistent error requiring user action
const persistentError = errorToast({
  message: 'Network connection lost. Check your internet connection.',
  duration: 0,
  dismissible: true,
  showCloseButton: true
});

// Persistent info with manual dismiss
const persistentInfo = ToastBuilder.create()
  .message('System update available. Restart to apply changes.')
  .info()
  .persistent()
  .dismissible(true)
  .topCenter()
  .build();
```

### Custom Styling

```typescript
// Toast with custom CSS classes
const customToast = toast({
  message: 'Custom styled notification',
  variant: ToastVariant.Success,
  classes: ['gradient-bg', 'shadow-lg', 'animate-slide']
});

// Themed toast
const themedToast = ToastBuilder.create()
  .message('Dark theme notification')
  .info()
  .classes(['dark-theme', 'rounded-corners'])
  .build();
```

## Real-World Examples

### Application Status System

```typescript
import { 
  ToastBuilder, 
  ToastVariant,
  ToastPosition,
  createSuccessToast,
  createErrorToast,
  createWarningToast,
  createInfoToast
} from 'reactive-tui';

class ApplicationStatusManager {
  private activeToasts: Map<string, any> = new Map();
  
  // System startup notifications
  showStartupSequence() {
    // Initial loading
    const loading = createInfoToast('Initializing application...')
      .topCenter()
      .duration(2000)
      .showProgress(true)
      .build();
    
    // Configuration loaded
    setTimeout(() => {
      const config = createSuccessToast('Configuration loaded')
        .topRight()
        .duration(1500)
        .build();
    }, 2000);
    
    // Services started
    setTimeout(() => {
      const services = createSuccessToast('All services started')
        .topRight()
        .duration(2000)
        .build();
    }, 3500);
    
    // Ready notification
    setTimeout(() => {
      const ready = ToastBuilder.create()
        .message('Application ready!')
        .success()
        .bottomCenter()
        .duration(3000)
        .showIcon(true)
        .classes(['startup-complete'])
        .build();
    }, 5000);
  }
  
  // Connection status notifications
  handleConnectionStatus(status: 'connected' | 'disconnected' | 'reconnecting') {
    // Clear previous connection toasts
    this.clearToastsByType('connection');
    
    switch (status) {
      case 'connected':
        const connected = createSuccessToast('Connected to server')
          .bottomRight()
          .duration(2000)
          .classes(['connection-toast'])
          .build();
        this.activeToasts.set('connection-success', connected);
        break;
        
      case 'disconnected':
        const disconnected = createErrorToast('Connection lost')
          .topCenter()
          .persistent()
          .dismissible(true)
          .classes(['connection-toast', 'critical'])
          .build();
        this.activeToasts.set('connection-error', disconnected);
        break;
        
      case 'reconnecting':
        const reconnecting = createWarningToast('Attempting to reconnect...')
          .topCenter()
          .duration(0)
          .showProgress(true)
          .dismissible(false)
          .classes(['connection-toast'])
          .build();
        this.activeToasts.set('connection-warning', reconnecting);
        break;
    }
  }
  
  // Operation status notifications
  showOperationStatus(operation: string, status: 'started' | 'progress' | 'completed' | 'failed', details?: string) {
    const operationId = `operation-${operation}`;
    
    switch (status) {
      case 'started':
        const started = createInfoToast(`${operation} started`)
          .bottomLeft()
          .duration(2000)
          .showIcon(true)
          .build();
        this.activeToasts.set(operationId, started);
        break;
        
      case 'progress':
        const progress = ToastBuilder.create()
          .message(`${operation} in progress${details ? `: ${details}` : ''}`)
          .info()
          .bottomLeft()
          .duration(3000)
          .showProgress(true)
          .build();
        this.activeToasts.set(operationId, progress);
        break;
        
      case 'completed':
        this.clearToast(operationId);
        const completed = createSuccessToast(`${operation} completed successfully`)
          .bottomRight()
          .duration(3000)
          .showIcon(true)
          .build();
        this.activeToasts.set(`${operationId}-success`, completed);
        break;
        
      case 'failed':
        this.clearToast(operationId);
        const failed = createErrorToast(`${operation} failed${details ? `: ${details}` : ''}`)
          .topCenter()
          .duration(0)
          .dismissible(true)
          .showCloseButton(true)
          .build();
        this.activeToasts.set(`${operationId}-error`, failed);
        break;
    }
  }
  
  // User action confirmations
  showUserActionFeedback(action: string, success: boolean, message?: string) {
    if (success) {
      const successToast = createSuccessToast(message || `${action} completed`)
        .bottomRight()
        .duration(2000)
        .showIcon(true)
        .classes(['user-action'])
        .build();
    } else {
      const errorToast = createErrorToast(message || `${action} failed`)
        .topCenter()
        .duration(5000)
        .dismissible(true)
        .classes(['user-action', 'error-action'])
        .build();
    }
  }
  
  // System warnings and alerts
  showSystemAlert(level: 'info' | 'warning' | 'critical', message: string, persistent: boolean = false) {
    let alertToast;
    
    switch (level) {
      case 'info':
        alertToast = createInfoToast(message)
          .topLeft()
          .duration(persistent ? 0 : 4000)
          .dismissible(persistent)
          .classes(['system-alert', 'alert-info']);
        break;
        
      case 'warning':
        alertToast = createWarningToast(message)
          .topCenter()
          .duration(persistent ? 0 : 8000)
          .dismissible(true)
          .showCloseButton(true)
          .classes(['system-alert', 'alert-warning']);
        break;
        
      case 'critical':
        alertToast = createErrorToast(message)
          .topCenter()
          .persistent()
          .dismissible(true)
          .showCloseButton(true)
          .classes(['system-alert', 'alert-critical', 'high-priority']);
        break;
    }
    
    const alertId = `system-alert-${level}-${Date.now()}`;
    this.activeToasts.set(alertId, alertToast.build());
  }
  
  // Utility methods
  private clearToast(id: string) {
    this.activeToasts.delete(id);
  }
  
  private clearToastsByType(type: string) {
    const keysToDelete = Array.from(this.activeToasts.keys())
      .filter(key => key.includes(type));
    keysToDelete.forEach(key => this.activeToasts.delete(key));
  }
  
  clearAllToasts() {
    this.activeToasts.clear();
  }
  
  getActiveToasts() {
    return Array.from(this.activeToasts.entries());
  }
}

// Usage
const statusManager = new ApplicationStatusManager();

// Show startup sequence
statusManager.showStartupSequence();

// Handle connection changes
statusManager.handleConnectionStatus('connected');
statusManager.handleConnectionStatus('disconnected');

// Show operation progress
statusManager.showOperationStatus('File Upload', 'started');
statusManager.showOperationStatus('File Upload', 'progress', '50% complete');
statusManager.showOperationStatus('File Upload', 'completed');

// User action feedback
statusManager.showUserActionFeedback('Save Document', true);
statusManager.showUserActionFeedback('Delete File', false, 'Permission denied');

// System alerts
statusManager.showSystemAlert('warning', 'High CPU usage detected');
statusManager.showSystemAlert('critical', 'Disk space critically low', true);
```

### Form Validation Feedback

```typescript
class FormValidationToasts {
  private validationToasts: Map<string, any> = new Map();
  
  // Field validation feedback
  showFieldValidation(fieldName: string, isValid: boolean, message: string) {
    const toastId = `validation-${fieldName}`;
    
    // Clear previous validation toast for this field
    this.clearValidationToast(toastId);
    
    if (!isValid) {
      const validationToast = createErrorToast(`${fieldName}: ${message}`)
        .topRight()
        .duration(4000)
        .dismissible(true)
        .classes(['validation-error', `field-${fieldName.toLowerCase()}`])
        .build();
      
      this.validationToasts.set(toastId, validationToast);
    }
  }
  
  // Form submission feedback
  showSubmissionResult(success: boolean, message: string, details?: string[]) {
    if (success) {
      // Clear all validation errors
      this.clearAllValidationToasts();
      
      const successToast = createSuccessToast(message)
        .bottomCenter()
        .duration(3000)
        .showIcon(true)
        .classes(['form-success'])
        .build();
    } else {
      const errorMessage = message + (details ? `\n• ${details.join('\n• ')}` : '');
      
      const errorToast = createErrorToast(errorMessage)
        .topCenter()
        .duration(0)
        .dismissible(true)
        .showCloseButton(true)
        .classes(['form-error', 'detailed-error'])
        .build();
    }
  }
  
  // Real-time validation feedback
  showRealTimeValidation(field: string, rules: ValidationRule[]) {
    const failures = rules.filter(rule => !rule.isValid);
    
    if (failures.length > 0) {
      const messages = failures.map(rule => rule.message);
      const combinedMessage = `${field}: ${messages.join(', ')}`;
      
      const realtimeToast = ToastBuilder.create()
        .message(combinedMessage)
        .warning()
        .topRight()
        .duration(2000)
        .showIcon(false)
        .classes(['realtime-validation', 'subtle'])
        .build();
      
      this.validationToasts.set(`realtime-${field}`, realtimeToast);
    } else {
      // Clear validation toast when field becomes valid
      this.clearValidationToast(`realtime-${field}`);
    }
  }
  
  // Password strength feedback
  showPasswordStrength(strength: 'weak' | 'medium' | 'strong') {
    const strengthMessages = {
      weak: 'Password is weak',
      medium: 'Password strength is medium',
      strong: 'Password is strong'
    };
    
    const strengthVariants = {
      weak: ToastVariant.Error,
      medium: ToastVariant.Warning,
      strong: ToastVariant.Success
    };
    
    const strengthToast = toast({
      message: strengthMessages[strength],
      variant: strengthVariants[strength],
      position: ToastPosition.BottomLeft,
      duration: 1500,
      showIcon: true,
      classes: ['password-strength', `strength-${strength}`]
    });
    
    this.validationToasts.set('password-strength', strengthToast);
  }
  
  // Required field reminders
  showRequiredFieldReminder(fields: string[]) {
    if (fields.length === 0) return;
    
    const message = fields.length === 1 
      ? `Please fill in the required field: ${fields[0]}`
      : `Please fill in the required fields: ${fields.join(', ')}`;
    
    const reminderToast = createWarningToast(message)
      .topCenter()
      .duration(5000)
      .dismissible(true)
      .classes(['required-fields', 'reminder'])
      .build();
    
    this.validationToasts.set('required-reminder', reminderToast);
  }
  
  private clearValidationToast(id: string) {
    this.validationToasts.delete(id);
  }
  
  private clearAllValidationToasts() {
    this.validationToasts.clear();
  }
  
  getValidationToasts() {
    return Array.from(this.validationToasts.entries());
  }
}

interface ValidationRule {
  isValid: boolean;
  message: string;
}

// Usage
const formToasts = new FormValidationToasts();

// Field validation
formToasts.showFieldValidation('Email', false, 'Invalid email format');
formToasts.showFieldValidation('Password', false, 'Must be at least 8 characters');

// Form submission
formToasts.showSubmissionResult(false, 'Form submission failed', [
  'Email field is required',
  'Password must contain special characters',
  'Terms of service must be accepted'
]);

// Real-time validation
formToasts.showRealTimeValidation('Username', [
  { isValid: false, message: 'Must be at least 3 characters' },
  { isValid: true, message: 'No spaces allowed' }
]);

// Password strength
formToasts.showPasswordStrength('weak');
formToasts.showPasswordStrength('strong');

// Required field reminder
formToasts.showRequiredFieldReminder(['First Name', 'Last Name', 'Email']);
```

### File Operation Notifications

```typescript
class FileOperationToasts {
  private operationToasts: Map<string, any> = new Map();
  
  // File upload progress
  showUploadProgress(fileName: string, progress: number, totalFiles?: number) {
    const operationId = `upload-${fileName}`;
    
    if (progress === 0) {
      // Start upload
      const startToast = createInfoToast(`Starting upload: ${fileName}`)
        .bottomLeft()
        .duration(1000)
        .showIcon(true)
        .build();
    } else if (progress < 100) {
      // Progress update
      const progressMessage = totalFiles 
        ? `Uploading ${fileName} (${progress}%) - ${totalFiles} files remaining`
        : `Uploading ${fileName} (${progress}%)`;
      
      const progressToast = ToastBuilder.create()
        .message(progressMessage)
        .info()
        .bottomLeft()
        .duration(500)
        .showProgress(true)
        .classes(['upload-progress'])
        .build();
      
      this.operationToasts.set(operationId, progressToast);
    } else {
      // Upload complete
      this.operationToasts.delete(operationId);
      
      const completeToast = createSuccessToast(`${fileName} uploaded successfully`)
        .bottomRight()
        .duration(2000)
        .showIcon(true)
        .classes(['upload-complete'])
        .build();
    }
  }
  
  // File download notifications
  showDownloadNotification(fileName: string, status: 'started' | 'completed' | 'failed', error?: string) {
    switch (status) {
      case 'started':
        const startedToast = createInfoToast(`Downloading ${fileName}...`)
          .topRight()
          .duration(2000)
          .showProgress(true)
          .build();
        break;
        
      case 'completed':
        const completedToast = createSuccessToast(`${fileName} downloaded`)
          .bottomRight()
          .duration(3000)
          .showIcon(true)
          .classes(['download-success'])
          .build();
        break;
        
      case 'failed':
        const failedMessage = error ? `Download failed: ${error}` : `Failed to download ${fileName}`;
        const failedToast = createErrorToast(failedMessage)
          .topCenter()
          .duration(5000)
          .dismissible(true)
          .classes(['download-error'])
          .build();
        break;
    }
  }
  
  // File operation batch notifications
  showBatchOperation(operation: string, totalFiles: number, completed: number, failed: number) {
    const remaining = totalFiles - completed - failed;
    
    if (remaining > 0) {
      // In progress
      const progressMessage = `${operation}: ${completed} completed, ${remaining} remaining${failed > 0 ? `, ${failed} failed` : ''}`;
      
      const batchToast = ToastBuilder.create()
        .message(progressMessage)
        .info()
        .bottomCenter()
        .duration(1000)
        .showProgress(true)
        .classes(['batch-operation'])
        .build();
      
      this.operationToasts.set('batch-operation', batchToast);
    } else {
      // Completed
      this.operationToasts.delete('batch-operation');
      
      const completedMessage = failed > 0 
        ? `${operation} completed with ${failed} errors (${completed} successful)`
        : `${operation} completed successfully (${completed} files)`;
      
      const variant = failed > 0 ? ToastVariant.Warning : ToastVariant.Success;
      
      const completedToast = toast({
        message: completedMessage,
        variant,
        position: ToastPosition.BottomCenter,
        duration: 4000,
        showIcon: true,
        classes: ['batch-complete']
      });
    }
  }
  
  // File deletion confirmation
  showDeletionResult(fileName: string, success: boolean, isRecoverable: boolean = true) {
    if (success) {
      const message = isRecoverable 
        ? `${fileName} moved to trash`
        : `${fileName} permanently deleted`;
      
      const deletionToast = createWarningToast(message)
        .bottomRight()
        .duration(3000)
        .showIcon(true)
        .classes(['deletion-toast'])
        .build();
    } else {
      const errorToast = createErrorToast(`Failed to delete ${fileName}`)
        .topCenter()
        .duration(4000)
        .dismissible(true)
        .build();
    }
  }
  
  // File sharing notifications
  showSharingStatus(fileName: string, recipient: string, status: 'sent' | 'delivered' | 'failed') {
    switch (status) {
      case 'sent':
        const sentToast = createInfoToast(`${fileName} sent to ${recipient}`)
          .topRight()
          .duration(2000)
          .build();
        break;
        
      case 'delivered':
        const deliveredToast = createSuccessToast(`${fileName} delivered to ${recipient}`)
          .bottomRight()
          .duration(3000)
          .showIcon(true)
          .build();
        break;
        
      case 'failed':
        const failedToast = createErrorToast(`Failed to send ${fileName} to ${recipient}`)
          .topCenter()
          .duration(5000)
          .dismissible(true)
          .build();
        break;
    }
  }
  
  // Storage quota warnings
  showStorageWarning(usedPercent: number, totalSpace: string) {
    let variant: ToastVariant;
    let message: string;
    let persistent = false;
    
    if (usedPercent >= 95) {
      variant = ToastVariant.Error;
      message = `Storage almost full (${usedPercent}% of ${totalSpace}). Delete files to continue.`;
      persistent = true;
    } else if (usedPercent >= 85) {
      variant = ToastVariant.Warning;
      message = `Storage running low (${usedPercent}% of ${totalSpace}). Consider cleaning up files.`;
    } else if (usedPercent >= 75) {
      variant = ToastVariant.Info;
      message = `Storage usage: ${usedPercent}% of ${totalSpace}`;
    } else {
      return; // No warning needed
    }
    
    const warningToast = toast({
      message,
      variant,
      position: ToastPosition.TopLeft,
      duration: persistent ? 0 : 6000,
      dismissible: true,
      showCloseButton: persistent,
      classes: ['storage-warning', `usage-${Math.floor(usedPercent / 10) * 10}`]
    });
    
    this.operationToasts.set('storage-warning', warningToast);
  }
  
  clearOperationToasts() {
    this.operationToasts.clear();
  }
  
  getActiveOperations() {
    return Array.from(this.operationToasts.entries());
  }
}

// Usage
const fileToasts = new FileOperationToasts();

// Upload progress
fileToasts.showUploadProgress('document.pdf', 0);
fileToasts.showUploadProgress('document.pdf', 50);
fileToasts.showUploadProgress('document.pdf', 100);

// Download notifications
fileToasts.showDownloadNotification('video.mp4', 'started');
fileToasts.showDownloadNotification('video.mp4', 'completed');

// Batch operations
fileToasts.showBatchOperation('File Compression', 10, 7, 1); // 2 remaining
fileToasts.showBatchOperation('File Compression', 10, 9, 1); // Complete

// Deletion
fileToasts.showDeletionResult('old-file.txt', true, true);

// Sharing
fileToasts.showSharingStatus('report.pdf', 'john@example.com', 'delivered');

// Storage warnings
fileToasts.showStorageWarning(87, '100 GB');
fileToasts.showStorageWarning(96, '100 GB');
```

## Styling

### CSS Classes

```css
/* Base toast styles */
.toast {
  position: fixed;
  padding: 12px 16px;
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  display: flex;
  align-items: center;
  gap: 8px;
  max-width: 400px;
  z-index: 1000;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 14px;
  line-height: 1.4;
}

/* Toast variants */
.toast-info {
  background: #3b82f6;
  color: white;
  border-left: 4px solid #1d4ed8;
}

.toast-success {
  background: #10b981;
  color: white;
  border-left: 4px solid #059669;
}

.toast-warning {
  background: #f59e0b;
  color: white;
  border-left: 4px solid #d97706;
}

.toast-error {
  background: #ef4444;
  color: white;
  border-left: 4px solid #dc2626;
}

/* Toast positioning */
.toast-top-left {
  top: 20px;
  left: 20px;
}

.toast-top-center {
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
}

.toast-top-right {
  top: 20px;
  right: 20px;
}

.toast-bottom-left {
  bottom: 20px;
  left: 20px;
}

.toast-bottom-center {
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
}

.toast-bottom-right {
  bottom: 20px;
  right: 20px;
}

/* Progress bar */
.toast-progress::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  height: 3px;
  background: rgba(255, 255, 255, 0.8);
  animation: toast-progress var(--duration, 5s) linear forwards;
}

@keyframes toast-progress {
  from { width: 100%; }
  to { width: 0%; }
}

/* Interactive elements */
.toast-dismissible {
  cursor: pointer;
}

.toast-close-button {
  margin-left: auto;
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 4px;
  border-radius: 2px;
  opacity: 0.8;
}

.toast-close-button:hover {
  opacity: 1;
  background: rgba(255, 255, 255, 0.2);
}

/* Icons */
.toast-icon {
  flex-shrink: 0;
  width: 20px;
  height: 20px;
}

/* Animations */
.toast {
  animation: toast-slide-in 0.3s ease-out;
}

@keyframes toast-slide-in {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.toast.dismissing {
  animation: toast-slide-out 0.2s ease-in forwards;
}

@keyframes toast-slide-out {
  from {
    opacity: 1;
    transform: translateY(0);
  }
  to {
    opacity: 0;
    transform: translateY(-20px);
  }
}

/* Responsive design */
@media (max-width: 480px) {
  .toast {
    left: 10px !important;
    right: 10px !important;
    max-width: none;
    transform: none !important;
  }
  
  .toast-top-center,
  .toast-bottom-center {
    left: 10px;
  }
}

/* Dark theme */
.theme-dark .toast-info {
  background: #1e3a8a;
  border-left-color: #3b82f6;
}

.theme-dark .toast-success {
  background: #064e3b;
  border-left-color: #10b981;
}

.theme-dark .toast-warning {
  background: #92400e;
  border-left-color: #f59e0b;
}

.theme-dark .toast-error {
  background: #991b1b;
  border-left-color: #ef4444;
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .toast {
    border: 2px solid currentColor;
    box-shadow: none;
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .toast {
    animation: none;
  }
  
  .toast-progress::after {
    animation: none;
  }
}

/* Custom toast styles */
.startup-complete {
  background: linear-gradient(135deg, #10b981, #059669);
  font-weight: 600;
}

.connection-toast.critical {
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.8; }
}

.validation-error {
  font-size: 13px;
  background: #fef2f2;
  color: #991b1b;
  border: 1px solid #fecaca;
}

.upload-progress {
  background: #eff6ff;
  color: #1e40af;
  border-left: 4px solid #3b82f6;
  font-family: monospace;
}

.batch-operation {
  font-family: monospace;
  font-size: 12px;
}

.system-alert.high-priority {
  border: 2px solid #dc2626;
  box-shadow: 0 0 20px rgba(220, 38, 38, 0.3);
}
```

## Accessibility

The Toast widget includes comprehensive accessibility support based on the actual implementation:

```typescript
// ARIA attributes automatically applied by the toast() function:
builder.attr('role', 'alert');
builder.attr('aria-live', config.variant === ToastVariant.Error ? 'assertive' : 'polite');

// aria-label only added if dismissible !== false
if (config.dismissible !== false) {
  builder.attr('aria-label', `${config.variant} notification: ${config.message}. Dismissible.`);
}

// Additional data attributes for Rust backend:
builder.attr('data-variant', config.variant);
builder.attr('data-position', config.position || ToastPosition.TopRight);
builder.attr('message', config.message);
if (config.duration !== undefined) builder.attr('duration', config.duration.toString());
if (config.dismissible !== undefined) builder.attr('dismissible', config.dismissible.toString());
if (config.showProgress) builder.attr('show-progress', 'true');
if (config.showIcon !== false) builder.attr('show-icon', 'true');
if (config.showCloseButton !== false) builder.attr('show-close-button', 'true');

// Screen reader considerations
const accessibleToast = toast({
  message: 'File uploaded successfully',
  variant: ToastVariant.Success,
  // Error toasts use 'assertive' aria-live for immediate attention
  // Other variants use 'polite' to avoid interrupting user flow
});
```

## Important Implementation Details

Based on the actual source code:

1. **Default Behavior**:
   - `dismissible` defaults to `true` when not specified (`config.dismissible !== false`)
   - `showIcon` defaults to `true` when not specified (`config.showIcon !== false`) 
   - `showCloseButton` defaults to `true` when not specified (`config.showCloseButton !== false`)
   - Default position is `ToastPosition.TopRight`

2. **CSS Classes Applied**:
   - Always: `'toast'`, `'toast-${variant}'`, `'toast-${position}'`
   - Conditionally: `'toast-dismissible'` (if dismissible !== false), `'toast-progress'` (if showProgress)

3. **Validation**:
   - ToastBuilder throws error if no message is provided: `'Toast requires a message'`

## Best Practices

1. **Choose Appropriate Variants**
   - Use `Info` for general notifications
   - Use `Success` for confirmations
   - Use `Warning` for non-critical alerts
   - Use `Error` for critical issues

2. **Duration Guidelines**
   - Info: 3-4 seconds
   - Success: 2-3 seconds
   - Warning: 5-8 seconds
   - Error: Persistent (0) or 8+ seconds

3. **Positioning Strategy**
   - Use `TopRight` (default) for general notifications
   - Use `TopCenter` for important alerts
   - Use `BottomRight` for confirmations
   - Use `BottomCenter` for progress updates

4. **Accessibility Considerations**
   - Provide meaningful messages
   - Use appropriate ARIA attributes
   - Consider screen reader users
   - Ensure sufficient contrast

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const container = new ElementBuilderImpl('div')
  .class('notification-container')
  .child(
    toast({
      message: 'Welcome to the application!',
      variant: ToastVariant.Info,
      position: ToastPosition.TopCenter,
      duration: 4000
    })
  )
  .build();
```

The Toast widget provides comprehensive notification capabilities with variant-based styling, flexible positioning, and accessibility support for effective user communication in terminal applications.