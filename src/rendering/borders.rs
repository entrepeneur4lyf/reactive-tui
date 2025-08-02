//! Unicode box-drawing border system
//! Complete implementation of U+2500-U+257F box-drawing characters

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderStyle {
    // Basic styles
    Light,
    Heavy,
    Double,
    Rounded,

    // Dashed styles
    LightTripleDash,
    HeavyTripleDash,
    LightQuadrupleDash,
    HeavyQuadrupleDash,
    LightDoubleDash,
    HeavyDoubleDash,

    // Mixed styles
    LightHeavyMixed,
    HeavyLightMixed,

    // Special
    Diagonal,
    None,

    // Block drawing styles
    BlockFull,
    BlockLight,
    BlockMedium,
    BlockDark,
    BlockQuadrant,
}

#[derive(Debug, Clone)]
pub struct BorderChars {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub cross: char,
    pub t_down: char,
    pub t_up: char,
    pub t_left: char,
    pub t_right: char,
}

pub struct BorderSet {
    chars: HashMap<BorderStyle, BorderChars>,
}

impl BorderSet {
    pub fn new() -> Self {
        let mut chars = HashMap::new();

        // Light borders (most common)
        chars.insert(
            BorderStyle::Light,
            BorderChars {
                horizontal: '─',   // 2500
                vertical: '│',     // 2502
                top_left: '┌',     // 250C
                top_right: '┐',    // 2510
                bottom_left: '└',  // 2514
                bottom_right: '┘', // 2518
                cross: '┼',        // 253C
                t_down: '┬',       // 252C
                t_up: '┴',         // 2534
                t_left: '┤',       // 2524
                t_right: '├',      // 251C
            },
        );

        // Heavy borders
        chars.insert(
            BorderStyle::Heavy,
            BorderChars {
                horizontal: '━',   // 2501
                vertical: '┃',     // 2503
                top_left: '┏',     // 250F
                top_right: '┓',    // 2513
                bottom_left: '┗',  // 2517
                bottom_right: '┛', // 251B
                cross: '╋',        // 254B
                t_down: '┳',       // 2533
                t_up: '┻',         // 253B
                t_left: '┫',       // 252B
                t_right: '┣',      // 2523
            },
        );

        // Double borders
        chars.insert(
            BorderStyle::Double,
            BorderChars {
                horizontal: '═',   // 2550
                vertical: '║',     // 2551
                top_left: '╔',     // 2554
                top_right: '╗',    // 2557
                bottom_left: '╚',  // 255A
                bottom_right: '╝', // 255D
                cross: '╬',        // 256C
                t_down: '╦',       // 2566
                t_up: '╩',         // 2569
                t_left: '╣',       // 2563
                t_right: '╠',      // 2560
            },
        );

        // Rounded borders (modern look)
        chars.insert(
            BorderStyle::Rounded,
            BorderChars {
                horizontal: '─',   // 2500
                vertical: '│',     // 2502
                top_left: '╭',     // 256D
                top_right: '╮',    // 256E
                bottom_left: '╰',  // 2570
                bottom_right: '╯', // 256F
                cross: '┼',        // 253C
                t_down: '┬',       // 252C
                t_up: '┴',         // 2534
                t_left: '┤',       // 2524
                t_right: '├',      // 251C
            },
        );

        // Light triple dash
        chars.insert(
            BorderStyle::LightTripleDash,
            BorderChars {
                horizontal: '┄',   // 2504
                vertical: '┆',     // 2506
                top_left: '┌',     // 250C
                top_right: '┐',    // 2510
                bottom_left: '└',  // 2514
                bottom_right: '┘', // 2518
                cross: '┼',        // 253C
                t_down: '┬',       // 252C
                t_up: '┴',         // 2534
                t_left: '┤',       // 2524
                t_right: '├',      // 251C
            },
        );

        // Heavy triple dash
        chars.insert(
            BorderStyle::HeavyTripleDash,
            BorderChars {
                horizontal: '┅',   // 2505
                vertical: '┇',     // 2507
                top_left: '┏',     // 250F
                top_right: '┓',    // 2513
                bottom_left: '┗',  // 2517
                bottom_right: '┛', // 251B
                cross: '╋',        // 254B
                t_down: '┳',       // 2533
                t_up: '┻',         // 253B
                t_left: '┫',       // 252B
                t_right: '┣',      // 2523
            },
        );

        // Light quadruple dash
        chars.insert(
            BorderStyle::LightQuadrupleDash,
            BorderChars {
                horizontal: '┈',   // 2508
                vertical: '┊',     // 250A
                top_left: '┌',     // 250C
                top_right: '┐',    // 2510
                bottom_left: '└',  // 2514
                bottom_right: '┘', // 2518
                cross: '┼',        // 253C
                t_down: '┬',       // 252C
                t_up: '┴',         // 2534
                t_left: '┤',       // 2524
                t_right: '├',      // 251C
            },
        );

        // Heavy quadruple dash
        chars.insert(
            BorderStyle::HeavyQuadrupleDash,
            BorderChars {
                horizontal: '┉',   // 2509
                vertical: '┋',     // 250B
                top_left: '┏',     // 250F
                top_right: '┓',    // 2513
                bottom_left: '┗',  // 2517
                bottom_right: '┛', // 251B
                cross: '╋',        // 254B
                t_down: '┳',       // 2533
                t_up: '┻',         // 253B
                t_left: '┫',       // 252B
                t_right: '┣',      // 2523
            },
        );

        // Light double dash
        chars.insert(
            BorderStyle::LightDoubleDash,
            BorderChars {
                horizontal: '╌',   // 254C
                vertical: '╎',     // 254E
                top_left: '┌',     // 250C
                top_right: '┐',    // 2510
                bottom_left: '└',  // 2514
                bottom_right: '┘', // 2518
                cross: '┼',        // 253C
                t_down: '┬',       // 252C
                t_up: '┴',         // 2534
                t_left: '┤',       // 2524
                t_right: '├',      // 251C
            },
        );

        // Heavy double dash
        chars.insert(
            BorderStyle::HeavyDoubleDash,
            BorderChars {
                horizontal: '╍',   // 254D
                vertical: '╏',     // 254F
                top_left: '┏',     // 250F
                top_right: '┓',    // 2513
                bottom_left: '┗',  // 2517
                bottom_right: '┛', // 251B
                cross: '╋',        // 254B
                t_down: '┳',       // 2533
                t_up: '┻',         // 253B
                t_left: '┫',       // 252B
                t_right: '┣',      // 2523
            },
        );

        // None (no border)
        chars.insert(
            BorderStyle::None,
            BorderChars {
                horizontal: ' ',
                vertical: ' ',
                top_left: ' ',
                top_right: ' ',
                bottom_left: ' ',
                bottom_right: ' ',
                cross: ' ',
                t_down: ' ',
                t_up: ' ',
                t_left: ' ',
                t_right: ' ',
            },
        );

        // Block drawing styles using Unicode block characters

        // Full block border style
        chars.insert(
            BorderStyle::BlockFull,
            BorderChars {
                horizontal: '█',   // 2588 FULL BLOCK
                vertical: '█',     // 2588 FULL BLOCK
                top_left: '█',     // 2588 FULL BLOCK
                top_right: '█',    // 2588 FULL BLOCK
                bottom_left: '█',  // 2588 FULL BLOCK
                bottom_right: '█', // 2588 FULL BLOCK
                cross: '█',        // 2588 FULL BLOCK
                t_down: '█',       // 2588 FULL BLOCK
                t_up: '█',         // 2588 FULL BLOCK
                t_left: '█',       // 2588 FULL BLOCK
                t_right: '█',      // 2588 FULL BLOCK
            },
        );

        // Light shade block border
        chars.insert(
            BorderStyle::BlockLight,
            BorderChars {
                horizontal: '░',   // 2591 LIGHT SHADE
                vertical: '░',     // 2591 LIGHT SHADE
                top_left: '▘',     // 2598 QUADRANT UPPER LEFT
                top_right: '▝',    // 259D QUADRANT UPPER RIGHT
                bottom_left: '▖',  // 2596 QUADRANT LOWER LEFT
                bottom_right: '▗', // 2597 QUADRANT LOWER RIGHT
                cross: '░',        // 2591 LIGHT SHADE
                t_down: '▀',       // 2580 UPPER HALF BLOCK
                t_up: '▄',         // 2584 LOWER HALF BLOCK
                t_left: '▐',       // 2590 RIGHT HALF BLOCK
                t_right: '▌',      // 258C LEFT HALF BLOCK
            },
        );

        // Medium shade block border
        chars.insert(
            BorderStyle::BlockMedium,
            BorderChars {
                horizontal: '▒',   // 2592 MEDIUM SHADE
                vertical: '▒',     // 2592 MEDIUM SHADE
                top_left: '▘',     // 2598 QUADRANT UPPER LEFT
                top_right: '▝',    // 259D QUADRANT UPPER RIGHT
                bottom_left: '▖',  // 2596 QUADRANT LOWER LEFT
                bottom_right: '▗', // 2597 QUADRANT LOWER RIGHT
                cross: '▒',        // 2592 MEDIUM SHADE
                t_down: '▀',       // 2580 UPPER HALF BLOCK
                t_up: '▄',         // 2584 LOWER HALF BLOCK
                t_left: '▐',       // 2590 RIGHT HALF BLOCK
                t_right: '▌',      // 258C LEFT HALF BLOCK
            },
        );

        // Dark shade block border
        chars.insert(
            BorderStyle::BlockDark,
            BorderChars {
                horizontal: '▓',   // 2593 DARK SHADE
                vertical: '▓',     // 2593 DARK SHADE
                top_left: '▘',     // 2598 QUADRANT UPPER LEFT
                top_right: '▝',    // 259D QUADRANT UPPER RIGHT
                bottom_left: '▖',  // 2596 QUADRANT LOWER LEFT
                bottom_right: '▗', // 2597 QUADRANT LOWER RIGHT
                cross: '▓',        // 2593 DARK SHADE
                t_down: '▀',       // 2580 UPPER HALF BLOCK
                t_up: '▄',         // 2584 LOWER HALF BLOCK
                t_left: '▐',       // 2590 RIGHT HALF BLOCK
                t_right: '▌',      // 258C LEFT HALF BLOCK
            },
        );

        // Quadrant-based border using block characters
        chars.insert(
            BorderStyle::BlockQuadrant,
            BorderChars {
                horizontal: '▄',   // 2584 LOWER HALF BLOCK
                vertical: '▌',     // 258C LEFT HALF BLOCK
                top_left: '▛',     // 259B QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER LEFT
                top_right: '▜',    // 259C QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER RIGHT
                bottom_left: '▙',  // 2599 QUADRANT UPPER LEFT AND LOWER LEFT AND LOWER RIGHT
                bottom_right: '▟', // 259F QUADRANT UPPER RIGHT AND LOWER LEFT AND LOWER RIGHT
                cross: '▚',        // 259A QUADRANT UPPER LEFT AND LOWER RIGHT
                t_down: '▛',       // 259B QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER LEFT
                t_up: '▙',         // 2599 QUADRANT UPPER LEFT AND LOWER LEFT AND LOWER RIGHT
                t_left: '▜',       // 259C QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER RIGHT
                t_right: '▟',      // 259F QUADRANT UPPER RIGHT AND LOWER LEFT AND LOWER RIGHT
            },
        );

        Self { chars }
    }

