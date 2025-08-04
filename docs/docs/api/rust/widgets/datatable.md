# DataTable Widget

Advanced data table component with sorting, filtering, pagination, and virtual scrolling for large datasets.

## Overview

The DataTable widget provides a comprehensive solution for displaying tabular data with features like sorting, filtering, pagination, row selection, and virtual rendering for performance.

```rust
use reactive_tui::widgets::{DataTable, DataTableBuilder, Column, SortOrder};

let table = DataTableBuilder::new("users-table")
    .column(Column::new("id", "ID").width(60).sortable(true))
    .column(Column::new("name", "Name").width(150).sortable(true))
    .column(Column::new("email", "Email").width(200).sortable(true))
    .column(Column::new("created", "Created").width(100).sortable(true))
    .data(user_data)
    .sortable(true)
    .filterable(true)
    .paginated(true)
    .page_size(20)
    .build();
```

## DataTableBuilder

```rust
impl DataTableBuilder {
    pub fn new(id: &str) -> Self
    pub fn column(mut self, column: Column) -> Self
    pub fn data<T: TableData>(mut self, data: Vec<T>) -> Self
    pub fn sortable(mut self, sortable: bool) -> Self
    pub fn filterable(mut self, filterable: bool) -> Self
    pub fn paginated(mut self, paginated: bool) -> Self
    pub fn page_size(mut self, size: usize) -> Self
    pub fn selectable(mut self, selectable: bool) -> Self
    pub fn multi_select(mut self, multi_select: bool) -> Self
    pub fn virtual_scrolling(mut self, enabled: bool) -> Self
    pub fn fixed_header(mut self, fixed: bool) -> Self
    pub fn zebra_striping(mut self, enabled: bool) -> Self
    pub fn build(self) -> DataTable
}
```

## Column Configuration

```rust
pub struct Column {
    pub id: String,
    pub title: String,
    pub width: u16,
    pub min_width: Option<u16>,
    pub max_width: Option<u16>,
    pub sortable: bool,
    pub filterable: bool,
    pub resizable: bool,
    pub alignment: Alignment,
    pub formatter: Option<Box<dyn Fn(&str) -> String>>,
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Column {
    pub fn new(id: &str, title: &str) -> Self
    pub fn width(mut self, width: u16) -> Self
    pub fn min_width(mut self, min_width: u16) -> Self
    pub fn max_width(mut self, max_width: u16) -> Self
    pub fn sortable(mut self, sortable: bool) -> Self
    pub fn filterable(mut self, filterable: bool) -> Self
    pub fn resizable(mut self, resizable: bool) -> Self
    pub fn alignment(mut self, alignment: Alignment) -> Self
    pub fn formatter<F>(mut self, formatter: F) -> Self
    where F: Fn(&str) -> String + 'static
}
```

## Sorting

```rust
pub enum SortOrder {
    Ascending,
    Descending,
    None,
}

pub struct SortConfig {
    pub column_id: String,
    pub order: SortOrder,
}

// Usage
table.sort_by("name", SortOrder::Ascending);
table.clear_sort();
table.set_multi_sort(vec![
    SortConfig { column_id: "priority".to_string(), order: SortOrder::Descending },
    SortConfig { column_id: "name".to_string(), order: SortOrder::Ascending },
]);
```

## Filtering

```rust
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Between,
    In,
    NotIn,
}

pub struct Filter {
    pub column_id: String,
    pub operator: FilterOperator,
    pub value: String,
    pub case_sensitive: bool,
}

// Usage
table.add_filter(Filter {
    column_id: "status".to_string(),
    operator: FilterOperator::Equals,
    value: "active".to_string(),
    case_sensitive: false,
});

table.add_filter(Filter {
    column_id: "name".to_string(),
    operator: FilterOperator::Contains,
    value: "john".to_string(),
    case_sensitive: false,
});
```

## Selection

```rust
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}

// Usage
table.set_selection_mode(SelectionMode::Multiple);
table.select_row(5);
table.select_rows(vec![1, 3, 5]);
table.select_all();
table.clear_selection();

let selected_rows = table.get_selected_rows();
```

## Pagination

```rust
pub struct PaginationConfig {
    pub page_size: usize,
    pub show_page_info: bool,
    pub show_page_size_selector: bool,
    pub page_sizes: Vec<usize>,
}

// Usage
table.set_page_size(50);
table.go_to_page(3);
table.next_page();
table.previous_page();
table.first_page();
table.last_page();

let page_info = table.get_page_info();
println!("Page {} of {}", page_info.current_page, page_info.total_pages);
```

## Virtual Scrolling

