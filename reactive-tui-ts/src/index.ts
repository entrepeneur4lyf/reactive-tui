/**
 * tui-bun
 *
 * CSS-styled Terminal User Interface framework for Bun/TypeScript
 *
 * @example
 * ```typescript
 * import { createApp, div, text } from 'tui-bun'
 *
 * const app = createApp({
 *   stylesheet: './styles.css',
 *   component: () =>
 *     div({ class: 'container' })
 *       .child(text('Hello, CSS-styled TUI!'))
 * })
 *
 * await app.run()
 * ```
 */

export * from './app'
export * from './components'
export * from './css'
export * from './types'
export * from './router'
export * from './layout'

// Widget Factory Pattern
export * from './widget-factory'
export * from './widgets/base-widget'
export * from './widgets/factory-button'

// Re-export commonly used functions
export { createApp } from './app'
export { createRouter, fullScreenRouter, inlineRouter } from './router'
export {
  div, text, span, section, header,
  footer, main, container, flexRow, flexColumn,
  // New CLI/TUI components
  line, hr, separator, spacer, code, pre, list,
  border, padding, center, left, right,
  // Widget bridge functions
  buttonWidget, primaryButton, secondaryButton, dangerButton
} from './components'

// Export ResponsiveWidget interface and types
export type { ResponsiveWidget, WidgetSizeConstraints } from './types'

// Widget exports
export {
  Tree, TreeBuilder, createTreeNode, createFolderNode, createLeafNode, tree, treePatterns,
  type TreeNode, type TreeState, type TreeConfig, type TreeNodeStyle, type TreeIndentChars,
  type LazyLoader, type TreeCallbacks, type NodeId, TreeNodeType
} from './widgets/tree'

// Theme exports
export { borderThemes, getBorderTheme, getThemeNames, getThemesByStyle, getThemesByWeight, createThemePreview } from './themes/borders'
export {
  colorThemes, getColorTheme, getColorThemeEnhanced, defaultTheme as defaultColorTheme, createCustomTheme,
  colorToAnsi, getSemanticColor, getSemanticBackground, resetColor,
  rgb, hex, createVariant,  themeBuilder,
  registerTheme, getRegisteredThemeNames, getRegisteredTheme,
  type ColorDefinition, type ColorPalette, type ColorTheme, type ColorMode
} from './themes/colors'
export {
  loadThemeFromFile, loadThemeCollectionFromFile, saveThemeToFile,
  themeToAnsiCodes, getThemePreview, clearThemeCache,
  parseJSONColor, validateJSONTheme, validateJSONThemeCollection,
  validateJSONColor, validateJSONPalette, validateSemanticMappings, sanitizeThemeName,
  type JSONColorDefinition, type JSONColorPalette, type JSONColorTheme, type JSONThemeCollection
} from './themes/json-loader'
export {
  ThemeManager, ThemeBuilder, parseThemeJson, themePresets,
  type ThemeDefinition, type ThemeMetadata, type ThemeColors, type ColorScale,
  type SemanticColors, type SurfaceColors, type Typography, type FontFamilies,
  type FontSizes, type FontWeights, type LineHeights, type ComponentStyle,
  type LayoutTheme, type SpacingScale, type RadiusScale, type ZIndexLayers,
  type AnimationTheme, type DurationScale
} from './themes/theme-system'

