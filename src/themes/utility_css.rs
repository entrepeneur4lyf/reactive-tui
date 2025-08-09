/*!
 * CSS Utility Classes System
 *
 * Provides a utility-first CSS class system for terminal UIs,
 * inspired by modern CSS frameworks like Tailwind CSS.
 * Makes styling familiar to web developers while maintaining
 * the performance and capabilities of terminal applications.
 */

use crate::themes::{hex, rgb, ColorDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Utility-first compatible color palette (comprehensive Tailwind CSS colors)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilityPalette {
  // Slate/Gray scale (primary neutral)
  pub gray_50: ColorDefinition,
  pub gray_100: ColorDefinition,
  pub gray_200: ColorDefinition,
  pub gray_300: ColorDefinition,
  pub gray_400: ColorDefinition,
  pub gray_500: ColorDefinition,
  pub gray_600: ColorDefinition,
  pub gray_700: ColorDefinition,
  pub gray_800: ColorDefinition,
  pub gray_900: ColorDefinition,
  pub gray_950: ColorDefinition,

  // Blue
  pub blue_50: ColorDefinition,
  pub blue_100: ColorDefinition,
  pub blue_200: ColorDefinition,
  pub blue_300: ColorDefinition,
  pub blue_400: ColorDefinition,
  pub blue_500: ColorDefinition,
  pub blue_600: ColorDefinition,
  pub blue_700: ColorDefinition,
  pub blue_800: ColorDefinition,
  pub blue_900: ColorDefinition,
  pub blue_950: ColorDefinition,

  // Green
  pub green_50: ColorDefinition,
  pub green_100: ColorDefinition,
  pub green_200: ColorDefinition,
  pub green_300: ColorDefinition,
  pub green_400: ColorDefinition,
  pub green_500: ColorDefinition,
  pub green_600: ColorDefinition,
  pub green_700: ColorDefinition,
  pub green_800: ColorDefinition,
  pub green_900: ColorDefinition,
  pub green_950: ColorDefinition,

  // Red
  pub red_50: ColorDefinition,
  pub red_100: ColorDefinition,
  pub red_200: ColorDefinition,
  pub red_300: ColorDefinition,
  pub red_400: ColorDefinition,
  pub red_500: ColorDefinition,
  pub red_600: ColorDefinition,
  pub red_700: ColorDefinition,
  pub red_800: ColorDefinition,
  pub red_900: ColorDefinition,
  pub red_950: ColorDefinition,

  // Amber/Yellow (Tailwind uses amber as primary yellow)
  pub yellow_50: ColorDefinition,
  pub yellow_100: ColorDefinition,
  pub yellow_200: ColorDefinition,
  pub yellow_300: ColorDefinition,
  pub yellow_400: ColorDefinition,
  pub yellow_500: ColorDefinition,
  pub yellow_600: ColorDefinition,
  pub yellow_700: ColorDefinition,
  pub yellow_800: ColorDefinition,
  pub yellow_900: ColorDefinition,
  pub yellow_950: ColorDefinition,

  // Violet/Purple (Tailwind uses violet as primary purple)
  pub purple_50: ColorDefinition,
  pub purple_100: ColorDefinition,
  pub purple_200: ColorDefinition,
  pub purple_300: ColorDefinition,
  pub purple_400: ColorDefinition,
  pub purple_500: ColorDefinition,
  pub purple_600: ColorDefinition,
  pub purple_700: ColorDefinition,
  pub purple_800: ColorDefinition,
  pub purple_900: ColorDefinition,
  pub purple_950: ColorDefinition,

  // Additional Tailwind colors for completeness
  // Orange
  pub orange_50: ColorDefinition,
  pub orange_100: ColorDefinition,
  pub orange_200: ColorDefinition,
  pub orange_300: ColorDefinition,
  pub orange_400: ColorDefinition,
  pub orange_500: ColorDefinition,
  pub orange_600: ColorDefinition,
  pub orange_700: ColorDefinition,
  pub orange_800: ColorDefinition,
  pub orange_900: ColorDefinition,
  pub orange_950: ColorDefinition,

  // Emerald (alternative green)
  pub emerald_50: ColorDefinition,
  pub emerald_100: ColorDefinition,
  pub emerald_200: ColorDefinition,
  pub emerald_300: ColorDefinition,
  pub emerald_400: ColorDefinition,
  pub emerald_500: ColorDefinition,
  pub emerald_600: ColorDefinition,
  pub emerald_700: ColorDefinition,
  pub emerald_800: ColorDefinition,
  pub emerald_900: ColorDefinition,
  pub emerald_950: ColorDefinition,

  // Cyan
  pub cyan_50: ColorDefinition,
  pub cyan_100: ColorDefinition,
  pub cyan_200: ColorDefinition,
  pub cyan_300: ColorDefinition,
  pub cyan_400: ColorDefinition,
  pub cyan_500: ColorDefinition,
  pub cyan_600: ColorDefinition,
  pub cyan_700: ColorDefinition,
  pub cyan_800: ColorDefinition,
  pub cyan_900: ColorDefinition,
  pub cyan_950: ColorDefinition,

  // Indigo
  pub indigo_50: ColorDefinition,
  pub indigo_100: ColorDefinition,
  pub indigo_200: ColorDefinition,
  pub indigo_300: ColorDefinition,
  pub indigo_400: ColorDefinition,
  pub indigo_500: ColorDefinition,
  pub indigo_600: ColorDefinition,
  pub indigo_700: ColorDefinition,
  pub indigo_800: ColorDefinition,
  pub indigo_900: ColorDefinition,
  pub indigo_950: ColorDefinition,

  // Pink
  pub pink_50: ColorDefinition,
  pub pink_100: ColorDefinition,
  pub pink_200: ColorDefinition,
  pub pink_300: ColorDefinition,
  pub pink_400: ColorDefinition,
  pub pink_500: ColorDefinition,
  pub pink_600: ColorDefinition,
  pub pink_700: ColorDefinition,
  pub pink_800: ColorDefinition,
  pub pink_900: ColorDefinition,
  pub pink_950: ColorDefinition,

  // Special colors
  pub white: ColorDefinition,
  pub black: ColorDefinition,
  pub transparent: ColorDefinition,
}

/// CSS utility classes for TUI components
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilityClasses {
  // Text colors
  pub text_colors: HashMap<String, String>,
  // Background colors
  pub bg_colors: HashMap<String, String>,
  // Border colors
  pub border_colors: HashMap<String, String>,
  // Spacing utilities
  pub spacing: HashMap<String, u16>,
  // Border styles
  pub borders: HashMap<String, String>,
  // Typography
  pub typography: HashMap<String, String>,
  // Layout utilities
  pub layout: HashMap<String, String>,
  // State variants
  pub states: HashMap<String, HashMap<String, String>>,
}

  #[inline]
  fn safe_hex(s: &str) -> ColorDefinition {
    hex(s).unwrap_or_else(|_| rgb(0, 0, 0))
  }

/// Default utility-first compatible color palette (matches Tailwind CSS v3.4+ colors)
pub fn default_utility_palette() -> UtilityPalette {
  UtilityPalette {
    // Slate (primary gray scale in Tailwind v3.4+)
    gray_50: safe_hex("#f8fafc"),
    gray_100: safe_hex("#f1f5f9"),
    gray_200: safe_hex("#e2e8f0"),
    gray_300: safe_hex("#cbd5e1"),
    gray_400: safe_hex("#94a3b8"),
    gray_500: safe_hex("#64748b"),
    gray_600: safe_hex("#475569"),
    gray_700: safe_hex("#334155"),
    gray_800: safe_hex("#1e293b"),
    gray_900: safe_hex("#0f172a"),
    gray_950: safe_hex("#020617"),

    // Blue (updated to match Tailwind v3.4+)
    blue_50: safe_hex("#eff6ff"),
    blue_100: safe_hex("#dbeafe"),
    blue_200: safe_hex("#bfdbfe"),
    blue_300: safe_hex("#93c5fd"),
    blue_400: safe_hex("#60a5fa"),
    blue_500: safe_hex("#3b82f6"),
    blue_600: safe_hex("#2563eb"),
    blue_700: safe_hex("#1d4ed8"),
    blue_800: safe_hex("#1e40af"),
    blue_900: safe_hex("#1e3a8a"),
    blue_950: safe_hex("#172554"),

    // Green (updated to match Tailwind v3.4+)
    green_50: safe_hex("#f0fdf4"),
    green_100: safe_hex("#dcfce7"),
    green_200: safe_hex("#bbf7d0"),
    green_300: safe_hex("#86efac"),
    green_400: safe_hex("#4ade80"),
    green_500: safe_hex("#22c55e"),
    green_600: safe_hex("#16a34a"),
    green_700: safe_hex("#15803d"),
    green_800: safe_hex("#166534"),
    green_900: safe_hex("#14532d"),
    green_950: safe_hex("#052e16"),

    // Red (updated to match Tailwind v3.4+)
    red_50: safe_hex("#fef2f2"),
    red_100: safe_hex("#fee2e2"),
    red_200: safe_hex("#fecaca"),
    red_300: safe_hex("#fca5a5"),
    red_400: safe_hex("#f87171"),
    red_500: safe_hex("#ef4444"),
    red_600: safe_hex("#dc2626"),
    red_700: safe_hex("#b91c1c"),
    red_800: safe_hex("#991b1b"),
    red_900: safe_hex("#7f1d1d"),
    red_950: safe_hex("#450a0a"),

    // Amber (Tailwind's primary yellow)
    yellow_50: safe_hex("#fffbeb"),
    yellow_100: safe_hex("#fef3c7"),
    yellow_200: safe_hex("#fde68a"),
    yellow_300: safe_hex("#fcd34d"),
    yellow_400: safe_hex("#fbbf24"),
    yellow_500: safe_hex("#f59e0b"),
    yellow_600: safe_hex("#d97706"),
    yellow_700: safe_hex("#b45309"),
    yellow_800: safe_hex("#92400e"),
    yellow_900: safe_hex("#78350f"),
    yellow_950: safe_hex("#451a03"),

    // Violet (Tailwind's primary purple)
    purple_50: safe_hex("#f5f3ff"),
    purple_100: safe_hex("#ede9fe"),
    purple_200: safe_hex("#ddd6fe"),
    purple_300: safe_hex("#c4b5fd"),
    purple_400: safe_hex("#a78bfa"),
    purple_500: safe_hex("#8b5cf6"),
    purple_600: safe_hex("#7c3aed"),
    purple_700: safe_hex("#6d28d9"),
    purple_800: safe_hex("#5b21b6"),
    purple_900: safe_hex("#4c1d95"),
    purple_950: safe_hex("#2e1065"),

    // Orange (updated to match Tailwind v3.4+)
    orange_50: safe_hex("#fff7ed"),
    orange_100: safe_hex("#ffedd5"),
    orange_200: safe_hex("#fed7aa"),
    orange_300: safe_hex("#fdba74"),
    orange_400: safe_hex("#fb923c"),
    orange_500: safe_hex("#f97316"),
    orange_600: safe_hex("#ea580c"),
    orange_700: safe_hex("#c2410c"),
    orange_800: safe_hex("#9a3412"),
    orange_900: safe_hex("#7c2d12"),
    orange_950: safe_hex("#431407"),

    // Emerald (updated to match Tailwind v3.4+)
    emerald_50: safe_hex("#ecfdf5"),
    emerald_100: safe_hex("#d1fae5"),
    emerald_200: safe_hex("#a7f3d0"),
    emerald_300: safe_hex("#6ee7b7"),
    emerald_400: safe_hex("#34d399"),
    emerald_500: safe_hex("#10b981"),
    emerald_600: safe_hex("#059669"),
    emerald_700: safe_hex("#047857"),
    emerald_800: safe_hex("#065f46"),
    emerald_900: safe_hex("#064e3b"),
    emerald_950: safe_hex("#022c22"),

    // Cyan (updated to match Tailwind v3.4+)
    cyan_50: safe_hex("#ecfeff"),
    cyan_100: safe_hex("#cffafe"),
    cyan_200: safe_hex("#a5f3fc"),
    cyan_300: safe_hex("#67e8f9"),
    cyan_400: safe_hex("#22d3ee"),
    cyan_500: safe_hex("#06b6d4"),
    cyan_600: safe_hex("#0891b2"),
    cyan_700: safe_hex("#0e7490"),
    cyan_800: safe_hex("#155e75"),
    cyan_900: safe_hex("#164e63"),
    cyan_950: safe_hex("#083344"),

    // Indigo (updated to match Tailwind v3.4+)
    indigo_50: safe_hex("#eef2ff"),
    indigo_100: safe_hex("#e0e7ff"),
    indigo_200: safe_hex("#c7d2fe"),
    indigo_300: safe_hex("#a5b4fc"),
    indigo_400: safe_hex("#818cf8"),
    indigo_500: safe_hex("#6366f1"),
    indigo_600: safe_hex("#4f46e5"),
    indigo_700: safe_hex("#4338ca"),
    indigo_800: safe_hex("#3730a3"),
    indigo_900: safe_hex("#312e81"),
    indigo_950: safe_hex("#1e1b4b"),

    // Pink (updated to match Tailwind v3.4+)
    pink_50: safe_hex("#fdf2f8"),
    pink_100: safe_hex("#fce7f3"),
    pink_200: safe_hex("#fbcfe8"),
    pink_300: safe_hex("#f9a8d4"),
    pink_400: safe_hex("#f472b6"),
    pink_500: safe_hex("#ec4899"),
    pink_600: safe_hex("#db2777"),
    pink_700: safe_hex("#be185d"),
    pink_800: safe_hex("#9d174d"),
    pink_900: safe_hex("#831843"),
    pink_950: safe_hex("#500724"),

    // Special colors
    white: rgb(255, 255, 255),
    black: rgb(0, 0, 0),
    transparent: rgb(0, 0, 0), // Treated as no-color in terminal
  }
}

/// Generate utility CSS classes
pub fn generate_utility_classes(palette: &UtilityPalette) -> UtilityClasses {
  let mut text_colors = HashMap::new();
  let mut bg_colors = HashMap::new();
  let mut border_colors = HashMap::new();

  // Generate color classes
  macro_rules! add_color_class {
    ($name:expr, $color:expr) => {
      text_colors.insert(
        format!("text-{}", $name),
        format!("\x1B[38;2;{};{};{}m", $color.r, $color.g, $color.b),
      );
      bg_colors.insert(
        format!("bg-{}", $name),
        format!("\x1B[48;2;{};{};{}m", $color.r, $color.g, $color.b),
      );
      border_colors.insert(
        format!("border-{}", $name),
        format!("\x1B[38;2;{};{};{}m", $color.r, $color.g, $color.b),
      );
    };
  }

  // Gray colors
  add_color_class!("gray-50", palette.gray_50);
  add_color_class!("gray-100", palette.gray_100);
  add_color_class!("gray-200", palette.gray_200);
  add_color_class!("gray-300", palette.gray_300);
  add_color_class!("gray-400", palette.gray_400);
  add_color_class!("gray-500", palette.gray_500);
  add_color_class!("gray-600", palette.gray_600);
  add_color_class!("gray-700", palette.gray_700);
  add_color_class!("gray-800", palette.gray_800);
  add_color_class!("gray-900", palette.gray_900);
  add_color_class!("gray-950", palette.gray_950);

  // Blue colors
  add_color_class!("blue-50", palette.blue_50);
  add_color_class!("blue-100", palette.blue_100);
  add_color_class!("blue-200", palette.blue_200);
  add_color_class!("blue-300", palette.blue_300);
  add_color_class!("blue-400", palette.blue_400);
  add_color_class!("blue-500", palette.blue_500);
  add_color_class!("blue-600", palette.blue_600);
  add_color_class!("blue-700", palette.blue_700);
  add_color_class!("blue-800", palette.blue_800);
  add_color_class!("blue-900", palette.blue_900);
  add_color_class!("blue-950", palette.blue_950);

  // Green colors
  add_color_class!("green-50", palette.green_50);
  add_color_class!("green-100", palette.green_100);
  add_color_class!("green-200", palette.green_200);
  add_color_class!("green-300", palette.green_300);
  add_color_class!("green-400", palette.green_400);
  add_color_class!("green-500", palette.green_500);
  add_color_class!("green-600", palette.green_600);
  add_color_class!("green-700", palette.green_700);
  add_color_class!("green-800", palette.green_800);
  add_color_class!("green-900", palette.green_900);
  add_color_class!("green-950", palette.green_950);

  // Red colors
  add_color_class!("red-50", palette.red_50);
  add_color_class!("red-100", palette.red_100);
  add_color_class!("red-200", palette.red_200);
  add_color_class!("red-300", palette.red_300);
  add_color_class!("red-400", palette.red_400);
  add_color_class!("red-500", palette.red_500);
  add_color_class!("red-600", palette.red_600);
  add_color_class!("red-700", palette.red_700);
  add_color_class!("red-800", palette.red_800);
  add_color_class!("red-900", palette.red_900);
  add_color_class!("red-950", palette.red_950);

  // Yellow colors
  add_color_class!("yellow-50", palette.yellow_50);
  add_color_class!("yellow-100", palette.yellow_100);
  add_color_class!("yellow-200", palette.yellow_200);
  add_color_class!("yellow-300", palette.yellow_300);
  add_color_class!("yellow-400", palette.yellow_400);
  add_color_class!("yellow-500", palette.yellow_500);
  add_color_class!("yellow-600", palette.yellow_600);
  add_color_class!("yellow-700", palette.yellow_700);
  add_color_class!("yellow-800", palette.yellow_800);
  add_color_class!("yellow-900", palette.yellow_900);
  add_color_class!("yellow-950", palette.yellow_950);

  // Purple colors
  add_color_class!("purple-50", palette.purple_50);
  add_color_class!("purple-100", palette.purple_100);
  add_color_class!("purple-200", palette.purple_200);
  add_color_class!("purple-300", palette.purple_300);
  add_color_class!("purple-400", palette.purple_400);
  add_color_class!("purple-500", palette.purple_500);
  add_color_class!("purple-600", palette.purple_600);
  add_color_class!("purple-700", palette.purple_700);
  add_color_class!("purple-800", palette.purple_800);
  add_color_class!("purple-900", palette.purple_900);
  add_color_class!("purple-950", palette.purple_950);

  // Orange colors
  add_color_class!("orange-50", palette.orange_50);
  add_color_class!("orange-100", palette.orange_100);
  add_color_class!("orange-200", palette.orange_200);
  add_color_class!("orange-300", palette.orange_300);
  add_color_class!("orange-400", palette.orange_400);
  add_color_class!("orange-500", palette.orange_500);
  add_color_class!("orange-600", palette.orange_600);
  add_color_class!("orange-700", palette.orange_700);
  add_color_class!("orange-800", palette.orange_800);
  add_color_class!("orange-900", palette.orange_900);
  add_color_class!("orange-950", palette.orange_950);

  // Emerald colors
  add_color_class!("emerald-50", palette.emerald_50);
  add_color_class!("emerald-100", palette.emerald_100);
  add_color_class!("emerald-200", palette.emerald_200);
  add_color_class!("emerald-300", palette.emerald_300);
  add_color_class!("emerald-400", palette.emerald_400);
  add_color_class!("emerald-500", palette.emerald_500);
  add_color_class!("emerald-600", palette.emerald_600);
  add_color_class!("emerald-700", palette.emerald_700);
  add_color_class!("emerald-800", palette.emerald_800);
  add_color_class!("emerald-900", palette.emerald_900);
  add_color_class!("emerald-950", palette.emerald_950);

  // Cyan colors
  add_color_class!("cyan-50", palette.cyan_50);
  add_color_class!("cyan-100", palette.cyan_100);
  add_color_class!("cyan-200", palette.cyan_200);
  add_color_class!("cyan-300", palette.cyan_300);
  add_color_class!("cyan-400", palette.cyan_400);
  add_color_class!("cyan-500", palette.cyan_500);
  add_color_class!("cyan-600", palette.cyan_600);
  add_color_class!("cyan-700", palette.cyan_700);
  add_color_class!("cyan-800", palette.cyan_800);
  add_color_class!("cyan-900", palette.cyan_900);
  add_color_class!("cyan-950", palette.cyan_950);

  // Indigo colors
  add_color_class!("indigo-50", palette.indigo_50);
  add_color_class!("indigo-100", palette.indigo_100);
  add_color_class!("indigo-200", palette.indigo_200);
  add_color_class!("indigo-300", palette.indigo_300);
  add_color_class!("indigo-400", palette.indigo_400);
  add_color_class!("indigo-500", palette.indigo_500);
  add_color_class!("indigo-600", palette.indigo_600);
  add_color_class!("indigo-700", palette.indigo_700);
  add_color_class!("indigo-800", palette.indigo_800);
  add_color_class!("indigo-900", palette.indigo_900);
  add_color_class!("indigo-950", palette.indigo_950);

  // Pink colors
  add_color_class!("pink-50", palette.pink_50);
  add_color_class!("pink-100", palette.pink_100);
  add_color_class!("pink-200", palette.pink_200);
  add_color_class!("pink-300", palette.pink_300);
  add_color_class!("pink-400", palette.pink_400);
  add_color_class!("pink-500", palette.pink_500);
  add_color_class!("pink-600", palette.pink_600);
  add_color_class!("pink-700", palette.pink_700);
  add_color_class!("pink-800", palette.pink_800);
  add_color_class!("pink-900", palette.pink_900);
  add_color_class!("pink-950", palette.pink_950);

  // Special colors
  add_color_class!("white", palette.white);
  add_color_class!("black", palette.black);

  // Spacing utilities (terminal-optimized)
  let mut spacing = HashMap::new();
  spacing.insert("p-0".to_string(), 0);
  spacing.insert("p-1".to_string(), 1);
  spacing.insert("p-2".to_string(), 2);
  spacing.insert("p-3".to_string(), 3);
  spacing.insert("p-4".to_string(), 4);
  spacing.insert("p-6".to_string(), 6);
  spacing.insert("p-8".to_string(), 8);
  spacing.insert("px-0".to_string(), 0);
  spacing.insert("px-1".to_string(), 1);
  spacing.insert("px-2".to_string(), 2);
  spacing.insert("px-3".to_string(), 3);
  spacing.insert("px-4".to_string(), 4);
  spacing.insert("py-0".to_string(), 0);
  spacing.insert("py-1".to_string(), 1);
  spacing.insert("py-2".to_string(), 2);
  spacing.insert("py-3".to_string(), 3);

  // Border utilities
  let mut borders = HashMap::new();
  borders.insert("border".to_string(), "1".to_string());
  borders.insert("border-0".to_string(), "0".to_string());
  borders.insert("border-2".to_string(), "2".to_string());
  borders.insert("border-solid".to_string(), "solid".to_string());
  borders.insert("border-dashed".to_string(), "dashed".to_string());
  borders.insert("border-dotted".to_string(), "dotted".to_string());
  borders.insert("rounded".to_string(), "rounded".to_string());
  borders.insert("rounded-md".to_string(), "rounded-md".to_string());
  borders.insert("rounded-lg".to_string(), "rounded-lg".to_string());
  borders.insert("rounded-none".to_string(), "none".to_string());

  // Typography utilities
  let mut typography = HashMap::new();
  typography.insert("font-normal".to_string(), "normal".to_string());
  typography.insert("font-bold".to_string(), "\x1B[1m".to_string());
  typography.insert("font-light".to_string(), "\x1B[2m".to_string());
  typography.insert("italic".to_string(), "\x1B[3m".to_string());
  typography.insert("underline".to_string(), "\x1B[4m".to_string());
  typography.insert("line-through".to_string(), "\x1B[9m".to_string());

  // Layout utilities
  let mut layout = HashMap::new();
  layout.insert("block".to_string(), "block".to_string());
  layout.insert("inline".to_string(), "inline".to_string());
  layout.insert("inline-block".to_string(), "inline-block".to_string());
  layout.insert("flex".to_string(), "flex".to_string());
  layout.insert("hidden".to_string(), "none".to_string());

  // State variants (hover, focus, etc.)
  let mut states = HashMap::new();

  let mut hover_states = HashMap::new();
  hover_states.insert(
    "hover:bg-gray-100".to_string(),
    format!(
      "\x1B[48;2;{};{};{}m",
      palette.gray_100.r, palette.gray_100.g, palette.gray_100.b
    ),
  );
  hover_states.insert(
    "hover:bg-blue-500".to_string(),
    format!(
      "\x1B[48;2;{};{};{}m",
      palette.blue_500.r, palette.blue_500.g, palette.blue_500.b
    ),
  );
  hover_states.insert(
    "hover:text-blue-600".to_string(),
    format!(
      "\x1B[38;2;{};{};{}m",
      palette.blue_600.r, palette.blue_600.g, palette.blue_600.b
    ),
  );

  let mut focus_states = HashMap::new();
  focus_states.insert(
    "focus:border-blue-500".to_string(),
    format!(
      "\x1B[38;2;{};{};{}m",
      palette.blue_500.r, palette.blue_500.g, palette.blue_500.b
    ),
  );
  focus_states.insert("focus:ring-2".to_string(), "ring-focus".to_string());
  focus_states.insert(
    "focus:ring-blue-500".to_string(),
    format!(
      "\x1B[38;2;{};{};{}m",
      palette.blue_500.r, palette.blue_500.g, palette.blue_500.b
    ),
  );

  states.insert("hover".to_string(), hover_states);
  states.insert("focus".to_string(), focus_states);

  UtilityClasses {
    text_colors,
    bg_colors,
    border_colors,
    spacing,
    borders,
    typography,
    layout,
    states,
  }
}

/// CSS Class processor that converts utility classes to ANSI codes
pub struct UtilityProcessor {
  palette: UtilityPalette,
  classes: UtilityClasses,
}

impl UtilityProcessor {
  pub fn new() -> Self {
    let palette = default_utility_palette();
    let classes = generate_utility_classes(&palette);

    Self { palette, classes }
  }

  pub fn with_palette(palette: UtilityPalette) -> Self {
    let classes = generate_utility_classes(&palette);

    Self { palette, classes }
  }

  /// Process a list of CSS classes and return combined ANSI codes
  pub fn process_classes(&self, class_names: &[String]) -> String {
    let mut result = String::new();

    for class_name in class_names {
      // Text colors
      if let Some(ansi) = self.classes.text_colors.get(class_name) {
        result.push_str(ansi);
      }
      // Background colors
      else if let Some(ansi) = self.classes.bg_colors.get(class_name) {
        result.push_str(ansi);
      }
      // Typography
      else if let Some(ansi) = self.classes.typography.get(class_name) {
        result.push_str(ansi);
      }
      // State variants
      else if class_name.contains(':') {
        let parts: Vec<&str> = class_name.split(':').collect();
        if parts.len() == 2 {
          let state = parts[0];
          let _class = parts[1];
          if let Some(state_map) = self.classes.states.get(state) {
            if let Some(ansi) = state_map.get(class_name) {
              result.push_str(ansi);
            }
          }
        }
      }
    }

    result
  }

  /// Get spacing value for a class
  pub fn get_spacing(&self, class_name: &str) -> Option<u16> {
    self.classes.spacing.get(class_name).copied()
  }

  /// Check if a class is a border class
  pub fn is_border_class(&self, class_name: &str) -> bool {
    self.classes.borders.contains_key(class_name)
  }

  /// Get color definition for a color class
  pub fn get_color(&self, color_name: &str) -> Option<ColorDefinition> {
    match color_name {
      // Gray colors
      "gray-50" => Some(self.palette.gray_50),
      "gray-100" => Some(self.palette.gray_100),
      "gray-200" => Some(self.palette.gray_200),
      "gray-300" => Some(self.palette.gray_300),
      "gray-400" => Some(self.palette.gray_400),
      "gray-500" => Some(self.palette.gray_500),
      "gray-600" => Some(self.palette.gray_600),
      "gray-700" => Some(self.palette.gray_700),
      "gray-800" => Some(self.palette.gray_800),
      "gray-900" => Some(self.palette.gray_900),
      "gray-950" => Some(self.palette.gray_950),

      // Blue colors
      "blue-50" => Some(self.palette.blue_50),
      "blue-100" => Some(self.palette.blue_100),
      "blue-200" => Some(self.palette.blue_200),
      "blue-300" => Some(self.palette.blue_300),
      "blue-400" => Some(self.palette.blue_400),
      "blue-500" => Some(self.palette.blue_500),
      "blue-600" => Some(self.palette.blue_600),
      "blue-700" => Some(self.palette.blue_700),
      "blue-800" => Some(self.palette.blue_800),
      "blue-900" => Some(self.palette.blue_900),
      "blue-950" => Some(self.palette.blue_950),

      // Green colors
      "green-50" => Some(self.palette.green_50),
      "green-100" => Some(self.palette.green_100),
      "green-200" => Some(self.palette.green_200),
      "green-300" => Some(self.palette.green_300),
      "green-400" => Some(self.palette.green_400),
      "green-500" => Some(self.palette.green_500),
      "green-600" => Some(self.palette.green_600),
      "green-700" => Some(self.palette.green_700),
      "green-800" => Some(self.palette.green_800),
      "green-900" => Some(self.palette.green_900),
      "green-950" => Some(self.palette.green_950),

      // Red colors
      "red-50" => Some(self.palette.red_50),
      "red-100" => Some(self.palette.red_100),
      "red-200" => Some(self.palette.red_200),
      "red-300" => Some(self.palette.red_300),
      "red-400" => Some(self.palette.red_400),
      "red-500" => Some(self.palette.red_500),
      "red-600" => Some(self.palette.red_600),
      "red-700" => Some(self.palette.red_700),
      "red-800" => Some(self.palette.red_800),
      "red-900" => Some(self.palette.red_900),
      "red-950" => Some(self.palette.red_950),

      // Yellow colors
      "yellow-50" => Some(self.palette.yellow_50),
      "yellow-100" => Some(self.palette.yellow_100),
      "yellow-200" => Some(self.palette.yellow_200),
      "yellow-300" => Some(self.palette.yellow_300),
      "yellow-400" => Some(self.palette.yellow_400),
      "yellow-500" => Some(self.palette.yellow_500),
      "yellow-600" => Some(self.palette.yellow_600),
      "yellow-700" => Some(self.palette.yellow_700),
      "yellow-800" => Some(self.palette.yellow_800),
      "yellow-900" => Some(self.palette.yellow_900),
      "yellow-950" => Some(self.palette.yellow_950),

      // Purple colors
      "purple-50" => Some(self.palette.purple_50),
      "purple-100" => Some(self.palette.purple_100),
      "purple-200" => Some(self.palette.purple_200),
      "purple-300" => Some(self.palette.purple_300),
      "purple-400" => Some(self.palette.purple_400),
      "purple-500" => Some(self.palette.purple_500),
      "purple-600" => Some(self.palette.purple_600),
      "purple-700" => Some(self.palette.purple_700),
      "purple-800" => Some(self.palette.purple_800),
      "purple-900" => Some(self.palette.purple_900),
      "purple-950" => Some(self.palette.purple_950),

      // Orange colors
      "orange-50" => Some(self.palette.orange_50),
      "orange-100" => Some(self.palette.orange_100),
      "orange-200" => Some(self.palette.orange_200),
      "orange-300" => Some(self.palette.orange_300),
      "orange-400" => Some(self.palette.orange_400),
      "orange-500" => Some(self.palette.orange_500),
      "orange-600" => Some(self.palette.orange_600),
      "orange-700" => Some(self.palette.orange_700),
      "orange-800" => Some(self.palette.orange_800),
      "orange-900" => Some(self.palette.orange_900),
      "orange-950" => Some(self.palette.orange_950),

      // Emerald colors
      "emerald-50" => Some(self.palette.emerald_50),
      "emerald-100" => Some(self.palette.emerald_100),
      "emerald-200" => Some(self.palette.emerald_200),
      "emerald-300" => Some(self.palette.emerald_300),
      "emerald-400" => Some(self.palette.emerald_400),
      "emerald-500" => Some(self.palette.emerald_500),
      "emerald-600" => Some(self.palette.emerald_600),
      "emerald-700" => Some(self.palette.emerald_700),
      "emerald-800" => Some(self.palette.emerald_800),
      "emerald-900" => Some(self.palette.emerald_900),
      "emerald-950" => Some(self.palette.emerald_950),

      // Cyan colors
      "cyan-50" => Some(self.palette.cyan_50),
      "cyan-100" => Some(self.palette.cyan_100),
      "cyan-200" => Some(self.palette.cyan_200),
      "cyan-300" => Some(self.palette.cyan_300),
      "cyan-400" => Some(self.palette.cyan_400),
      "cyan-500" => Some(self.palette.cyan_500),
      "cyan-600" => Some(self.palette.cyan_600),
      "cyan-700" => Some(self.palette.cyan_700),
      "cyan-800" => Some(self.palette.cyan_800),
      "cyan-900" => Some(self.palette.cyan_900),
      "cyan-950" => Some(self.palette.cyan_950),

      // Indigo colors
      "indigo-50" => Some(self.palette.indigo_50),
      "indigo-100" => Some(self.palette.indigo_100),
      "indigo-200" => Some(self.palette.indigo_200),
      "indigo-300" => Some(self.palette.indigo_300),
      "indigo-400" => Some(self.palette.indigo_400),
      "indigo-500" => Some(self.palette.indigo_500),
      "indigo-600" => Some(self.palette.indigo_600),
      "indigo-700" => Some(self.palette.indigo_700),
      "indigo-800" => Some(self.palette.indigo_800),
      "indigo-900" => Some(self.palette.indigo_900),
      "indigo-950" => Some(self.palette.indigo_950),

      // Pink colors
      "pink-50" => Some(self.palette.pink_50),
      "pink-100" => Some(self.palette.pink_100),
      "pink-200" => Some(self.palette.pink_200),
      "pink-300" => Some(self.palette.pink_300),
      "pink-400" => Some(self.palette.pink_400),
      "pink-500" => Some(self.palette.pink_500),
      "pink-600" => Some(self.palette.pink_600),
      "pink-700" => Some(self.palette.pink_700),
      "pink-800" => Some(self.palette.pink_800),
      "pink-900" => Some(self.palette.pink_900),
      "pink-950" => Some(self.palette.pink_950),

      // Special colors
      "white" => Some(self.palette.white),
      "black" => Some(self.palette.black),

      _ => None,
    }
  }
}

impl Default for UtilityProcessor {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_utility_palette_creation() {
    let palette = default_utility_palette();

    // Test that colors are properly defined
    assert_eq!(palette.white, rgb(255, 255, 255));
    assert_eq!(palette.black, rgb(0, 0, 0));
    assert_eq!(palette.blue_500, hex("#3b82f6").unwrap());
    assert_eq!(palette.red_500, hex("#ef4444").unwrap());
  }

  #[test]
  fn test_class_generation() {
    let palette = default_utility_palette();
    let classes = generate_utility_classes(&palette);

    // Test that classes are generated
    assert!(classes.text_colors.contains_key("text-blue-500"));
    assert!(classes.bg_colors.contains_key("bg-red-500"));
    assert!(classes.border_colors.contains_key("border-gray-300"));
    assert!(classes.spacing.contains_key("p-4"));
    assert!(classes.typography.contains_key("font-bold"));
  }

  #[test]
  fn test_processor() {
    let processor = UtilityProcessor::new();

    let classes = vec![
      "text-blue-500".to_string(),
      "bg-gray-100".to_string(),
      "font-bold".to_string(),
    ];

    let result = processor.process_classes(&classes);

    // Should contain ANSI codes
    assert!(result.contains("\x1B[38;2;")); // Text color
    assert!(result.contains("\x1B[48;2;")); // Background color
    assert!(result.contains("\x1B[1m")); // Bold
  }

  #[test]
  fn test_spacing_utilities() {
    let processor = UtilityProcessor::new();

    assert_eq!(processor.get_spacing("p-2"), Some(2));
    assert_eq!(processor.get_spacing("px-4"), Some(4));
    assert_eq!(processor.get_spacing("py-1"), Some(1));
    assert_eq!(processor.get_spacing("invalid"), None);
  }

  #[test]
  fn test_color_retrieval() {
    let processor = UtilityProcessor::new();

    assert_eq!(
      processor.get_color("blue-500"),
      Some(hex("#3b82f6").unwrap())
    );
    assert_eq!(
      processor.get_color("red-500"),
      Some(hex("#ef4444").unwrap())
    );
    assert_eq!(processor.get_color("white"), Some(rgb(255, 255, 255)));
    assert_eq!(processor.get_color("invalid"), None);
  }
}
