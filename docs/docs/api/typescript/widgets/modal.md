---
sidebar_position: 6
---

# Modal Widget

Overlay dialogs with backdrop and focus trapping, supporting multiple modal types including alert, confirm, prompt, and custom modals.

## Overview

The Modal widget provides comprehensive dialog functionality with keyboard navigation, focus management, and multiple predefined modal types for common use cases.

```typescript
import { modal, alertModal, confirmModal, promptModal, customModal } from 'reactive-tui-ts'

const basicModal = modal({
  id: 'basic-modal',
  title: 'Basic Dialog',
  content: 'This is a basic modal dialog.',
  modalType: 'basic',
  position: 'center',
  size: 'medium',
  closeable: true
})
```

## Types

### ModalType

```typescript
export type ModalType = 'basic' | 'alert' | 'confirm' | 'prompt' | 'custom'
```

### ModalPosition

```typescript
export type ModalPosition = 'center' | 'top-center' | 'bottom-center' | 'left-center' | 'right-center' |
                           'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'custom'
```

### ModalSize

```typescript
export type ModalSize = 'small' | 'medium' | 'large' | 'extra-large' | 'full-screen' | 'custom'
```

## Configuration

### ModalConfig

```typescript
interface ModalConfig {
  id: string
  title?: string
  content?: string
  modalType?: ModalType
  position?: ModalPosition
  size?: ModalSize
  backdrop?: ModalBackdrop
  style?: ModalStyle
  cssClasses?: string[]
  closeable?: boolean
  animate?: boolean
  customPosition?: { x: number, y: number }
  customSize?: { width: number, height: number }
  // Type-specific configurations
  alertMessage?: string
  confirmMessage?: string
  confirmYesLabel?: string
  confirmNoLabel?: string
  promptMessage?: string
  promptPlaceholder?: string
  promptDefaultValue?: string
  customButtons?: ModalButton[]
}
```

### ModalButton

```typescript
interface ModalButton {
  id: string
  label: string
  variant: string
  closesModal: boolean
  isDefault: boolean
  action?: () => void
}
```

### ModalBackdrop

```typescript
interface ModalBackdrop {
  visible: boolean
  clickToClose: boolean
  color?: string
  opacity: number
  character: string
}
```

### ModalStyle

```typescript
interface ModalStyle {
  background?: string
  textColor?: string
  borderColor?: string
  borderChar: string
  titleColor?: string
  buttonColors: Record<string, string>
  padding: number
  shadow: boolean
  rounded: boolean
}
```

## Basic Usage

### Basic Modal

```typescript
import { modal } from 'reactive-tui-ts'

const basicModal = modal({
  id: 'basic-modal',
  title: 'Basic Dialog',
  content: 'This is a basic modal dialog with simple content.',
  modalType: 'basic',
  position: 'center',
  size: 'medium',
  closeable: true,
  animate: true
})

// Open the modal
basicModal.open()

// Check if modal is open
if (basicModal.isOpen()) {
  console.log('Modal is currently open')
}

// Close the modal
basicModal.close()
```

### Modal with Custom Styling

```typescript
const styledModal = modal({
  id: 'styled-modal',
  title: 'Custom Styled Modal',
  content: 'This modal has custom styling.',
  modalType: 'basic',
  size: 'large',
  style: {
    background: '#f8f9fa',
    textColor: '#212529',
    borderColor: '#6c757d',
    borderChar: '┌',
    titleColor: '#007bff',
    buttonColors: {
      primary: '#007bff',
      danger: '#dc3545'
    },
    padding: 3,
    shadow: true,
    rounded: true
  },
  backdrop: {
    visible: true,
    clickToClose: true,
    color: '#000000',
    opacity: 0.7,
    character: '█'
  }
})
```

## Modal Types

### Alert Modal

