# Switch Widget

The Switch widget provides toggle switch functionality with customizable handles, labels, track characters, and flexible positioning options. It integrates with the TUI framework via FFI and includes 6 convenience functions for common switch patterns.

## Basic Usage

```typescript
import { 
  switchToggle, 
  createSwitch, 
  createCustomSwitch, 
  createDisabledSwitch,
  createFormSwitch,
  createCompactSwitch,
  createUnicodeSwitch,
  LabelPosition,
  SwitchConfig 
} from 'reactive-tui';

// Basic switch toggle
const basicSwitch = switchToggle({
  id: 'dark-mode',
  enabled: false,
  onLabel: 'Dark',
  offLabel: 'Light',
  description: 'Toggle dark mode',
  interactive: true
});

// Convenience function
const settingSwitch = createSwitch('notifications', true);

// Custom styled switch
const customSwitch = createCustomSwitch({
  id: 'custom-toggle',
  enabled: false,
  labels: { on: 'YES', off: 'NO' },
  handles: { on: '●', off: '○' },
  width: 10
});
```

## Types

### LabelPosition

```typescript
export enum LabelPosition {
  Before = 'before',
  After = 'after',
  Both = 'both'
}
```

### SwitchConfig

```typescript
interface SwitchConfig {
  id?: string;
  enabled?: boolean;
  interactive?: boolean;
  onLabel?: string;
  offLabel?: string;
  onHandle?: string;
  offHandle?: string;
  trackChar?: string;
  width?: number;
  showLabels?: boolean;
  labelPosition?: LabelPosition;
  description?: string;
  classes?: string[];
}
```

### SwitchState

```typescript
interface SwitchState {
  enabled: boolean;
  interactive: boolean;
  focused: boolean;
}
```

## Examples

### Basic Switch Toggle

```typescript
import { switchToggle, LabelPosition } from 'reactive-tui'

const powerSwitch = switchToggle({
  id: 'power-switch',
  enabled: false,
  onLabel: 'ON',
  offLabel: 'OFF',
  onHandle: '●',
  offHandle: '○',
  trackChar: '─',
  width: 8,
  showLabels: true,
  labelPosition: LabelPosition.After,
  description: 'Power toggle switch',
  interactive: true
});
```

### Label Position Variations

```typescript
const labelPositionSwitches = [
  switchToggle({
    id: 'before-switch',
    enabled: true,
    onLabel: 'Enabled',
    offLabel: 'Disabled',
    labelPosition: LabelPosition.Before,
    description: 'Label before switch'
  }),
  
  switchToggle({
    id: 'after-switch',
    enabled: false,
    onLabel: 'Active',
    offLabel: 'Inactive',
    labelPosition: LabelPosition.After,
    description: 'Label after switch'
  }),
  
  switchToggle({
    id: 'both-switch',
    enabled: true,
    onLabel: 'YES',
    offLabel: 'NO',
    labelPosition: LabelPosition.Both,
    description: 'Labels on both sides'
  })
];
```

### Custom Handles and Track

```typescript
const customSwitch = switchToggle({
  id: 'custom-handles',
  enabled: false,
  onHandle: '✓',
  offHandle: '✗',
  trackChar: '━',
  width: 12,
  onLabel: 'Approved',
  offLabel: 'Pending',
  showLabels: true,
  labelPosition: LabelPosition.After
});
```

### Disabled Switch

```typescript
const readOnlySwitch = switchToggle({
  id: 'readonly-switch',
  enabled: true,
  interactive: false,
  onLabel: 'Locked',
  offLabel: 'Unlocked',
  classes: ['switch-readonly']
});
```

## Convenience Functions

The Switch widget provides 6 convenience functions for common switch patterns:

### createSwitch

Creates a basic switch with default settings:

```typescript
function createSwitch(id: string, enabled: boolean = false): ElementBuilder
```

```typescript
import { createSwitch } from 'reactive-tui'

// Basic switches with default styling
const notificationSwitch = createSwitch('notifications', false);
const autoSaveSwitch = createSwitch('auto-save', true);
const soundSwitch = createSwitch('sound-effects', false);

// Display as: [○─────] OFF  or  [─────●] ON
```

### createCustomSwitch

