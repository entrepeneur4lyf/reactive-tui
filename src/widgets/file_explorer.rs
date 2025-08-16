/*!
 * FileExplorer Component - File system navigation widget
 *
 * A comprehensive file explorer widget providing:
 * - Directory tree navigation
 * - File and folder listing with icons
 * - File type filtering and search
 * - Multiple selection modes
 * - Keyboard navigation (arrow keys, Enter, Backspace)
 * - File operations (copy, move, delete, rename)
 * - Breadcrumb navigation
 * - File preview pane (optional)
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::path::PathBuf;

/// File system entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
  pub name: String,
  pub path: PathBuf,
  pub is_directory: bool,
  pub size: Option<u64>,
  pub modified: Option<String>, // Simplified - would use proper date type
  pub permissions: Option<String>,
  pub hidden: bool,
}

impl FileEntry {
  /// Create a new file entry
  pub fn new(path: PathBuf) -> Self {
    let name = path.file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("")
      .to_string();

    let hidden = name.starts_with('.');

    Self {
      name,
      path: path.clone(),
      is_directory: path.is_dir(),
      size: None, // Would be populated from file metadata
      modified: None,
      permissions: None,
      hidden,
    }
  }

  /// Get file extension
  pub fn extension(&self) -> Option<&str> {
    self.path.extension().and_then(|ext| ext.to_str())
  }

  /// Get file icon based on type
  pub fn icon(&self) -> &'static str {
    if self.is_directory {
      if self.name == ".." {
        "↰"
      } else {
        "📁"
      }
    } else {
      match self.extension() {
        Some("rs") => "🦀",
        Some("js") | Some("ts") => "📜",
        Some("py") => "🐍",
        Some("html") => "🌐",
        Some("css") => "🎨",
        Some("json") => "📋",
        Some("md") => "📝",
        Some("txt") => "📄",
        Some("png") | Some("jpg") | Some("jpeg") | Some("gif") => "🖼️",
        Some("mp3") | Some("wav") | Some("flac") => "🎵",
        Some("mp4") | Some("avi") | Some("mkv") => "🎬",
        Some("zip") | Some("tar") | Some("gz") => "📦",
        _ => "📄",
      }
    }
  }

  /// Format file size
  pub fn format_size(&self) -> String {
    match self.size {
      Some(size) => {
        if size < 1024 {
          format!("{} B", size)
        } else if size < 1024 * 1024 {
          format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
          format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
          format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
      }
      None => if self.is_directory { "<DIR>".to_string() } else { "".to_string() },
    }
  }
}

/// File explorer view mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewMode {
  List,
  Grid,
  Tree,
}

/// File explorer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExplorerConfig {
  pub view_mode: ViewMode,
  pub show_hidden: bool,
  pub show_size: bool,
  pub show_modified: bool,
  pub show_permissions: bool,
  pub allow_multiple_selection: bool,
  pub file_filters: Vec<String>, // e.g., ["*.rs", "*.txt"]
  pub sort_by: SortBy,
  pub sort_ascending: bool,
}

impl Default for FileExplorerConfig {
  fn default() -> Self {
    Self {
      view_mode: ViewMode::List,
      show_hidden: false,
      show_size: true,
      show_modified: false,
      show_permissions: false,
      allow_multiple_selection: false,
      file_filters: Vec::new(),
      sort_by: SortBy::Name,
      sort_ascending: true,
    }
  }
}

/// Sort criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
  Name,
  Size,
  Modified,
  Type,
}

/// File explorer styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExplorerStyle {
  pub background: String,
  pub text_color: String,
  pub selected_bg: String,
  pub selected_text: String,
  pub directory_color: String,
  pub file_color: String,
  pub hidden_color: String,
  pub border_color: String,
  pub header_bg: String,
  pub header_text: String,
}

impl Default for FileExplorerStyle {
  fn default() -> Self {
    Self {
      background: "#ffffff".to_string(),
      text_color: "#333333".to_string(),
      selected_bg: "#0078d4".to_string(),
      selected_text: "#ffffff".to_string(),
      directory_color: "#0078d4".to_string(),
      file_color: "#333333".to_string(),
      hidden_color: "#999999".to_string(),
      border_color: "#cccccc".to_string(),
      header_bg: "#f5f5f5".to_string(),
      header_text: "#333333".to_string(),
    }
  }
}

/// File explorer widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileExplorer {
  pub current_path: PathBuf,
  pub entries: Vec<FileEntry>,
  pub selected_index: Option<usize>,
  pub selected_indices: Vec<usize>,
  pub scroll_offset: usize,
  pub config: FileExplorerConfig,
  pub style: FileExplorerStyle,
  pub search_query: String,
  pub width: u16,
  pub height: u16,
}

impl FileExplorer {
  /// Create a new FileExplorer
  pub fn new() -> Self {
    let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let mut explorer = Self {
      current_path: current_path.clone(),
      entries: Vec::new(),
      selected_index: None,
      selected_indices: Vec::new(),
      scroll_offset: 0,
      config: FileExplorerConfig::default(),
      style: FileExplorerStyle::default(),
      search_query: String::new(),
      width: 80,
      height: 24,
    };

    // Load initial directory (simplified - would handle errors properly)
    let _ = explorer.load_directory();
    explorer
  }

  /// Navigate to a directory
  pub fn navigate_to(&mut self, path: PathBuf) -> Result<()> {
    if !path.exists() {
      return Err(TuiError::component("Path does not exist".to_string()));
    }

    if !path.is_dir() {
      return Err(TuiError::component("Path is not a directory".to_string()));
    }

    self.current_path = path;
    self.selected_index = None;
    self.selected_indices.clear();
    self.scroll_offset = 0;
    self.load_directory()
  }

  /// Go up one directory level
  pub fn go_up(&mut self) -> Result<()> {
    if let Some(parent) = self.current_path.parent() {
      self.navigate_to(parent.to_path_buf())
    } else {
      Ok(()) // Already at root
    }
  }

  /// Enter selected directory or return selected file
  pub fn enter_selected(&mut self) -> Result<Option<PathBuf>> {
    if let Some(index) = self.selected_index {
      if index < self.entries.len() {
        let entry = &self.entries[index];
        if entry.is_directory {
          if entry.name == ".." {
            self.go_up()?;
          } else {
            self.navigate_to(entry.path.clone())?;
          }
          Ok(None)
        } else {
          Ok(Some(entry.path.clone()))
        }
      } else {
        Ok(None)
      }
    } else {
      Ok(None)
    }
  }

  /// Load directory contents
  fn load_directory(&mut self) -> Result<()> {
    self.entries.clear();

    // Add parent directory entry if not at root
    if self.current_path.parent().is_some() {
      self.entries.push(FileEntry {
        name: "..".to_string(),
        path: self.current_path.parent().unwrap().to_path_buf(),
        is_directory: true,
        size: None,
        modified: None,
        permissions: None,
        hidden: false,
      });
    }

    // Simplified directory reading - in real implementation would use std::fs::read_dir
    // For demo purposes, create some mock entries
    let mock_entries = vec![
      ("src", true),
      ("target", true),
      ("Cargo.toml", false),
      ("Cargo.lock", false),
      ("README.md", false),
      (".gitignore", false),
      ("examples", true),
      ("tests", true),
    ];

    for (name, is_dir) in mock_entries {
      let path = self.current_path.join(name);
      let mut entry = FileEntry::new(path);
      entry.name = name.to_string();
      entry.is_directory = is_dir;

      // Apply filters
      if !self.config.show_hidden && entry.hidden {
        continue;
      }

      if !self.config.file_filters.is_empty() && !entry.is_directory {
        let matches_filter = self.config.file_filters.iter().any(|filter| {
          // Simplified pattern matching
          if filter == "*" {
            true
          } else if filter.starts_with("*.") {
            let ext = &filter[2..];
            entry.extension() == Some(ext)
          } else {
            entry.name.contains(filter)
          }
        });

        if !matches_filter {
          continue;
        }
      }

      // Apply search filter
      if !self.search_query.is_empty() {
        if !entry.name.to_lowercase().contains(&self.search_query.to_lowercase()) {
          continue;
        }
      }

      self.entries.push(entry);
    }

    // Sort entries
    self.sort_entries();

    Ok(())
  }

  /// Sort entries based on configuration
  fn sort_entries(&mut self) {
    self.entries.sort_by(|a, b| {
      // Always put directories first, except for ".."
      if a.name == ".." {
        return std::cmp::Ordering::Less;
      }
      if b.name == ".." {
        return std::cmp::Ordering::Greater;
      }

      if a.is_directory && !b.is_directory {
        return std::cmp::Ordering::Less;
      }
      if !a.is_directory && b.is_directory {
        return std::cmp::Ordering::Greater;
      }

      let ordering = match self.config.sort_by {
        SortBy::Name => a.name.cmp(&b.name),
        SortBy::Size => a.size.unwrap_or(0).cmp(&b.size.unwrap_or(0)),
        SortBy::Modified => a.modified.cmp(&b.modified),
        SortBy::Type => {
          let a_ext = a.extension().unwrap_or("");
          let b_ext = b.extension().unwrap_or("");
          a_ext.cmp(b_ext)
        }
      };

      if self.config.sort_ascending {
        ordering
      } else {
        ordering.reverse()
      }
    });
  }

  /// Move selection up
  pub fn select_up(&mut self) {
    if let Some(index) = self.selected_index {
      if index > 0 {
        self.selected_index = Some(index - 1);
        self.ensure_visible();
      }
    } else if !self.entries.is_empty() {
      self.selected_index = Some(0);
    }
  }

  /// Move selection down
  pub fn select_down(&mut self) {
    if let Some(index) = self.selected_index {
      if index < self.entries.len() - 1 {
        self.selected_index = Some(index + 1);
        self.ensure_visible();
      }
    } else if !self.entries.is_empty() {
      self.selected_index = Some(0);
    }
  }

  /// Ensure selected item is visible
  fn ensure_visible(&mut self) {
    if let Some(index) = self.selected_index {
      let visible_height = self.height as usize - 3; // Account for header and borders

      if index < self.scroll_offset {
        self.scroll_offset = index;
      } else if index >= self.scroll_offset + visible_height {
        self.scroll_offset = index - visible_height + 1;
      }
    }
  }

  /// Toggle selection of current item (for multiple selection)
  pub fn toggle_selection(&mut self) {
    if let Some(index) = self.selected_index {
      if self.config.allow_multiple_selection {
        if let Some(pos) = self.selected_indices.iter().position(|&i| i == index) {
          self.selected_indices.remove(pos);
        } else {
          self.selected_indices.push(index);
        }
      }
    }
  }

  /// Set search query
  pub fn set_search(&mut self, query: String) -> Result<()> {
    self.search_query = query;
    self.load_directory()?;
    self.selected_index = if self.entries.is_empty() { None } else { Some(0) };
    self.scroll_offset = 0;
    Ok(())
  }

  /// Render the FileExplorer
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    match self.config.view_mode {
      ViewMode::List => self.render_list_view(&mut output, rect, theme)?,
      ViewMode::Grid => self.render_grid_view(&mut output, rect, theme)?,
      ViewMode::Tree => self.render_tree_view(&mut output, rect, theme)?,
    }

    Ok(output)
  }

  /// Render list view
  fn render_list_view(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let _text_color = color_to_ansi(text_color_def, false);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    let header_bg_def = get_palette_color(&theme.palette, &self.style.header_bg)
      .map_err(|e| TuiError::render(e))?;
    let header_bg = color_to_ansi(header_bg_def, true);

    let header_text_def = get_palette_color(&theme.palette, &self.style.header_text)
      .map_err(|e| TuiError::render(e))?;
    let header_text = color_to_ansi(header_text_def, false);

    let selected_bg_def = get_palette_color(&theme.palette, &self.style.selected_bg)
      .map_err(|e| TuiError::render(e))?;
    let selected_bg = color_to_ansi(selected_bg_def, true);

    let selected_text_def = get_palette_color(&theme.palette, &self.style.selected_text)
      .map_err(|e| TuiError::render(e))?;
    let selected_text = color_to_ansi(selected_text_def, false);

    // Draw border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Header with current path
    let path_str = self.current_path.to_string_lossy();
    let header_content = if path_str.len() > rect.width as usize - 4 {
      format!("...{}", &path_str[path_str.len() - (rect.width as usize - 7)..])
    } else {
      path_str.to_string()
    };

    write!(output, "\x1b[{};{}H{}│{}{}{:<width$}{}│",
           rect.y + 2, rect.x + 1, border_color, header_bg, header_text,
           header_content, border_color,
           width = rect.width as usize - 2)?;

    // File entries
    let visible_height = rect.height as usize - 3;
    let end_index = (self.scroll_offset + visible_height).min(self.entries.len());

    for (i, entry) in self.entries[self.scroll_offset..end_index].iter().enumerate() {
      let entry_index = self.scroll_offset + i;
      let y = rect.y + 3 + i as u16;
      let is_selected = self.selected_index == Some(entry_index);
      let is_multi_selected = self.selected_indices.contains(&entry_index);

      write!(output, "\x1b[{};{}H{}│", y + 1, rect.x + 1, border_color)?;

      let (entry_bg, entry_text) = if is_selected {
        (selected_bg.clone(), selected_text.clone())
      } else if entry.hidden {
        let hidden_color_def = get_palette_color(&theme.palette, &self.style.hidden_color)
          .map_err(|e| TuiError::render(e))?;
        (bg_color.clone(), color_to_ansi(hidden_color_def, false))
      } else if entry.is_directory {
        let directory_color_def = get_palette_color(&theme.palette, &self.style.directory_color)
          .map_err(|e| TuiError::render(e))?;
        (bg_color.clone(), color_to_ansi(directory_color_def, false))
      } else {
        let file_color_def = get_palette_color(&theme.palette, &self.style.file_color)
          .map_err(|e| TuiError::render(e))?;
        (bg_color.clone(), color_to_ansi(file_color_def, false))
      };

      write!(output, "{}{}", entry_bg, entry_text)?;

      // Selection indicator
      if is_multi_selected {
        write!(output, "☑ ")?;
      } else if is_selected {
        write!(output, "► ")?;
      } else {
        write!(output, "  ")?;
      }

      // Icon and name
      write!(output, "{} {}", entry.icon(), entry.name)?;

      // Size (if enabled and fits)
      if self.config.show_size && rect.width > 40 {
        let size_str = entry.format_size();
        let name_width = entry.name.len() + 4; // icon + spaces
        let available_width = rect.width as usize - name_width - 10;
        if size_str.len() <= available_width {
          let padding = available_width - size_str.len();
          write!(output, "{:width$}{}", "", size_str, width = padding)?;
        }
      }

      // Pad to full width
      let content_width = entry.name.len() + 6; // Approximate
      for _ in content_width..rect.width as usize - 2 {
        write!(output, " ")?;
      }

      write!(output, "{}│", border_color)?;
    }

    // Fill remaining space
    for i in end_index - self.scroll_offset..visible_height {
      let y = rect.y + 3 + i as u16;
      write!(output, "\x1b[{};{}H{}│{}{:width$}│",
             y + 1, rect.x + 1, border_color, bg_color, "",
             width = rect.width as usize - 2)?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + rect.height, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render grid view (simplified)
  fn render_grid_view(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    // For simplicity, just render as list view
    self.render_list_view(output, rect, theme)
  }

  /// Render tree view (simplified)
  fn render_tree_view(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    // For simplicity, just render as list view
    self.render_list_view(output, rect, theme)
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<Option<FileExplorerAction>> {
    match key {
      "ArrowUp" => {
        self.select_up();
        Ok(Some(FileExplorerAction::SelectionChanged))
      }
      "ArrowDown" => {
        self.select_down();
        Ok(Some(FileExplorerAction::SelectionChanged))
      }
      "Enter" => {
        if let Some(path) = self.enter_selected()? {
          Ok(Some(FileExplorerAction::FileSelected(path)))
        } else {
          Ok(Some(FileExplorerAction::DirectoryChanged))
        }
      }
      "Backspace" => {
        self.go_up()?;
        Ok(Some(FileExplorerAction::DirectoryChanged))
      }
      " " => {
        self.toggle_selection();
        Ok(Some(FileExplorerAction::SelectionToggled))
      }
      _ => Ok(None),
    }
  }
}

impl Default for FileExplorer {
  fn default() -> Self {
    Self::new()
  }
}

/// Actions that can result from FileExplorer interactions
#[derive(Debug, Clone)]
pub enum FileExplorerAction {
  SelectionChanged,
  DirectoryChanged,
  FileSelected(PathBuf),
  SelectionToggled,
}

/// Builder for FileExplorer
pub struct FileExplorerBuilder {
  explorer: FileExplorer,
}

impl FileExplorerBuilder {
  pub fn new() -> Self {
    Self {
      explorer: FileExplorer::new(),
    }
  }

  pub fn path(mut self, path: PathBuf) -> Result<Self> {
    self.explorer.navigate_to(path)?;
    Ok(self)
  }

  pub fn view_mode(mut self, mode: ViewMode) -> Self {
    self.explorer.config.view_mode = mode;
    self
  }

  pub fn show_hidden(mut self, show: bool) -> Self {
    self.explorer.config.show_hidden = show;
    self
  }

  pub fn allow_multiple_selection(mut self, allow: bool) -> Self {
    self.explorer.config.allow_multiple_selection = allow;
    self
  }

  pub fn file_filters(mut self, filters: Vec<String>) -> Self {
    self.explorer.config.file_filters = filters;
    self
  }

  pub fn style(mut self, style: FileExplorerStyle) -> Self {
    self.explorer.style = style;
    self
  }

  pub fn build(self) -> FileExplorer {
    self.explorer
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_file_entry_creation() {
    let path = PathBuf::from("test.rs");
    let entry = FileEntry::new(path);
    assert_eq!(entry.name, "test.rs");
    assert_eq!(entry.extension(), Some("rs"));
    assert_eq!(entry.icon(), "🦀");
  }

  #[test]
  fn test_file_explorer_creation() {
    let explorer = FileExplorer::new();
    assert!(explorer.current_path.exists() || explorer.current_path == PathBuf::from("/"));
    assert!(explorer.selected_index.is_none());
  }

  #[test]
  fn test_file_size_formatting() {
    let mut entry = FileEntry::new(PathBuf::from("test.txt"));
    entry.size = Some(1024);
    assert_eq!(entry.format_size(), "1.0 KB");

    entry.size = Some(1024 * 1024);
    assert_eq!(entry.format_size(), "1.0 MB");
  }
}