```typescript
import { alertModal } from 'reactive-tui-ts'

const alert = alertModal('alert-id', 'Success', 'Operation completed successfully!')

// Show the alert
alert.open()

// Wait for user interaction and get result
setTimeout(() => {
  const result = alert.getResult()
  if (result === 'ok') {
    console.log('User acknowledged the alert')
  }
}, 1000)
```

### Confirm Modal

```typescript
import { confirmModal } from 'reactive-tui-ts'

const confirm = confirmModal(
  'confirm-delete',
  'Delete File',
  'Are you sure you want to delete this file? This action cannot be undone.',
  'Delete',
  'Cancel'
)

// Show confirmation dialog
confirm.open()

// Handle result
setTimeout(() => {
  const result = confirm.getResult()
  if (result === 'yes') {
    console.log('User confirmed deletion')
    deleteFile()
  } else if (result === 'no') {
    console.log('User cancelled deletion')
  }
}, 2000)
```

### Prompt Modal

```typescript
import { promptModal } from 'reactive-tui-ts'

const prompt = promptModal(
  'name-prompt',
  'Enter Name',
  'Please enter your full name:',
  'John Doe',
  ''
)

// Show prompt dialog
prompt.open()

// Handle result with input value
setTimeout(() => {
  const result = prompt.getResult()
  if (typeof result === 'object' && result.action === 'ok') {
    console.log('User entered:', result.value)
    processUserInput(result.value)
  } else {
    console.log('User cancelled prompt')
  }
}, 3000)
```

### Custom Modal with Multiple Buttons

```typescript
import { customModal, createPrimaryButton, createSecondaryButton, createDangerButton } from 'reactive-tui-ts'

const saveDialog = customModal(
  'save-dialog',
  'Unsaved Changes',
  'You have unsaved changes. What would you like to do?',
  [
    createSecondaryButton('cancel', 'Cancel', false), // Don't close modal
    createDangerButton('discard', 'Discard Changes'),
    createPrimaryButton('save', 'Save Changes')
  ]
)

// Add custom actions to buttons
saveDialog.customButtons?.forEach(button => {
  if (button.id === 'save') {
    button.action = () => {
      console.log('Saving changes...')
      saveDocument()
    }
  } else if (button.id === 'discard') {
    button.action = () => {
      console.log('Discarding changes...')
      discardChanges()
    }
  } else if (button.id === 'cancel') {
    button.action = () => {
      console.log('Cancelled')
      saveDialog.close()
    }
  }
})

// Show the dialog
saveDialog.open()
```

### Modal Button Helpers

```typescript
import { createModalButton, createPrimaryButton, createSecondaryButton, createDangerButton } from 'reactive-tui-ts'

// Generic button
const genericButton = createModalButton('generic', 'Generic Button', {
  variant: 'outlined',
  closesModal: true,
  isDefault: false
})

// Primary button (default focus)
const primaryButton = createPrimaryButton('primary', 'Primary Action', true)

// Secondary button
const secondaryButton = createSecondaryButton('secondary', 'Secondary Action', true)

// Danger button
const dangerButton = createDangerButton('danger', 'Dangerous Action', true)
```

## Modal Builder Pattern

### Using ModalBuilder

```typescript
import { modalBuilder } from 'reactive-tui-ts'

const builderModal = modalBuilder('builder-modal', 'basic')
  .title('Builder Pattern Modal')
  .content('This modal was created using the builder pattern.')
  .size('large')
  .position('center')
  .backdrop({
    visible: true,
    clickToClose: false,
    opacity: 0.8,
    character: '█'
  })
  .style({
    background: '#ffffff',
    textColor: '#333333',
    borderColor: '#007bff',
    padding: 2,
    shadow: true,
    rounded: true
  })
  .cssClass('custom-modal')
  .build()

// Chaining methods for different configurations
const customPositionModal = modalBuilder('custom-pos', 'basic')
  .title('Custom Position')
  .content('This modal is positioned at custom coordinates.')
  .customPosition(100, 50)
  .customSize(400, 200)
  .noAnimation()
  .notCloseable()
  .build()
```