Creates a customizable switch with advanced options:

```typescript
function createCustomSwitch(config: {
  id: string;
  enabled?: boolean;
  labels?: { on: string; off: string };
  handles?: { on: string; off: string };
  width?: number;
  position?: LabelPosition;
  description?: string;
}): ElementBuilder
```

```typescript
import { createCustomSwitch, LabelPosition } from 'reactive-tui'

// Custom gaming mode switch
const gamingModeSwitch = createCustomSwitch({
  id: 'gaming-mode',
  enabled: false,
  labels: { on: 'GAMING', off: 'NORMAL' },
  handles: { on: '🎮', off: '🖥️' },
  width: 10,
  position: LabelPosition.Before,
  description: 'Toggle gaming mode'
});

// Security switch with symbols
const securitySwitch = createCustomSwitch({
  id: 'security',
  enabled: true,
  labels: { on: 'SECURE', off: 'OPEN' },
  handles: { on: '🔒', off: '🔓' },
  width: 8,
  position: LabelPosition.After
});

// Performance toggle
const performanceSwitch = createCustomSwitch({
  id: 'performance',
  enabled: false,
  labels: { on: 'HIGH', off: 'NORMAL' },
  handles: { on: '⚡', off: '🐌' },
  width: 12,
  position: LabelPosition.Both
});
```

### createDisabledSwitch

Creates a non-interactive switch for display purposes:

```typescript
function createDisabledSwitch(id: string, enabled: boolean = false): ElementBuilder
```

```typescript
import { createDisabledSwitch } from 'reactive-tui'

// Status indicators (non-interactive)
const systemStatusSwitch = createDisabledSwitch('system-status', true);
const connectionStatusSwitch = createDisabledSwitch('connection-status', false);
const licenseStatusSwitch = createDisabledSwitch('license-status', true);

// Display as: [─────●] Locked  or  [○─────] Unlocked
```

### createFormSwitch

Creates a form-style switch with label positioning optimized for forms:

```typescript
function createFormSwitch(config: {
  id: string;
  label: string;
  enabled?: boolean;
  description?: string;
}): ElementBuilder
```

```typescript
import { createFormSwitch } from 'reactive-tui'

// Form field switches
const marketingEmailsSwitch = createFormSwitch({
  id: 'marketing-emails',
  label: 'Receive marketing emails',
  enabled: false,
  description: 'Get updates about new features and promotions'
});

const termsAcceptedSwitch = createFormSwitch({
  id: 'terms-accepted',
  label: 'I agree to the terms and conditions',
  enabled: false,
  description: 'Required to create an account'
});

const newsletterSwitch = createFormSwitch({
  id: 'newsletter',
  label: 'Subscribe to newsletter',
  enabled: true,
  description: 'Weekly updates and tips'
});

// Display as: Subscribe to newsletter [─────●]
```

### createCompactSwitch

Creates a compact switch without labels for space-constrained layouts:

```typescript
function createCompactSwitch(id: string, enabled: boolean = false): ElementBuilder
```

```typescript
import { createCompactSwitch } from 'reactive-tui'

// Compact switches for toolbars and tight spaces
const boldSwitch = createCompactSwitch('bold', false);
const italicSwitch = createCompactSwitch('italic', false);
const underlineSwitch = createCompactSwitch('underline', false);

// Compact settings panel
const compactSettings = [
  createCompactSwitch('wifi', true),
  createCompactSwitch('bluetooth', false),
  createCompactSwitch('location', true),
  createCompactSwitch('notifications', false)
];

// Display as: [○────] (6 characters wide, no labels)
```

### createUnicodeSwitch

Creates a Unicode-styled switch with emoji or symbol handles:

```typescript
function createUnicodeSwitch(config: {
  id: string;
  enabled?: boolean;
  style?: 'emoji' | 'symbols' | 'geometric';
}): ElementBuilder
```

