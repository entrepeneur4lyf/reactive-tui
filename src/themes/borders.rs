/*!
 * Border Styles for Terminal UI
 *
 * Provides Unicode border character sets for drawing boxes and frames.
 */

use serde::{Deserialize, Serialize};

/// Border style definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BorderStyle {
    #[serde(rename = "rounded")]
    #[default]
    Rounded,
    #[serde(rename = "sharp")]
    Sharp,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "thick")]
    Thick,
    #[serde(rename = "ascii")]
    Ascii,
}

/// Complete border character set
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorderSet {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub cross: char,
    pub top_tee: char,
    pub bottom_tee: char,
    pub left_tee: char,
    pub right_tee: char,
}

/// Get border character set for a given style
pub fn get_border_set(style: BorderStyle) -> BorderSet {
    match style {
        BorderStyle::Rounded => BorderSet {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
            cross: '┼',
            top_tee: '┬',
            bottom_tee: '┴',
            left_tee: '├',
            right_tee: '┤',
        },
        BorderStyle::Sharp => BorderSet {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            cross: '┼',
            top_tee: '┬',
            bottom_tee: '┴',
            left_tee: '├',
            right_tee: '┤',
        },
        BorderStyle::Double => BorderSet {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            horizontal: '═',
            vertical: '║',
            cross: '╬',
            top_tee: '╦',
            bottom_tee: '╩',
            left_tee: '╠',
            right_tee: '╣',
        },
        BorderStyle::Thick => BorderSet {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            horizontal: '━',
            vertical: '┃',
            cross: '╋',
            top_tee: '┳',
            bottom_tee: '┻',
            left_tee: '┣',
            right_tee: '┫',
        },
        BorderStyle::Ascii => BorderSet {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            cross: '+',
            top_tee: '+',
            bottom_tee: '+',
            left_tee: '+',
            right_tee: '+',
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_styles() {
        let rounded = get_border_set(BorderStyle::Rounded);
        assert_eq!(rounded.top_left, '╭');

        let sharp = get_border_set(BorderStyle::Sharp);
        assert_eq!(sharp.top_left, '┌');

        let ascii = get_border_set(BorderStyle::Ascii);
        assert_eq!(ascii.top_left, '+');
        assert_eq!(ascii.horizontal, '-');
        assert_eq!(ascii.vertical, '|');
    }
}
