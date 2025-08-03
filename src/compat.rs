//! Compatibility layer - direct re-exports of crossterm
//!
//! This module provides a central point for importing crossterm functionality.

pub use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode, ModifierKeyCode,
    MouseButton, MouseEvent, MouseEventKind,
  },
  style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
  },
  terminal::{self, Clear, ClearType},
  tty::IsTty,
  Command,
};