### Full-Screen Modal

```typescript
import { fullscreenModal } from 'reactive-tui-ts'

const fullscreen = fullscreenModal(
  'fullscreen-modal',
  'This is a full-screen modal that covers the entire terminal.'
)

// Full-screen modals have no backdrop and cover the entire screen
fullscreen.open()
```

## Modal Positioning and Sizing

### Positioning Options

```typescript
// Centered modal (default)
const centeredModal = modal({
  id: 'centered',
  title: 'Centered Modal',
  content: 'This modal is centered on screen.',
  position: 'center'
})

// Top-center position
const topModal = modal({
  id: 'top-modal',
  title: 'Top Modal',
  content: 'This appears at the top center.',
  position: 'top-center',
  size: 'small'
})

// Corner positions
const corners = [
  { id: 'top-left', position: 'top-left' as ModalPosition },
  { id: 'top-right', position: 'top-right' as ModalPosition },
  { id: 'bottom-left', position: 'bottom-left' as ModalPosition },
  { id: 'bottom-right', position: 'bottom-right' as ModalPosition }
]

corners.forEach(config => {
  const modal = modal({
    id: config.id,
    title: `${config.position} Modal`,
    content: `This modal is positioned at ${config.position}.`,
    position: config.position,
    size: 'small'
  })
})

// Custom position
const customModal = modal({
  id: 'custom-position',
  title: 'Custom Position',
  content: 'This modal is at custom coordinates.',
  position: 'custom',
  customPosition: { x: 150, y: 75 },
  size: 'medium'
})
```

### Size Options

```typescript
// Different size options
const sizes: { size: ModalSize, title: string }[] = [
  { size: 'small', title: 'Small Modal (40x15)' },
  { size: 'medium', title: 'Medium Modal (60x20)' },
  { size: 'large', title: 'Large Modal (80x30)' },
  { size: 'extra-large', title: 'Extra Large Modal (100x40)' },
  { size: 'full-screen', title: 'Full Screen Modal' }
]

sizes.forEach(config => {
  const sizedModal = modal({
    id: `${config.size}-modal`,
    title: config.title,
    content: `This is a ${config.size} modal.`,
    size: config.size
  })
})

// Custom size
const customSizeModal = modal({
  id: 'custom-size',
  title: 'Custom Size Modal',
  content: 'This modal has custom dimensions.',
  size: 'custom',
  customSize: { width: 70, height: 25 }
})
```

## Keyboard Navigation and Focus Management

### Focus Handling

```typescript
const focusModal = modal({
  id: 'focus-modal',
  title: 'Focus Management',
  content: 'Use Tab/Shift+Tab to navigate, Enter to activate, Escape to close.',
  modalType: 'custom',
  customButtons: [
    createSecondaryButton('cancel', 'Cancel'),
    createPrimaryButton('ok', 'OK') // This will be default focused
  ]
})

// Open and manage focus
focusModal.open()

// Get currently focused element
const focusedElement = focusModal.getFocusedElement()
console.log('Currently focused:', focusedElement)

// Set focus to specific element
focusModal.setFocusedElement('ok')

// Handle keyboard events manually if needed
const handleKeyboard = (key: string) => {
  const handled = focusModal.handleKey(key)
  if (handled) {
    console.log(`Modal handled key: ${key}`)
  }
  return handled
}
```

### Keyboard Shortcuts

```typescript
// The modal automatically handles these keys:
// - Tab: Move focus to next element
// - Shift+Tab: Move focus to previous element
// - Enter: Activate focused element
// - Escape: Close modal (if closeable)
// - Backspace: Delete character in prompt input
// - Any character: Type in prompt input (when focused)

const keyboardModal = modal({
  id: 'keyboard-demo',
  title: 'Keyboard Navigation Demo',
  content: 'Try using keyboard navigation!',
  modalType: 'confirm',
  confirmMessage: 'Use Tab to navigate, Enter to select.',
  confirmYesLabel: 'Accept',
  confirmNoLabel: 'Decline',
  closeable: true
})

keyboardModal.open()
```