// Widget exports
export { panel, dashboardPanel, cardPanel, menuPanel } from './widgets/panel'
export {
  bar, barBuilder, headerBar, footerBar, statusBar, navigationBar, toolbar,
  barItem, clickableBarItem, iconBarItem, weightedBarItem, menuBarItem, statusIndicator,
  createApplicationLayout,
  type BarType, type BarPosition, type BarBorderStyle, type BarSize, type BarItem, type BarStyle, type BarConfig
} from './widgets/bar'
export {
  input, textInput, passwordInput, emailInput, numberInput, searchInput, phoneInput, urlInput,
  formInput, inputGroup,
  type InputType, type InputVariant, type InputSize, type InputStatus, type InputValidation, type InputFormatting, type InputConfig
} from './widgets/input'
export {
  button, successButton, ghostButton, linkButton,
  iconButton, loadingButton, buttonGroup, submitButton, cancelButton, resetButton, toggleButton,
  ButtonWidget,
  type ButtonVariant, type ButtonSize, type ButtonColor, type ButtonShape, type ButtonIcon, type ButtonConfig
} from './widgets/button'
export { grid } from './widgets/grid'
export { slider } from './widgets/slider'
export { toast } from './widgets/toast'
export {
  switchToggle, createSwitch, createCustomSwitch, createDisabledSwitch,
  createFormSwitch, createCompactSwitch, createUnicodeSwitch,
  LabelPosition
} from './widgets/switch'
export {
  radioGroup, createRadioGroup, createHorizontalRadioGroup,
  createCustomRadioGroup, createFormRadioGroup, createCompactRadioGroup,
  createDisabledRadioGroup, createYesNoRadio, createRatingRadio,
  RadioOrientation
} from './widgets/radio'
export {
  linearProgress, circularProgress, arcProgress,
  createProgress, createLinearProgress, createCircularProgress,
  ProgressBuilder, SPINNER_TYPES,
  ProgressStyle, ProgressState
} from './widgets/progress'
export {
  spinnerWidget, createLoadingSpinner, createProcessingSpinner, createSavingSpinner,
  createCustomSpinner, createEmojiSpinner, createMinimalSpinner,
  createProgressSpinner, createBinarySpinner,
  SpinnerType, SpinnerLabelPosition, SpinnerAnimationState
} from './widgets/spinner'
export {
  checkboxWidget, checkboxGroupWidget, createSimpleCheckbox, createAnimatedCheckbox,
  createCustomCheckbox, createCheckboxGroup, createHorizontalCheckboxGroup,
  createFeatureCheckboxGroup, createMultiSelectCheckboxGroup, checkboxAnimationCSS,
  CheckboxStyle, CheckboxLabelPosition, CheckboxGroupOrientation, CheckboxAnimationState
} from './widgets/checkbox'
export {
  tabs, tabsBuilder, horizontalTabs, verticalTabs, bottomTabs, cardTabs, minimalTabs,
  createTab, createIconTab, createBadgeTab, createCloseableTab, createTabLayout,
  type TabPosition, type TabOrientation, type TabSize, type TabBorderStyle, type Tab, type TabStyle, type TabsConfig
} from './widgets/tabs'
export {
  modal as modalWidget, modalBuilder, alertModal, confirmModal, promptModal, customModal, fullscreenModal,
  createModalButton, createPrimaryButton, createSecondaryButton, createDangerButton, createModalExamples,
  type ModalPosition, type ModalSize, type ModalType, type ModalButton, type ModalBackdrop, type ModalStyle, type ModalConfig
} from './widgets/modal'
export {
  menu, contextMenu, menuBar, dropdownMenu,
  menuItem, submenuItem, separatorItem, headerItem, toggleItem, radioItem,
  type MenuOrientation, type MenuItemState, type MenuItemType, type MenuItem, type MenuStyle, type MenuState, type MenuConfig
} from './widgets/menu'
export {
  select, selectPatterns,
  type SelectOption, type SelectMode, type DropdownPosition, type SelectState, type SelectStyle, type SelectConfig, type SelectAPI
} from './widgets/select'
export {
  dataTable, createColumn, simpleDataTable, paginatedDataTable, selectableDataTable, virtualDataTable,
  type DataTableColumn, type DataTableState, type DataTableConfig, type DataTableProps, type DataTableCallbacks,
  type RowId, type ColumnId, ColumnAlignment, SortOrder
} from './widgets/datatable'
export {
  Accordion, createAccordion, createSettingsAccordion, createFaqAccordion, createCompactAccordion, accordionElement,
  type AccordionOptions, type AccordionSection, type AccordionState, type AccordionConfig, type AccordionCallbacks,
  type AccordionAnimation, type AccordionSectionStyle, AnimationEasing, AnimationState
} from './widgets/accordion'
export {
  Viewport, ViewportBuilder, fileViewer, logViewer, dataTableViewport,
  type ViewportItem, type ViewportState, type ViewportConfig, type ViewportStyle, type ViewportCallbacks,
  type ContentId, ScrollMode, SelectionMode, ScrollbarPosition, LazyLoadState
} from './widgets/viewport'
export {
  Autocomplete, AutocompleteBuilder, countryAutocomplete, languageAutocomplete, userAutocomplete, commandAutocomplete,
  type AutocompleteSuggestion, type AutocompleteState, type AutocompleteConfig, type AutocompleteStyle, type AutocompleteCallbacks,
  type SuggestionId, FilterMode, SelectionMode as AutocompleteSelectionMode
} from './widgets/autocomplete'
export {
  RichText, RichTextBuilder, documentationViewer, readmeViewer, codePreview, helpText,
  type MarkdownElement, type RichTextState, type RichTextConfig, type RichTextStyle, type RichTextCallbacks,
  type SyntaxPattern, TableAlignment, SyntaxLanguage, SyntaxPatternType
} from './widgets/rich_text'
export {
  HotReloadManager, HotReloadBuilder, createHotReload, devHotReload,
  type HotReloadConfig, type HotReloadEvent, type HotReloadStats, HotReloadEventType
} from './widgets/hot_reload'

// Plugin system exports
export {
  Plugin, WidgetPlugin, PluginManager, PluginContext, PluginBuilder,
  createPlugin, ExampleCustomButton,
  type PluginMetadata, type PluginDependency, type WidgetConfig,
  type PluginEvent, type PluginResponse, PluginCapability
} from './plugin'