```typescript
import { createUnicodeSwitch } from 'reactive-tui'

// Emoji style switches
const onlineStatusSwitch = createUnicodeSwitch({
  id: 'online-status',
  enabled: true,
  style: 'emoji'
});
// Display as: [─────🟢] Active  or  [🔴─────] Inactive

// Symbol style switches
const taskCompleteSwitch = createUnicodeSwitch({
  id: 'task-complete',
  enabled: false,
  style: 'symbols'
});
// Display as: [✗─────] Inactive  or  [─────✓] Active

// Geometric style switches
const featureToggleSwitch = createUnicodeSwitch({
  id: 'feature-toggle',
  enabled: true,
  style: 'geometric'
});
// Display as: [─────◉] Active  or  [◯─────] Inactive

// Default style (same as basic switch)
const defaultUnicodeSwitch = createUnicodeSwitch({
  id: 'default-unicode',
  enabled: false
});
// Display as: [○─────] Inactive  or  [─────●] Active
```

## Real-World Examples

### Application Settings Panel

```typescript
import { 
  switchToggle, 
  createSwitch, 
  createFormSwitch, 
  createUnicodeSwitch,
  LabelPosition 
} from 'reactive-tui'

class ApplicationSettingsPanel {
  private settings: Map<string, any> = new Map();

  constructor() {
    this.setupSettingSwitches();
  }

  private setupSettingSwitches() {
    // Theme settings
    const darkModeSwitch = createSwitch('dark-mode', false);
    this.settings.set('darkMode', darkModeSwitch);

    const highContrastSwitch = createFormSwitch({
      id: 'high-contrast',
      label: 'High contrast mode',
      enabled: false,
      description: 'Improves visibility for users with visual impairments'
    });
    this.settings.set('highContrast', highContrastSwitch);

    // Notification settings
    const pushNotificationsSwitch = createUnicodeSwitch({
      id: 'push-notifications',
      enabled: true,
      style: 'emoji'
    });
    this.settings.set('pushNotifications', pushNotificationsSwitch);

    const soundNotificationsSwitch = switchToggle({
      id: 'sound-notifications',
      enabled: false,
      onLabel: '🔊 ON',
      offLabel: '🔇 OFF',
      showLabels: true,
      labelPosition: LabelPosition.After,
      description: 'Play sounds for notifications'
    });
    this.settings.set('soundNotifications', soundNotificationsSwitch);

    // Performance settings
    const animationsSwitch = createFormSwitch({
      id: 'animations',
      label: 'Enable animations',
      enabled: true,
      description: 'Smooth transitions and effects'
    });
    this.settings.set('animations', animationsSwitch);

    const autoSaveSwitch = switchToggle({
      id: 'auto-save',
      enabled: true,
      onLabel: 'AUTO',
      offLabel: 'MANUAL',
      onHandle: '💾',
      offHandle: '✏️',
      trackChar: '━',
      width: 10,
      showLabels: true,
      labelPosition: LabelPosition.Both,
      description: 'Automatically save changes'
    });
    this.settings.set('autoSave', autoSaveSwitch);

    // Privacy settings
    const analyticsSwitch = createFormSwitch({
      id: 'analytics',
      label: 'Share usage analytics',
      enabled: false,
      description: 'Help improve the application by sharing anonymous usage data'
    });
    this.settings.set('analytics', analyticsSwitch);

    const crashReportsSwitch = createFormSwitch({
      id: 'crash-reports',
      label: 'Send crash reports',
      enabled: true,
      description: 'Automatically send crash reports to help fix bugs'
    });
    this.settings.set('crashReports', crashReportsSwitch);
  }

  // Get setting value
  getSetting(settingId: string): boolean {
    const switchElement = this.settings.get(settingId);
    if (switchElement) {
      return switchElement.attr('data-enabled') === 'true';
    }
    return false;
  }

  // Update setting value
  setSetting(settingId: string, enabled: boolean) {
    const switchElement = this.settings.get(settingId);
    if (switchElement) {
      switchElement.attr('data-enabled', enabled.toString());
      switchElement.attr('aria-checked', enabled.toString());
      
      // Update visual state classes
      if (enabled) {
        switchElement.addClass('switch-on');
        switchElement.removeClass('switch-off');
      } else {
        switchElement.addClass('switch-off');
        switchElement.removeClass('switch-on');
      }
      
      // Trigger change handlers
      this.handleSettingChange(settingId, enabled);
    }
  }

  private handleSettingChange(settingId: string, enabled: boolean) {
    switch (settingId) {
      case 'darkMode':
        this.applyTheme(enabled ? 'dark' : 'light');
        break;
      case 'highContrast':
        this.toggleHighContrast(enabled);
        break;
      case 'pushNotifications':
        this.updateNotificationPermissions(enabled);
        break;
      case 'soundNotifications':
        this.toggleSoundNotifications(enabled);
        break;
      case 'animations':
        this.toggleAnimations(enabled);
        break;
      case 'autoSave':
        this.configureAutoSave(enabled);
        break;
      case 'analytics':
        this.updateAnalyticsConsent(enabled);
        break;
      case 'crashReports':
        this.updateCrashReporting(enabled);
        break;
    }
  }

  private applyTheme(theme: 'dark' | 'light') {
    document.body.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
    console.log(`Applied ${theme} theme`);
  }

  private toggleHighContrast(enabled: boolean) {
    document.body.classList.toggle('high-contrast', enabled);
    localStorage.setItem('highContrast', enabled.toString());
    console.log(`High contrast mode: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private updateNotificationPermissions(enabled: boolean) {
    if (enabled && 'Notification' in window) {
      Notification.requestPermission().then(permission => {
        if (permission !== 'granted') {
          this.setSetting('pushNotifications', false);
        }
      });
    }
    localStorage.setItem('pushNotifications', enabled.toString());
    console.log(`Push notifications: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private toggleSoundNotifications(enabled: boolean) {
    localStorage.setItem('soundNotifications', enabled.toString());
    console.log(`Sound notifications: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private toggleAnimations(enabled: boolean) {
    document.body.classList.toggle('no-animations', !enabled);
    localStorage.setItem('animations', enabled.toString());
    console.log(`Animations: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private configureAutoSave(enabled: boolean) {
    if (enabled) {
      this.startAutoSave();
    } else {
      this.stopAutoSave();
    }
    localStorage.setItem('autoSave', enabled.toString());
    console.log(`Auto save: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private updateAnalyticsConsent(enabled: boolean) {
    localStorage.setItem('analyticsConsent', enabled.toString());
    if (enabled) {
      this.initializeAnalytics();
    } else {
      this.disableAnalytics();
    }
    console.log(`Analytics: ${enabled ? 'enabled' : 'disabled'}`);
  }

  private updateCrashReporting(enabled: boolean) {
    localStorage.setItem('crashReports', enabled.toString());
    console.log(`Crash reports: ${enabled ? 'enabled' : 'disabled'}`);
  }

  // Helper methods (implementation details)
  private startAutoSave() {
    // Implement auto-save functionality
  }

  private stopAutoSave() {
    // Stop auto-save functionality
  }

  private initializeAnalytics() {
    // Initialize analytics tracking
  }

  private disableAnalytics() {
    // Disable analytics tracking
  }

  // Get all settings for export/import
  exportSettings() {
    const settings: Record<string, boolean> = {};
    for (const [key] of this.settings) {
      settings[key] = this.getSetting(key);
    }
    return settings;
  }

  importSettings(settings: Record<string, boolean>) {
    for (const [key, value] of Object.entries(settings)) {
      if (this.settings.has(key)) {
        this.setSetting(key, value);
      }
    }
  }

  getAllSwitches() {
    return Array.from(this.settings.values());
  }
}

// Usage
const settingsPanel = new ApplicationSettingsPanel();

// Toggle a setting
settingsPanel.setSetting('darkMode', true);

// Get current setting
const isDarkMode = settingsPanel.getSetting('darkMode');

// Export all settings
const currentSettings = settingsPanel.exportSettings();
```

### Gaming Control Panel

```typescript
import { 
  createCustomSwitch, 
  createUnicodeSwitch, 
  createFormSwitch,
  LabelPosition 
} from 'reactive-tui'

class GamingControlPanel {
  private gameSettings: Map<string, any> = new Map();

  constructor() {
    this.setupGamingControls();
  }

  private setupGamingControls() {
    // Performance settings
    const gameMode = createCustomSwitch({
      id: 'game-mode',
      enabled: false,
      labels: { on: 'GAMING', off: 'NORMAL' },
      handles: { on: '🎮', off: '🖥️' },
      width: 12,
      position: LabelPosition.Before,
      description: 'Optimize system for gaming'
    });
    this.gameSettings.set('gameMode', gameMode);

    const highPerformance = createUnicodeSwitch({
      id: 'high-performance',
      enabled: false,
      style: 'symbols'
    });
    this.gameSettings.set('highPerformance', highPerformance);

    // Audio settings
    const surroundSound = createFormSwitch({
      id: 'surround-sound',
      label: '7.1 Surround Sound',
      enabled: false,
      description: 'Enable virtual surround sound'
    });
    this.gameSettings.set('surroundSound', surroundSound);

    const voiceChat = createCustomSwitch({
      id: 'voice-chat',
      enabled: true,
      labels: { on: 'MIC ON', off: 'MIC OFF' },
      handles: { on: '🎤', off: '🔇' },
      width: 10,
      position: LabelPosition.After
    });
    this.gameSettings.set('voiceChat', voiceChat);

    // Display settings
    const vsync = createFormSwitch({
      id: 'vsync',
      label: 'Vertical Sync (VSync)',
      enabled: true,
      description: 'Prevent screen tearing'
    });
    this.gameSettings.set('vsync', vsync);

    const hdr = createCustomSwitch({
      id: 'hdr',
      enabled: false,
      labels: { on: 'HDR', off: 'SDR' },
      handles: { on: '🌟', off: '💡' },
      width: 8,
      position: LabelPosition.Both
    });
    this.gameSettings.set('hdr', hdr);

    // Network settings
    const gameAccelerator = createUnicodeSwitch({
      id: 'game-accelerator',
      enabled: false,
      style: 'emoji'
    });
    this.gameSettings.set('gameAccelerator', gameAccelerator);

    // Streaming settings
    const streamMode = createCustomSwitch({
      id: 'stream-mode',
      enabled: false,
      labels: { on: 'STREAMING', off: 'OFFLINE' },
      handles: { on: '📺', off: '🎯' },
      width: 14,
      position: LabelPosition.Before
    });
    this.gameSettings.set('streamMode', streamMode);
  }

  toggleGameMode(enabled: boolean) {
    this.updateSetting('gameMode', enabled);
    
    if (enabled) {
      // Enable game mode optimizations
      this.updateSetting('highPerformance', true);
      this.updateSetting('gameAccelerator', true);
      this.applyGameModeOptimizations();
    } else {
      // Restore normal mode
      this.restoreNormalMode();
    }
  }

  private updateSetting(settingId: string, enabled: boolean) {
    const switchElement = this.gameSettings.get(settingId);
    if (switchElement) {
      switchElement.attr('data-enabled', enabled.toString());
      switchElement.attr('aria-checked', enabled.toString());
      
      // Update CSS classes for visual state
      if (enabled) {
        switchElement.addClass('switch-on');
        switchElement.removeClass('switch-off');
      } else {
        switchElement.addClass('switch-off');
        switchElement.removeClass('switch-on');
      }
    }
  }

  private applyGameModeOptimizations() {
    console.log('Applying game mode optimizations...');
    // Implement system optimizations for gaming
    this.setPowerProfile('high-performance');
    this.disableBackgroundApps();
    this.optimizeNetworkSettings();
  }

  private restoreNormalMode() {
    console.log('Restoring normal system settings...');
    // Restore normal system settings
    this.setPowerProfile('balanced');
    this.enableBackgroundApps();
    this.restoreNetworkSettings();
  }

  private setPowerProfile(profile: string) {
    console.log(`Setting power profile to: ${profile}`);
  }

  private disableBackgroundApps() {
    console.log('Disabling background applications');
  }

  private enableBackgroundApps() {
    console.log('Re-enabling background applications');
  }

  private optimizeNetworkSettings() {
    console.log('Optimizing network settings for gaming');
  }

  private restoreNetworkSettings() {
    console.log('Restoring default network settings');
  }

  // Profile management
  saveProfile(profileName: string) {
    const profile: Record<string, boolean> = {};
    for (const [key, switchElement] of this.gameSettings) {
      profile[key] = switchElement.attr('data-enabled') === 'true';
    }
    localStorage.setItem(`gaming-profile-${profileName}`, JSON.stringify(profile));
    console.log(`Saved gaming profile: ${profileName}`);
  }

  loadProfile(profileName: string) {
    const profileData = localStorage.getItem(`gaming-profile-${profileName}`);
    if (profileData) {
      const profile = JSON.parse(profileData);
      for (const [key, value] of Object.entries(profile)) {
        if (this.gameSettings.has(key)) {
          this.updateSetting(key, value as boolean);
        }
      }
      console.log(`Loaded gaming profile: ${profileName}`);
    }
  }

  getSettingValue(settingId: string): boolean {
    const switchElement = this.gameSettings.get(settingId);
    return switchElement ? switchElement.attr('data-enabled') === 'true' : false;
  }

  getAllSettings() {
    return Array.from(this.gameSettings.values());
  }
}

// Usage
const gamingPanel = new GamingControlPanel();

// Toggle game mode
gamingPanel.toggleGameMode(true);

// Save current settings as a profile
gamingPanel.saveProfile('competitive-fps');

// Load a different profile
gamingPanel.loadProfile('casual-gaming');
```

### System Status Dashboard

```typescript
import { 
  createDisabledSwitch, 
  createUnicodeSwitch, 
  switchToggle,
  LabelPosition 
} from 'reactive-tui'

class SystemStatusDashboard {
  private statusIndicators: Map<string, any> = new Map();
  private services: Map<string, any> = new Map();

  constructor() {
    this.setupStatusIndicators();
    this.setupServiceControls();
    this.startStatusMonitoring();
  }

  private setupStatusIndicators() {
    // System status indicators (non-interactive)
    const powerStatus = createDisabledSwitch('power-status', true);
    this.statusIndicators.set('power', powerStatus);

    const networkStatus = createUnicodeSwitch({
      id: 'network-status',
      enabled: true,
      style: 'emoji'
    });
    // Make it non-interactive for status display
    networkStatus.attr('data-interactive', 'false');
    networkStatus.addClass('switch-disabled');
    this.statusIndicators.set('network', networkStatus);

    const storageStatus = createDisabledSwitch('storage-status', false);
    this.statusIndicators.set('storage', storageStatus);

    const backupStatus = createUnicodeSwitch({
      id: 'backup-status',
      enabled: false,
      style: 'symbols'
    });
    backupStatus.attr('data-interactive', 'false');
    backupStatus.addClass('switch-disabled');
    this.statusIndicators.set('backup', backupStatus);
  }

  private setupServiceControls() {
    // Interactive service controls
    const webServer = switchToggle({
      id: 'web-server',
      enabled: true,
      onLabel: 'RUNNING',
      offLabel: 'STOPPED',
      onHandle: '🟢',
      offHandle: '🔴',
      trackChar: '━',
      width: 10,
      showLabels: true,
      labelPosition: LabelPosition.After,
      description: 'Web server service',
      interactive: true
    });
    this.services.set('webServer', webServer);

    const database = switchToggle({
      id: 'database',
      enabled: false,
      onLabel: 'ONLINE',
      offLabel: 'OFFLINE',
      onHandle: '💾',
      offHandle: '💿',
      trackChar: '─',
      width: 12,
      showLabels: true,
      labelPosition: LabelPosition.Before,
      description: 'Database service',
      interactive: true
    });
    this.services.set('database', database);

    const monitoring = createUnicodeSwitch({
      id: 'monitoring',
      enabled: true,
      style: 'geometric'
    });
    this.services.set('monitoring', monitoring);

    const firewall = switchToggle({
      id: 'firewall',
      enabled: true,
      onLabel: 'PROTECTED',
      offLabel: 'EXPOSED',
      onHandle: '🛡️',
      offHandle: '⚠️',
      trackChar: '━',
      width: 14,
      showLabels: true,
      labelPosition: LabelPosition.Both,
      description: 'Firewall protection',
      interactive: true
    });
    this.services.set('firewall', firewall);
  }

  private startStatusMonitoring() {
    // Simulate real-time status updates
    setInterval(() => {
      this.updateSystemStatus();
    }, 5000);
  }

  private updateSystemStatus() {
    // Simulate status changes
    const networkConnected = Math.random() > 0.1; // 90% uptime
    const storageHealthy = Math.random() > 0.05; // 95% healthy
    const backupRunning = Math.random() > 0.7; // 30% of time

    this.updateStatusIndicator('network', networkConnected);
    this.updateStatusIndicator('storage', storageHealthy);
    this.updateStatusIndicator('backup', backupRunning);
  }

  private updateStatusIndicator(indicatorId: string, status: boolean) {
    const indicator = this.statusIndicators.get(indicatorId);
    if (indicator) {
      indicator.attr('data-enabled', status.toString());
      indicator.attr('aria-checked', status.toString());
      
      if (status) {
        indicator.addClass('switch-on');
        indicator.removeClass('switch-off');
      } else {
        indicator.addClass('switch-off');
        indicator.removeClass('switch-on');
      }
    }
  }

  // Service control methods
  toggleService(serviceId: string, enabled: boolean): Promise<boolean> {
    return new Promise((resolve) => {
      const service = this.services.get(serviceId);
      if (!service) {
        resolve(false);
        return;
      }

      console.log(`${enabled ? 'Starting' : 'Stopping'} service: ${serviceId}`);
      
      // Simulate service operation delay
      setTimeout(() => {
        service.attr('data-enabled', enabled.toString());
        service.attr('aria-checked', enabled.toString());
        
        if (enabled) {
          service.addClass('switch-on');
          service.removeClass('switch-off');
        } else {
          service.addClass('switch-off');
          service.removeClass('switch-on');
        }
        
        this.logServiceChange(serviceId, enabled);
        resolve(true);
      }, 1000);
    });
  }

  private logServiceChange(serviceId: string, enabled: boolean) {
    const timestamp = new Date().toISOString();
    const action = enabled ? 'STARTED' : 'STOPPED';
    console.log(`[${timestamp}] Service ${serviceId} ${action}`);
  }

  // Emergency shutdown
  async emergencyShutdown() {
    console.log('Initiating emergency shutdown...');
    
    const services = ['webServer', 'database', 'monitoring'];
    for (const serviceId of services) {
      await this.toggleService(serviceId, false);
    }
    
    // Keep firewall enabled for security
    console.log('Emergency shutdown complete. Firewall remains active.');
  }

  // Service health check
  getServiceStatus(serviceId: string): boolean {
    const service = this.services.get(serviceId);
    return service ? service.attr('data-enabled') === 'true' : false;
  }

  getAllServiceStatuses(): Record<string, boolean> {
    const statuses: Record<string, boolean> = {};
    for (const [serviceId] of this.services) {
      statuses[serviceId] = this.getServiceStatus(serviceId);
    }
    return statuses;
  }

  getSystemHealth(): {
    indicators: Record<string, boolean>;
    services: Record<string, boolean>;
    overallHealth: number;
  } {
    const indicators: Record<string, boolean> = {};
    for (const [indicatorId, indicator] of this.statusIndicators) {
      indicators[indicatorId] = indicator.attr('data-enabled') === 'true';
    }

    const services = this.getAllServiceStatuses();
    
    // Calculate overall health score
    const totalChecks = Object.keys(indicators).length + Object.keys(services).length;
    const healthyChecks = Object.values({...indicators, ...services}).filter(Boolean).length;
    const overallHealth = Math.round((healthyChecks / totalChecks) * 100);

    return { indicators, services, overallHealth };
  }
}

// Usage
const dashboard = new SystemStatusDashboard();

// Toggle a service
dashboard.toggleService('database', true).then(success => {
  if (success) {
    console.log('Database service started successfully');
  }
});

// Check system health
const health = dashboard.getSystemHealth();
console.log(`System health: ${health.overallHealth}%`);

// Emergency procedures
if (health.overallHealth < 50) {
  dashboard.emergencyShutdown();
}
```

## CSS Styling

```css
/* Base switch styles */
.switch {
  display: inline-block;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  line-height: 1;
  cursor: pointer;
  user-select: none;
  transition: all 0.2s ease;
}

.switch:focus {
  outline: 2px solid #4299e1;
  outline-offset: 2px;
}

/* Switch states */
.switch-on {
  color: #38a169;
}

.switch-off {
  color: #e53e3e;
}

.switch-disabled {
  color: #a0aec0;
  cursor: not-allowed;
  opacity: 0.6;
}

/* Switch track styling */
.switch-track {
  display: inline-block;
  background-color: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  padding: 2px;
  transition: all 0.2s ease;
}

.switch-on .switch-track {
  background-color: #e6fffa;
  border-color: #38a169;
}

.switch-off .switch-track {
  background-color: #fed7d7;
  border-color: #e53e3e;
}

/* Label positioning */
.switch-label-before {
  margin-right: 8px;
}

.switch-label-after {
  margin-left: 8px;
}

.switch-label-both .switch-label-before {
  margin-right: 8px;
}

.switch-label-both .switch-label-after {
  margin-left: 8px;
}

/* Size variants */
.switch-compact {
  font-size: 12px;
}

.switch-form {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
}

.switch-form .switch-label-before {
  flex: 1;
  text-align: left;
}

/* Unicode style variants */
.switch-unicode {
  font-size: 16px;
}

.switch-emoji {
  font-size: 18px;
}

.switch-symbols {
  font-size: 14px;
  font-weight: bold;
}

.switch-geometric {
  font-size: 16px;
}

/* Animation effects */
.switch {
  transition: color 0.2s ease, background-color 0.2s ease;
}

.switch:hover:not(.switch-disabled) {
  transform: scale(1.05);
}

.switch:active:not(.switch-disabled) {
  transform: scale(0.95);
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .switch-on {
    color: #000;
    background-color: #fff;
  }
  
  .switch-off {
    color: #fff;
    background-color: #000;
  }
  
  .switch-track {
    border-width: 2px;
  }
}

/* Dark theme support */
[data-theme="dark"] .switch {
  color: #e2e8f0;
}

[data-theme="dark"] .switch-on {
  color: #68d391;
}

[data-theme="dark"] .switch-off {
  color: #fc8181;
}

[data-theme="dark"] .switch-track {
  background-color: #2d3748;
  border-color: #4a5568;
}

/* Reduced motion support */
@media (prefers-reduced-motion: reduce) {
  .switch {
    transition: none;
  }
  
  .switch:hover:not(.switch-disabled) {
    transform: none;
  }
}
```

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';
import { createSwitch, createFormSwitch } from 'reactive-tui'

const settingsForm = new ElementBuilderImpl('div')
  .class('settings-form')
  .child(
    createFormSwitch({
      id: 'notifications',
      label: 'Enable notifications',
      enabled: true
    })
  )
  .child(
    createSwitch('dark-mode', false)
  )
  .build();
```

## Best Practices

1. **Switch Type Selection**
   - Use basic switches for simple on/off settings
   - Use form switches for form fields with descriptive labels
   - Use compact switches in space-constrained layouts
   - Use Unicode switches for visual appeal and context

2. **Label Design**
   - Use clear, descriptive labels that indicate the setting
   - Position labels consistently within your interface
   - Consider using "Both" label position for important switches

3. **Interactive vs. Status**
   - Use interactive switches for user controls
   - Use disabled switches for status indicators
   - Clearly distinguish between actionable and informational switches

4. **Accessibility**
   - Always provide meaningful labels and descriptions
   - Use ARIA attributes for screen readers
   - Ensure sufficient color contrast
   - Support keyboard navigation

## Accessibility

The Switch widget includes comprehensive accessibility features:

- ARIA attributes for screen readers (`role="switch"`, `aria-checked`, `aria-label`)
- Keyboard navigation support with focus indicators
- Semantic HTML structure with proper labeling
- High contrast mode support
- Screen reader announcements for state changes

```typescript
const accessibleSwitch = switchToggle({
  id: 'accessible-switch',
  enabled: false,
  onLabel: 'Feature enabled',
  offLabel: 'Feature disabled',
  description: 'Toggle accessibility feature',
  // Automatically includes:
  // - role="switch"
  // - aria-checked="false"
  // - aria-label="Toggle accessibility feature"
  // - focusable="true"
  // - keyboard event handling
});
```

The Switch widget provides comprehensive toggle functionality with 6 convenience functions, customizable handles and labels, flexible positioning options, and extensive accessibility support for building intuitive user interfaces.