## Advanced Features

### Layout Calculation

```typescript
// The modal automatically calculates layout based on screen size
const responsiveModal = modal({
  id: 'responsive',
  title: 'Responsive Modal',
  content: 'This modal adapts to screen size.',
  size: 'large'
})

// Get calculated layout for specific screen size
const layout = responsiveModal.calculateLayout(120, 40) // 120x40 terminal
console.log('Modal will be positioned at:', layout)
// Output: { x: 20, y: 5, width: 80, height: 30 }

// Layout respects screen boundaries and adjusts automatically
const smallScreenLayout = responsiveModal.calculateLayout(60, 20) // Small terminal
console.log('Small screen layout:', smallScreenLayout)
// Output: { x: 0, y: 0, width: 60, height: 20 } // Fills available space
```

### Rendering System

```typescript
// The modal provides advanced rendering capabilities
const renderingModal = modal({
  id: 'rendering-demo',
  title: 'Advanced Rendering',
  content: 'This demonstrates the rendering system.',
  modalType: 'basic',
  animate: true
})

// Render the modal for specific screen dimensions
const rendered = renderingModal.render(80, 24)
if (rendered.isOpen) {
  console.log('Backdrop:', rendered.backdrop.length, 'characters')
  console.log('Modal content:', rendered.modal.split('\n').length, 'lines')
  console.log('Position:', rendered.position)
  console.log('Focused element:', rendered.focusedElement)
}
```

### Dynamic Configuration Updates

```typescript
const dynamicModal = modal({
  id: 'dynamic',
  title: 'Dynamic Modal',
  content: 'This modal can be reconfigured.',
  modalType: 'basic',
  size: 'medium',
  position: 'center'
})

// Update modal configuration dynamically
dynamicModal.updateConfig({
  size: 'large',
  position: 'top-center',
  animate: false,
  closeable: false
})

// Configuration changes are applied immediately
console.log('Updated modal configuration')
```

## Modal Examples Collection

### Creating Modal Examples

```typescript
import { createModalExamples } from 'reactive-tui-ts'

// Get a collection of pre-configured modal examples
const examples = createModalExamples()

// Simple alert example
examples.alert.open()
console.log('Alert result:', examples.alert.getResult())

// Confirmation dialog example
examples.confirm.open()
setTimeout(() => {
  const result = examples.confirm.getResult()
  if (result === 'yes') {
    console.log('User confirmed deletion')
  }
}, 2000)

// Prompt dialog example
examples.prompt.open()
setTimeout(() => {
  const result = examples.prompt.getResult()
  if (typeof result === 'object') {
    console.log('User entered:', result.value)
  }
}, 3000)

// Custom save dialog example
examples.custom.open()

// Settings modal example
examples.settings.open()
```

### Modal State Management

```typescript
class ModalStateManager {
  private modals = new Map<string, any>()
  private currentModal: string | null = null

  registerModal(id: string, modalConfig: any) {
    const modalInstance = modal(modalConfig)
    this.modals.set(id, modalInstance)
    return modalInstance
  }

  showModal(id: string): boolean {
    const modalInstance = this.modals.get(id)
    if (modalInstance && !modalInstance.isOpen()) {
      // Close current modal if open
      if (this.currentModal) {
        this.hideModal(this.currentModal)
      }
      
      modalInstance.open()
      this.currentModal = id
      return true
    }
    return false
  }

  hideModal(id: string): boolean {
    const modalInstance = this.modals.get(id)
    if (modalInstance && modalInstance.isOpen()) {
      modalInstance.close()
      if (this.currentModal === id) {
        this.currentModal = null
      }
      return true
    }
    return false
  }

  getCurrentModal(): string | null {
    return this.currentModal
  }

  getModalResult(id: string): any {
    const modalInstance = this.modals.get(id)
    return modalInstance ? modalInstance.getResult() : null
  }

  isModalOpen(id: string): boolean {
    const modalInstance = this.modals.get(id)
    return modalInstance ? modalInstance.isOpen() : false
  }
}

// Usage example
const modalManager = new ModalStateManager()

// Register modals
modalManager.registerModal('welcome', {
  id: 'welcome',
  title: 'Welcome',
  content: 'Welcome to the application!',
  modalType: 'alert'
})

modalManager.registerModal('confirm-exit', {
  id: 'confirm-exit',
  title: 'Exit Application',
  modalType: 'confirm',
  confirmMessage: 'Are you sure you want to exit?',
  confirmYesLabel: 'Exit',
  confirmNoLabel: 'Stay'
})

// Show modals
modalManager.showModal('welcome')

setTimeout(() => {
  modalManager.hideModal('welcome')
  modalManager.showModal('confirm-exit')
}, 3000)
```