```rust
// For large datasets, enable virtual scrolling
let large_table = DataTableBuilder::new("large-data")
    .virtual_scrolling(true)
    .virtual_item_height(1)
    .virtual_buffer_size(100)
    .data_provider(Box::new(DatabaseDataProvider::new(connection)))
    .build();
```

## Examples

### Basic Data Table

```rust
use reactive_tui::widgets::{DataTable, Column};

let data = vec![
    vec!["1".to_string(), "Alice".to_string(), "alice@example.com".to_string()],
    vec!["2".to_string(), "Bob".to_string(), "bob@example.com".to_string()],
    vec!["3".to_string(), "Charlie".to_string(), "charlie@example.com".to_string()],
];

let table = DataTableBuilder::new("simple-table")
    .column(Column::new("id", "ID").width(50))
    .column(Column::new("name", "Name").width(100))
    .column(Column::new("email", "Email").width(200))
    .data(data)
    .build();
```

### Advanced Table with All Features

```rust
let advanced_table = DataTableBuilder::new("advanced-table")
    .column(
        Column::new("id", "ID")
            .width(60)
            .sortable(true)
            .alignment(Alignment::Center)
    )
    .column(
        Column::new("name", "Name")
            .width(150)
            .sortable(true)
            .filterable(true)
            .resizable(true)
    )
    .column(
        Column::new("email", "Email")
            .width(200)
            .sortable(true)
            .filterable(true)
    )
    .column(
        Column::new("balance", "Balance")
            .width(100)
            .sortable(true)
            .alignment(Alignment::Right)
            .formatter(|value| {
                format!("${:.2}", value.parse::<f64>().unwrap_or(0.0))
            })
    )
    .column(
        Column::new("created", "Created")
            .width(120)
            .sortable(true)
            .formatter(|value| {
                format_date(value)
            })
    )
    .data(user_data)
    .sortable(true)
    .filterable(true)
    .paginated(true)
    .page_size(25)
    .selectable(true)
    .multi_select(true)
    .zebra_striping(true)
    .fixed_header(true)
    .build();
```

### Table with Custom Data Provider

```rust
use reactive_tui::widgets::{DataProvider, DataRequest, DataResponse};

struct CustomDataProvider {
    database: Database,
}

impl DataProvider for CustomDataProvider {
    fn fetch_data(&self, request: &DataRequest) -> DataResponse {
        let query = build_query(request);
        let results = self.database.execute(query).unwrap();
        
        DataResponse {
            data: results,
            total_count: self.database.count_all(),
            has_more: request.offset + request.limit < self.database.count_all(),
        }
    }
}

let table_with_provider = DataTableBuilder::new("db-table")
    .columns(columns)
    .data_provider(Box::new(CustomDataProvider { database }))
    .virtual_scrolling(true)
    .lazy_loading(true)
    .build();
```

### Reactive Data Table

```rust
use reactive_tui::{widgets::DataTable, reactive::Reactive};

let table_data = Reactive::new(load_initial_data());
let selected_rows = Reactive::new(Vec::<usize>::new());

let reactive_table = DataTableBuilder::new("reactive-table")
    .columns(columns)
    .data(table_data.get())
    .selectable(true)
    .multi_select(true)
    .on_selection_change({
        let selected = selected_rows.clone();
        move |new_selection| {
            selected.set(new_selection);
            Ok(())
        }
    })
    .on_row_double_click(|row_index, row_data| {
        open_detail_view(row_data);
        Ok(())
    })
    .build();

// Update data reactively
table_data.set(updated_data);
```

## CSS Styling

```css
.datatable {
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    overflow: hidden;
}

.datatable-header {
    background-color: #f9fafb;
    border-bottom: 1px solid #e5e7eb;
    font-weight: 600;
}

.datatable-header th {
    padding: 12px 8px;
    text-align: left;
    border-right: 1px solid #e5e7eb;
}

.datatable-header th.sortable {
    cursor: pointer;
}

.datatable-header th.sortable:hover {
    background-color: #f3f4f6;
}

.datatable-body {
    background-color: white;
}

.datatable-row {
    border-bottom: 1px solid #f3f4f6;
}

.datatable-row:hover {
    background-color: #f9fafb;
}

.datatable-row.selected {
    background-color: #dbeafe;
}

.datatable-row.zebra:nth-child(even) {
    background-color: #f9fafb;
}

.datatable-cell {
    padding: 12px 8px;
    border-right: 1px solid #f3f4f6;
}

.datatable-pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: #f9fafb;
    border-top: 1px solid #e5e7eb;
}
```

The DataTable widget provides enterprise-grade data display capabilities with comprehensive features for handling large datasets efficiently in terminal applications.