    pub fn get_chars(&self, style: BorderStyle) -> &BorderChars {
        self.chars
            .get(&style)
            .unwrap_or(&self.chars[&BorderStyle::Light])
    }

    pub fn get_char(&self, style: BorderStyle, position: BorderPosition) -> char {
        let chars = self.get_chars(style);
        match position {
            BorderPosition::Horizontal => chars.horizontal,
            BorderPosition::Vertical => chars.vertical,
            BorderPosition::TopLeft => chars.top_left,
            BorderPosition::TopRight => chars.top_right,
            BorderPosition::BottomLeft => chars.bottom_left,
            BorderPosition::BottomRight => chars.bottom_right,
            BorderPosition::Cross => chars.cross,
            BorderPosition::TDown => chars.t_down,
            BorderPosition::TUp => chars.t_up,
            BorderPosition::TLeft => chars.t_left,
            BorderPosition::TRight => chars.t_right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderPosition {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Cross,
    TDown,
    TUp,
    TLeft,
    TRight,
}

impl Default for BorderSet {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to convert string to BorderStyle
impl std::str::FromStr for BorderStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(BorderStyle::Light),
            "heavy" => Ok(BorderStyle::Heavy),
            "double" => Ok(BorderStyle::Double),
            "rounded" => Ok(BorderStyle::Rounded),
            "light-triple-dash" => Ok(BorderStyle::LightTripleDash),
            "heavy-triple-dash" => Ok(BorderStyle::HeavyTripleDash),
            "light-quadruple-dash" => Ok(BorderStyle::LightQuadrupleDash),
            "heavy-quadruple-dash" => Ok(BorderStyle::HeavyQuadrupleDash),
            "light-double-dash" => Ok(BorderStyle::LightDoubleDash),
            "heavy-double-dash" => Ok(BorderStyle::HeavyDoubleDash),
            "block-full" => Ok(BorderStyle::BlockFull),
            "block-light" => Ok(BorderStyle::BlockLight),
            "block-medium" => Ok(BorderStyle::BlockMedium),
            "block-dark" => Ok(BorderStyle::BlockDark),
            "block-quadrant" => Ok(BorderStyle::BlockQuadrant),
            "none" => Ok(BorderStyle::None),
            _ => Err(()),
        }
    }
}