## Complete Application Example

```typescript
import { modal, alertModal, confirmModal, promptModal, customModal, createPrimaryButton, createSecondaryButton } from 'reactive-tui-ts'

class TaskManager {
  private tasks: string[] = []
  private modals = new Map<string, any>()

  constructor() {
    this.setupModals()
  }

  private setupModals() {
    // Welcome modal
    this.modals.set('welcome', alertModal(
      'welcome',
      'Welcome to Task Manager',
      'Manage your tasks efficiently with this simple task manager.'
    ))

    // Add task prompt
    this.modals.set('add-task', promptModal(
      'add-task',
      'Add New Task',
      'Enter a description for the new task:',
      'Enter task description...',
      ''
    ))

    // Delete confirmation
    this.modals.set('confirm-delete', confirmModal(
      'confirm-delete',
      'Delete Task',
      'Are you sure you want to delete this task?',
      'Delete',
      'Cancel'
    ))

    // Settings modal
    this.modals.set('settings', modal({
      id: 'settings',
      title: 'Settings',
      content: 'Configure your task manager preferences.',
      modalType: 'custom',
      size: 'large',
      customButtons: [
        createSecondaryButton('reset', 'Reset to Defaults'),
        createSecondaryButton('cancel', 'Cancel'),
        createPrimaryButton('save', 'Save Settings')
      ]
    }))

    // About modal
    this.modals.set('about', modal({
      id: 'about',
      title: 'About Task Manager',
      content: `Task Manager v1.0.0\n\nA simple task management application built with Reactive TUI.\n\nFeatures:\n• Add and delete tasks\n• Simple and clean interface\n• Keyboard navigation`,
      modalType: 'basic',
      size: 'medium'
    }))
  }

  public async start() {
    // Show welcome modal
    const welcomeModal = this.modals.get('welcome')
    welcomeModal.open()
    
    // Wait for user to acknowledge
    await this.waitForModalResult('welcome')
    
    console.log('Task Manager started!')
    this.showMainMenu()
  }

  private showMainMenu() {
    console.log('\n=== Task Manager ===')
    console.log('1. Add Task')
    console.log('2. Delete Task')
    console.log('3. Settings')
    console.log('4. About')
    console.log('5. Exit')
    
    // In a real application, you would handle user input here
    // For demo purposes, we'll show different modal interactions
  }

  public async addTask() {
    const promptModal = this.modals.get('add-task')
    promptModal.open()
    
    const result = await this.waitForModalResult('add-task')
    
    if (typeof result === 'object' && result.action === 'ok' && result.value.trim()) {
      this.tasks.push(result.value.trim())
      console.log(`Task added: "${result.value.trim()}"`)
      
      // Show success message
      const successModal = alertModal(
        'task-added',
        'Success',
        'Task has been added successfully!'
      )
      successModal.open()
      await this.waitForModalResult('task-added')
    } else {
      console.log('Task addition cancelled or empty')
    }
  }

  public async deleteTask(taskIndex: number) {
    if (taskIndex < 0 || taskIndex >= this.tasks.length) {
      const errorModal = alertModal(
        'error',
        'Error',
        'Invalid task selection.'
      )
      errorModal.open()
      await this.waitForModalResult('error')
      return
    }

    const confirmModal = this.modals.get('confirm-delete')
    confirmModal.open()
    
    const result = await this.waitForModalResult('confirm-delete')
    
    if (result === 'yes') {
      const deletedTask = this.tasks.splice(taskIndex, 1)[0]
      console.log(`Task deleted: "${deletedTask}"`)
      
      // Show deletion confirmation
      const deletedModal = alertModal(
        'task-deleted',
        'Deleted',
        'Task has been deleted.'
      )
      deletedModal.open()
      await this.waitForModalResult('task-deleted')
    } else {
      console.log('Task deletion cancelled')
    }
  }

  public async showSettings() {
    const settingsModal = this.modals.get('settings')
    
    // Add button actions
    settingsModal.customButtons.forEach((button: any) => {
      if (button.id === 'reset') {
        button.action = () => {
          console.log('Settings reset to defaults')
          settingsModal.closeWithResult('reset')
        }
      } else if (button.id === 'save') {
        button.action = () => {
          console.log('Settings saved')
          settingsModal.closeWithResult('save')
        }
      } else if (button.id === 'cancel') {
        button.action = () => {
          settingsModal.closeWithResult('cancel')
        }
      }
    })

    settingsModal.open()
    const result = await this.waitForModalResult('settings')
    
    if (result === 'save') {
      const savedModal = alertModal(
        'settings-saved',
        'Settings Saved',
        'Your settings have been saved successfully.'
      )
      savedModal.open()
      await this.waitForModalResult('settings-saved')
    }
  }

  public async showAbout() {
    const aboutModal = this.modals.get('about')
    aboutModal.open()
    await this.waitForModalResult('about')
  }

  public async confirmExit(): Promise<boolean> {
    const exitModal = confirmModal(
      'confirm-exit',
      'Exit Application',
      'Are you sure you want to exit Task Manager?',
      'Exit',
      'Stay'
    )
    
    exitModal.open()
    const result = await this.waitForModalResult('confirm-exit')
    
    return result === 'yes'
  }

  private waitForModalResult(modalId: string): Promise<any> {
    return new Promise((resolve) => {
      const modal = this.modals.get(modalId) || 
                   (modalId.includes('-') ? this.findModalById(modalId) : null)
      
      if (!modal) {
        resolve(null)
        return
      }

      const checkResult = () => {
        if (!modal.isOpen()) {
          const result = modal.getResult()
          resolve(result)
        } else {
          setTimeout(checkResult, 100)
        }
      }
      
      checkResult()
    })
  }

  private findModalById(id: string): any {
    // Helper to find modals that might not be in the main map
    // (like dynamically created ones)
    return null
  }

  public getTasks(): string[] {
    return [...this.tasks]
  }

  public getTaskCount(): number {
    return this.tasks.length
  }
}

// Usage example
const taskManager = new TaskManager()

// Start the application
const runTaskManager = async () => {
  await taskManager.start()
  
  // Simulate user interactions
  console.log('\nDemo: Adding a task')
  await taskManager.addTask()
  
  console.log('\nDemo: Showing settings')
  await taskManager.showSettings()
  
  console.log('\nDemo: Showing about')
  await taskManager.showAbout()
  
  console.log('\nDemo: Confirming exit')
  const shouldExit = await taskManager.confirmExit()
  
  if (shouldExit) {
    console.log('Application exited by user')
  } else {
    console.log('User chose to stay')
  }
  
  console.log(`\nFinal task count: ${taskManager.getTaskCount()}`)
}

// Start the demo
runTaskManager().catch(console.error)
```

## CSS Styling

```css
/* Modal base styles */
.modal {
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  border: 2px solid #6c757d;
  background: #ffffff;
  color: #212529;
  position: fixed;
  z-index: 1000;
}

/* Modal positioning */
.modal-center {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.modal-top-center {
  top: 25%;
  left: 50%;
  transform: translate(-50%, 0);
}

.modal-bottom-center {
  bottom: 25%;
  left: 50%;
  transform: translate(-50%, 0);
}

.modal-top-left {
  top: 10%;
  left: 10%;
}

.modal-top-right {
  top: 10%;
  right: 10%;
}

.modal-bottom-left {
  bottom: 10%;
  left: 10%;
}

.modal-bottom-right {
  bottom: 10%;
  right: 10%;
}

/* Modal sizes */
.modal-small {
  width: 40ch;
  height: 15rem;
}

.modal-medium {
  width: 60ch;
  height: 20rem;
}

.modal-large {
  width: 80ch;
  height: 30rem;
}

.modal-extra-large {
  width: 100ch;
  height: 40rem;
}

.modal-full-screen {
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  transform: none;
}

/* Modal types */
.modal-basic {
  border-color: #6c757d;
}

.modal-alert {
  border-color: #ffc107;
  background: #fff3cd;
}

.modal-confirm {
  border-color: #dc3545;
  background: #f8d7da;
}

.modal-prompt {
  border-color: #17a2b8;
  background: #d1ecf1;
}

.modal-custom {
  border-color: #6f42c1;
  background: #e2d9f3;
}

/* Modal content areas */
.modal-header {
  padding: 1rem;
  border-bottom: 1px solid #dee2e6;
  font-weight: bold;
}

.modal-body {
  padding: 1rem;
  flex-grow: 1;
  overflow-y: auto;
}

.modal-footer {
  padding: 1rem;
  border-top: 1px solid #dee2e6;
  display: flex;
  justify-content: center;
  gap: 1rem;
}

/* Backdrop */
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
}

.modal-backdrop.modal-backdrop-dark {
  background: rgba(0, 0, 0, 0.8);
}

.modal-backdrop.modal-backdrop-light {
  background: rgba(255, 255, 255, 0.8);
}

/* Modal buttons */
.modal-button {
  padding: 0.5rem 1rem;
  border: 1px solid #6c757d;
  background: #ffffff;
  color: #212529;
  cursor: pointer;
  font-family: inherit;
}

.modal-button:hover {
  background: #e9ecef;
}

.modal-button:focus,
.modal-button.focused {
  outline: 2px solid #007bff;
  outline-offset: 2px;
}

.modal-button.button-primary {
  background: #007bff;
  color: white;
  border-color: #007bff;
}

.modal-button.button-primary:hover {
  background: #0056b3;
}

.modal-button.button-secondary {
  background: #6c757d;
  color: white;
  border-color: #6c757d;
}

.modal-button.button-secondary:hover {
  background: #545b62;
}

.modal-button.button-danger {
  background: #dc3545;
  color: white;
  border-color: #dc3545;
}

.modal-button.button-danger:hover {
  background: #c82333;
}

/* Prompt input */
.modal-prompt-input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ced4da;
  background: #ffffff;
  color: #212529;
  font-family: inherit;
}

.modal-prompt-input:focus {
  outline: 2px solid #007bff;
  outline-offset: 1px;
}

.modal-prompt-input::placeholder {
  color: #6c757d;
  font-style: italic;
}

/* Animation states */
.modal-animated {
  transition: all 0.3s ease;
}

.modal-hidden {
  opacity: 0;
  transform: scale(0.8);
  pointer-events: none;
}

.modal-animated:not(.modal-hidden) {
  opacity: 1;
  transform: scale(1);
}

/* Focus management */
.modal-focus-trap {
  position: relative;
}

.modal-focus-trap:focus-within {
  outline: none;
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .modal-animated {
    transition: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .modal {
    border-width: 3px;
  }
  
  .modal-button:focus {
    outline-width: 3px;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .modal {
    background: #2d3748;
    color: #e2e8f0;
    border-color: #4a5568;
  }
  
  .modal-button {
    background: #4a5568;
    color: #e2e8f0;
    border-color: #4a5568;
  }
  
  .modal-button:hover {
    background: #2d3748;
  }
  
  .modal-prompt-input {
    background: #4a5568;
    color: #e2e8f0;
    border-color: #2d3748;
  }
}
```

## Best Practices

### 1. Use Appropriate Modal Types

```typescript
// ✅ Good - use specific modal types for common scenarios
const deleteConfirm = confirmModal(
  'delete-confirm',
  'Delete Item',
  'This action cannot be undone. Are you sure?',
  'Delete',
  'Cancel'
)

const userInfo = alertModal(
  'info',
  'Information',
  'Here is some important information for the user.'
)

const getUserInput = promptModal(
  'user-input',
  'Enter Value',
  'Please enter the required value:',
  'Placeholder text...',
  'default value'
)
```

### 2. Provide Clear Visual Hierarchy

```typescript
// ✅ Good - clear button hierarchy with primary actions
const saveModal = customModal(
  'save-changes',
  'Save Changes',
  'You have unsaved changes. What would you like to do?',
  [
    createSecondaryButton('cancel', 'Cancel', false),
    createSecondaryButton('discard', 'Discard Changes'),
    createPrimaryButton('save', 'Save Changes') // Primary action
  ]
)
```

### 3. Handle Keyboard Navigation Properly

```typescript
// ✅ Good - ensure proper focus management
const accessibleModal = modal({
  id: 'accessible',
  title: 'Accessible Modal',
  content: 'This modal properly handles keyboard navigation.',
  modalType: 'custom',
  closeable: true, // Allow Escape key to close
  customButtons: [
    createSecondaryButton('cancel', 'Cancel'),
    createPrimaryButton('ok', 'OK') // This gets default focus
  ]
})

// The modal automatically handles Tab, Shift+Tab, Enter, and Escape
accessibleModal.open()
```

### 4. Manage Modal State Properly

```typescript
// ✅ Good - check modal state before operations
const stateAwareModal = modal({
  id: 'state-aware',
  title: 'State Management',
  content: 'This demonstrates proper state management.',
  modalType: 'basic'
})

// Always check if modal is already open
if (!stateAwareModal.isOpen()) {
  stateAwareModal.open()
}

// Handle results properly
const handleModalResult = () => {
  if (!stateAwareModal.isOpen()) {
    const result = stateAwareModal.getResult()
    if (result) {
      console.log('Modal result:', result)
      // Process the result
    }
  }
}
```

### 5. Use Appropriate Sizing and Positioning

```typescript
// ✅ Good - choose appropriate sizes and positions
const notificationModal = modal({
  id: 'notification',
  title: 'Notification',
  content: 'Brief notification message.',
  modalType: 'alert',
  size: 'small', // Small for simple messages
  position: 'top-center', // Non-intrusive position
  animate: true
})

const detailsModal = modal({
  id: 'details',
  title: 'Detailed Information',
  content: 'This modal contains detailed information that requires more space.',
  modalType: 'basic',
  size: 'large', // Large for detailed content
  position: 'center' // Center for important content
})
```

## Related Widgets

- **[Button](./button)** - Modal action buttons and triggers
- **[Input](./input)** - Form inputs in modal dialogs
- **[Toast](./toast)** - Non-blocking notifications as alternative to modals
- **[Panel](./panel)** - Alternative container for complex content

## Examples

- **[Modal Gallery](../../examples/basic/modals)** - All modal types demonstrated
- **[Task Manager](../../examples/apps/task-manager)** - Real-world modal usage
- **[Settings Dialog](../../examples/advanced/settings-modal)** - Complex modal with forms
- **[Confirmation Flows](../../examples/patterns/confirmations)** - Confirmation dialog patterns

The Modal widget provides comprehensive dialog functionality with full keyboard navigation, focus management, and multiple modal types for building intuitive user interfaces.
