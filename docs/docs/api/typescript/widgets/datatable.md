# DataTable Widget

The DataTable widget provides comprehensive data table functionality with sorting, filtering, pagination, row selection, virtual scrolling, column management, and extensive customization options. It supports large datasets with efficient rendering and includes convenience functions for common table patterns.

## Basic Usage

```typescript
import { 
  dataTable, 
  createColumn, 
  simpleDataTable,
  paginatedDataTable,
  ColumnAlignment 
} from 'reactive-tui';

// Basic data table
const userTable = dataTable({
  id: 'users-table',
  columns: [
    createColumn({
      id: 'name',
      title: 'Name',
      width: 200,
      sortable: true
    }),
    createColumn({
      id: 'email',
      title: 'Email',
      width: 250,
      sortable: true
    }),
    createColumn({
      id: 'role',
      title: 'Role',
      width: 120,
      alignment: ColumnAlignment.Center
    })
  ],
  data: [
    { name: 'John Doe', email: 'john@example.com', role: 'Admin' },
    { name: 'Jane Smith', email: 'jane@example.com', role: 'User' }
  ],
  config: {
    sortable: true,
    filterable: true,
    selectable: true,
    paginated: true
  }
});

// Simple table with convenience function
const simple = simpleDataTable(
  'simple-table',
  [
    { id: 'name', title: 'Name', width: 150 },
    { id: 'value', title: 'Value', width: 100 }
  ],
  [
    { name: 'Item 1', value: 100 },
    { name: 'Item 2', value: 200 }
  ]
);

// Paginated data table with 20 items per page
const paginated = paginatedDataTable(
  'paginated-table',
  [
    { id: 'id', title: 'ID', width: 60 },
    { id: 'name', title: 'Name', width: 200 },
    { id: 'status', title: 'Status', width: 100 }
  ],
  largeDataset,
  20
);

// Selectable data table with multi-select
const selectable = selectableDataTable(
  'selectable-table',
  [
    { id: 'name', title: 'Name', width: 150 },
    { id: 'email', title: 'Email', width: 250 }
  ],
  userData,
  true // Multi-select enabled
);

// Virtual data table for large datasets
const virtual = virtualDataTable(
  'virtual-table',
  [
    { id: 'id', title: 'ID', width: 80 },
    { id: 'content', title: 'Content', width: 300 }
  ],
  massiveDataset // 10k+ rows with virtual scrolling
);
```

## Types

### RowId and ColumnId

```typescript
export type RowId = string
export type ColumnId = string
```

### ColumnAlignment

```typescript
export enum ColumnAlignment {
  Left = 'left',
  Center = 'center',
  Right = 'right'
}
```

### SortOrder

```typescript
export enum SortOrder {
  None = 'none',
  Ascending = 'asc',
  Descending = 'desc'
}
```

## Configuration

### DataTableProps

```typescript
interface DataTableProps {
  id: string
  columns: DataTableColumn<any>[]
  data: any[]
  config?: Partial<DataTableConfig>
  callbacks?: DataTableCallbacks<any>
  cssClasses?: string[]
  style?: {
    backgroundColor?: string
    borderColor?: string
    headerBackgroundColor?: string
    headerTextColor?: string
    rowBackgroundColor?: string
    rowTextColor?: string
    selectedRowBackgroundColor?: string
    hoveredRowBackgroundColor?: string
  }
}
```

### DataTableConfig

```typescript
interface DataTableConfig {
  sortable: boolean
  filterable: boolean
  selectable: boolean
  multiSelect: boolean
  paginated: boolean
  virtualScrolling: boolean
  resizableColumns: boolean
  exportable: boolean
  showHeader: boolean
  showFooter: boolean
  striped: boolean
  bordered: boolean
  hover: boolean
  dense: boolean
  stickyHeader: boolean
  minColumnWidth: number
  maxColumnWidth: number
  rowHeight: number
  headerHeight: number
  footerHeight: number
}
```

### DataTableColumn

```typescript
interface DataTableColumn<T = any> {
  id: ColumnId
  title: string
  width: number
  sortable: boolean
  filterable: boolean
  resizable: boolean
  alignment: ColumnAlignment
  renderer?: (data: T, row: T) => string
  sorter?: (a: T, b: T) => number
  filter?: (data: T, query: string) => boolean
  cssClasses: string[]
}
```

### SortState

```typescript
interface SortState {
  primary?: { columnId: ColumnId; order: SortOrder }
  secondary: Array<{ columnId: ColumnId; order: SortOrder }>
}
```

### PaginationState

```typescript
interface PaginationState {
  currentPage: number
  pageSize: number
  totalPages: number
  totalRows: number
}
```

### DataTableState

```typescript
interface DataTableState {
  selectedRows: Set<RowId>
  focusedRow?: RowId
  sortState: SortState
  pagination: PaginationState
  searchQuery: string
  loading: boolean
  error?: string
}
```

### DataTableCallbacks

```typescript
interface DataTableCallbacks<T = any> {
  onRowSelect?: (rowId: RowId, row: T, selected: boolean) => void
  onRowAction?: (rowId: RowId, row: T, action: string) => void
  onSort?: (columnId: ColumnId, order: SortOrder) => void
  onFilter?: (query: string) => void
  onPageChange?: (page: number) => void
  onColumnResize?: (columnId: ColumnId, width: number) => void
  onExport?: (format: 'csv' | 'json', data: T[]) => void
}
```

## Examples

### Basic DataTable

```typescript
import { dataTable, createColumn } from 'reactive-tui-ts'

const basicTable = dataTable({
  id: 'basic-table',
  columns: [
    createColumn({
      id: 'id',
      title: 'ID',
      width: 80,
      alignment: ColumnAlignment.Right
    }),
    createColumn({
      id: 'name',
      title: 'Product Name',
      width: 200
    }),
    createColumn({
      id: 'price',
      title: 'Price',
      width: 100,
      alignment: ColumnAlignment.Right,
      renderer: (value) => `$${value.toFixed(2)}`
    })
  ],
  data: [
    { id: 1, name: 'Laptop', price: 999.99 },
    { id: 2, name: 'Mouse', price: 29.99 },
    { id: 3, name: 'Keyboard', price: 79.99 }
  ]
})

### Sortable DataTable

```typescript
const sortableTable = dataTable({
  id: 'sortable-table',
  columns: [
    createColumn({
      id: 'name',
      title: 'Name',
      width: 150,
      sortable: true
    }),
    createColumn({
      id: 'age',
      title: 'Age',
      width: 80,
      sortable: true,
      sorter: (a, b) => a.age - b.age,
      alignment: ColumnAlignment.Right
    }),
    createColumn({
      id: 'email',
      title: 'Email',
      width: 200,
      sortable: true
    })
  ],
  data: [
    { name: 'Alice', age: 30, email: 'alice@example.com' },
    { name: 'Bob', age: 25, email: 'bob@example.com' },
    { name: 'Charlie', age: 35, email: 'charlie@example.com' }
  ],
  config: {
    sortable: true,
    hover: true,
    striped: true
  },
  callbacks: {
    onSort: (columnId, order) => {
      console.log(`Sorted by ${columnId} in ${order} order`)
    }
  }
})

### Selectable DataTable

```typescript
const selectableTable = dataTable({
  id: 'selectable-table',
  columns: [
    createColumn({
      id: 'id',
      title: 'ID',
      width: 60,
      alignment: ColumnAlignment.Center
    }),
    createColumn({
      id: 'task',
      title: 'Task',
      width: 250
    }),
    createColumn({
      id: 'status',
      title: 'Status',
      width: 100,
      alignment: ColumnAlignment.Center,
      renderer: (value) => {
        const statusMap = {
          'completed': '✅',
          'pending': '⏳',
          'failed': '❌'
        }
        return statusMap[value] || value
      }
    })
  ],
  data: [
    { id: 1, task: 'Complete project documentation', status: 'completed' },
    { id: 2, task: 'Review code changes', status: 'pending' },
    { id: 3, task: 'Deploy to production', status: 'failed' }
  ],
  config: {
    selectable: true,
    multiSelect: true,
    striped: true,
    hover: true
  },
  callbacks: {
    onRowSelect: (rowId, row, selected) => {
      console.log(`Task ${row.id} ${selected ? 'selected' : 'deselected'}`)
    },
    onRowAction: (rowId, row, action) => {
      if (action === 'double-click') {
        console.log(`Opening task details for: ${row.task}`)
      }
    }
  }
})
```

## Convenience Functions

The DataTable widget provides four convenience functions for common table configurations:

### simpleDataTable

Creates a basic data table with sorting, filtering, and selection enabled:

```typescript
function simpleDataTable<T = any>(
  id: string,
  columns: Array<{ id: string; title: string; width?: number }>,
  data: T[]
): any
```

```typescript
import { simpleDataTable } from 'reactive-tui'

const productTable = simpleDataTable(
  'products-table',
  [
    { id: 'name', title: 'Product Name', width: 200 },
    { id: 'price', title: 'Price', width: 100 },
    { id: 'category', title: 'Category', width: 150 }
  ],
  [
    { name: 'Laptop', price: 999.99, category: 'Electronics' },
    { name: 'Book', price: 19.99, category: 'Education' },
    { name: 'Headphones', price: 149.99, category: 'Electronics' }
  ]
);
```

### paginatedDataTable

Creates a data table with pagination enabled:

```typescript
function paginatedDataTable<T = any>(
  id: string,
  columns: Array<{ id: string; title: string; width?: number }>,
  data: T[],
  pageSize: number = 10
): any
```

```typescript
import { paginatedDataTable } from 'reactive-tui'

const userTable = paginatedDataTable(
  'users-table',
  [
    { id: 'id', title: 'ID', width: 60 },
    { id: 'name', title: 'Full Name', width: 200 },
    { id: 'email', title: 'Email Address', width: 250 },
    { id: 'role', title: 'Role', width: 100 }
  ],
  largeUserDataset, // 1000+ users
  25 // 25 users per page
);
```

### selectableDataTable

Creates a data table with row selection capabilities:

```typescript
function selectableDataTable<T = any>(
  id: string,
  columns: Array<{ id: string; title: string; width?: number }>,
  data: T[],
  multiSelect: boolean = false
): any
```

```typescript
import { selectableDataTable } from 'reactive-tui'

// Single selection table
const taskTable = selectableDataTable(
  'tasks-table',
  [
    { id: 'title', title: 'Task Title', width: 300 },
    { id: 'status', title: 'Status', width: 100 },
    { id: 'priority', title: 'Priority', width: 100 }
  ],
  taskData,
  false // Single selection only
);

// Multi-selection table
const multiSelectTable = selectableDataTable(
  'multi-select-table',
  [
    { id: 'name', title: 'File Name', width: 250 },
    { id: 'size', title: 'Size', width: 100 },
    { id: 'modified', title: 'Modified', width: 150 }
  ],
  fileData,
  true // Multiple selection enabled
);
```

### virtualDataTable

Creates a data table with virtual scrolling for large datasets:

```typescript
function virtualDataTable<T = any>(
  id: string,
  columns: Array<{ id: string; title: string; width?: number }>,
  data: T[]
): any
```

```typescript
import { virtualDataTable } from 'reactive-tui'

const logTable = virtualDataTable(
  'logs-table',
  [
    { id: 'timestamp', title: 'Timestamp', width: 150 },
    { id: 'level', title: 'Level', width: 80 },
    { id: 'message', title: 'Message', width: 400 },
    { id: 'source', title: 'Source', width: 200 }
  ],
  logEntries // 100,000+ log entries
);
```

## Column Creation

### createColumn Helper

```typescript
import { createColumn, ColumnAlignment } from 'reactive-tui-ts'

const nameColumn = createColumn({
  id: 'name',
  title: 'Full Name',
  width: 200,
  sortable: true,
  filterable: true,
  alignment: ColumnAlignment.Left
})

const priceColumn = createColumn({
  id: 'price',
  title: 'Price',
  width: 100,
  sortable: true,
  alignment: ColumnAlignment.Right,
  renderer: (value) => `$${value.toFixed(2)}`,
  sorter: (a, b) => a.price - b.price
})

const statusColumn = createColumn({
  id: 'status',
  title: 'Status',
  width: 120,
  alignment: ColumnAlignment.Center,
  filter: (row, query) => {
    return row.status.toLowerCase() === query.toLowerCase()
  },
  renderer: (value) => {
    const statusIcons = {
      'active': '🟢',
      'inactive': '🔴',
      'pending': '🟡'
    }
    return `${statusIcons[value] || '⚫'} ${value}`
  }
})
```

## Pagination

### Pagination Configuration

```typescript
interface PaginationConfig {
  pageSize: number              // Items per page
  currentPage?: number          // Current page (0-based)
  showPageInfo?: boolean        // Show "Page X of Y"
  showPageSizeSelector?: boolean // Show page size dropdown
  pageSizeOptions?: number[]    // Available page sizes
  showFirstLast?: boolean       // Show first/last buttons
  showPrevNext?: boolean        // Show prev/next buttons
}
```

```typescript
const paginatedTable = dataTable({
  id: 'paginated',
  columns: columns,
  data: largeDataset,
  pagination: {
    pageSize: 20,
    currentPage: 0,
    showPageInfo: true,
    showPageSizeSelector: true,
    pageSizeOptions: [10, 20, 50, 100],
    showFirstLast: true,
    showPrevNext: true
  }
})
```

## Sorting and Filtering

### Multi-Column Sorting

```typescript
const advancedSortingTable = dataTable({
  id: 'advanced-sorting',
  columns: [
    createColumn({
      id: 'name',
      title: 'Name',
      width: 200,
      sortable: true
    }),
    createColumn({
      id: 'date',
      title: 'Date',
      width: 120,
      sortable: true,
      renderer: (value) => new Date(value).toLocaleDateString(),
      sorter: (a, b) => new Date(a.date).getTime() - new Date(b.date).getTime()
    }),
    createColumn({
      id: 'amount',
      title: 'Amount',
      width: 120,
      sortable: true,
      alignment: ColumnAlignment.Right,
      renderer: (value) => `$${value.toFixed(2)}`,
      sorter: (a, b) => a.amount - b.amount
    })
  ],
  data: transactionData,
  config: {
    sortable: true
  },
  callbacks: {
    onSort: (columnId, order) => {
      console.log(`Sorted by ${columnId} in ${order} order`);
      // Update sort indicators in UI
      updateSortIndicators(columnId, order);
    }
  }
});
```

### Advanced Filtering

```typescript
const filterableTable = dataTable({
  id: 'filterable-products',
  columns: [
    createColumn({
      id: 'name',
      title: 'Product Name',
      width: 200,
      filterable: true,
      filter: (row, query) => {
        return row.name.toLowerCase().includes(query.toLowerCase());
      }
    }),
    createColumn({
      id: 'category',
      title: 'Category',
      width: 150,
      filterable: true,
      filter: (row, query) => {
        return row.category.toLowerCase() === query.toLowerCase();
      }
    }),
    createColumn({
      id: 'price',
      title: 'Price',
      width: 100,
      filterable: true,
      alignment: ColumnAlignment.Right,
      renderer: (value) => `$${value.toFixed(2)}`,
      filter: (row, query) => {
        const price = parseFloat(row.price);
        const range = query.split('-').map(v => parseFloat(v.trim()));
        if (range.length === 2) {
          return price >= range[0] && price <= range[1];
        }
        return price.toString().includes(query);
      }
    }),
    createColumn({
      id: 'inStock',
      title: 'In Stock',
      width: 100,
      filterable: true,
      renderer: (value) => value ? '✅ Yes' : '❌ No',
      filter: (row, query) => {
        const queryLower = query.toLowerCase();
        if (queryLower === 'yes' || queryLower === 'true') return row.inStock;
        if (queryLower === 'no' || queryLower === 'false') return !row.inStock;
        return true;
      }
    })
  ],
  data: productData,
  config: {
    filterable: true,
    sortable: true
  },
  callbacks: {
    onFilter: (query) => {
      console.log('Filter applied:', query);
      // Update search statistics
      updateSearchStats(query);
    }
  }
});
```

## Virtual Scrolling

For large datasets (10,000+ rows), enable virtual scrolling for optimal performance:

```typescript
const virtualTable = dataTable({
  id: 'virtual-logs-table',
  columns: [
    createColumn({
      id: 'timestamp',
      title: 'Timestamp',
      width: 180,
      sortable: true,
      renderer: (value) => new Date(value).toLocaleString()
    }),
    createColumn({
      id: 'level',
      title: 'Level',
      width: 80,
      sortable: true,
      renderer: (value) => {
        const levelColors = {
          'ERROR': '🔴 ERROR',
          'WARN': '🟡 WARN',
          'INFO': '🔵 INFO',
          'DEBUG': '⚪ DEBUG'
        };
        return levelColors[value] || value;
      }
    }),
    createColumn({
      id: 'message',
      title: 'Message',
      width: 400,
      filterable: true
    }),
    createColumn({
      id: 'source',
      title: 'Source',
      width: 150,
      filterable: true
    })
  ],
  data: logEntries, // 100,000+ log entries
  config: {
    virtualScrolling: true,
    sortable: true,
    filterable: true,
    paginated: true,
    rowHeight: 32,
    headerHeight: 40
  },
  callbacks: {
    onRowAction: (rowId, row, action) => {
      if (action === 'double-click') {
        showLogDetails(row);
      }
    }
  }
});
```

## Column Resizing

```typescript
const resizableTable = dataTable({
  id: 'resizable-table',
  columns: [
    createColumn({
      id: 'name',
      title: 'Name',
      width: 200,
      resizable: true
    }),
    createColumn({
      id: 'description',
      title: 'Description',
      width: 300,
      resizable: true
    })
  ],
  data: data,
  config: {
    resizableColumns: true,
    minColumnWidth: 50,
    maxColumnWidth: 500
  },
  callbacks: {
    onColumnResize: (columnId, width) => {
      console.log(`Column ${columnId} resized to ${width}px`);
      // Save column widths to preferences
      saveColumnWidth(columnId, width);
    }
  }
});
```

## Export Functionality

```typescript
const exportableTable = dataTable({
  id: 'exportable-table',
  columns: columns,
  data: data,
  config: {
    exportable: true
  },
  callbacks: {
    onExport: (format, data) => {
      if (format === 'csv') {
        downloadCSV(data);
      } else if (format === 'json') {
        downloadJSON(data);
      }
    }
  }
});

// Helper functions for export
function downloadCSV(data: any[]) {
  const csv = convertToCSV(data);
  const blob = new Blob([csv], { type: 'text/csv' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'data.csv';
  a.click();
}

function downloadJSON(data: any[]) {
  const json = JSON.stringify(data, null, 2);
  const blob = new Blob([json], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'data.json';
  a.click();
}

## Event Handling

### `DataTableCallbacks`

```typescript
interface DataTableCallbacks {
  onRowSelect?: (row: any, selected: boolean) => void
  onSelectionChange?: (selectedRows: any[]) => void
  onRowDoubleClick?: (row: any) => void
  onSort?: (column: ColumnId, order: SortOrder) => void
  onFilter?: (filters: Record<string, any>) => void
  onPageChange?: (page: number) => void
  onPageSizeChange?: (pageSize: number) => void
  onRowExpand?: (row: any) => void
  onCellEdit?: (row: any, column: ColumnId, newValue: any) => void
}
```

```typescript
const interactiveTable = dataTable({
  id: 'interactive',
  columns: columns,
  data: data,
  selectable: true,
  callbacks: {
    onRowSelect: (row, selected) => {
      console.log(`Row ${row.id} ${selected ? 'selected' : 'deselected'}`)
    },
    
    onSelectionChange: (selectedRows) => {
      console.log(`${selectedRows.length} rows selected`)
      updateBulkActions(selectedRows)
    },
    
    onRowDoubleClick: (row) => {
      openDetailView(row)
    },
    
    onSort: (column, order) => {
      // Custom sorting logic
      sortData(column, order)
    },
    
    onFilter: (filters) => {
      // Custom filtering logic
      applyFilters(filters)
    }
  }
})
```

## Real-World Examples

### Complete User Management System

```typescript
import { dataTable, createColumn, simpleDataTable, selectableDataTable } from 'reactive-tui'

class UserManagementSystem {
  private users = [
    { 
      id: 1, 
      name: 'John Doe', 
      email: 'john@example.com', 
      role: 'Admin', 
      status: 'Active', 
      lastLogin: '2024-01-15T10:30:00Z',
      permissions: ['read', 'write', 'delete']
    },
    { 
      id: 2, 
      name: 'Jane Smith', 
      email: 'jane@example.com', 
      role: 'User', 
      status: 'Active', 
      lastLogin: '2024-01-14T15:45:00Z',
      permissions: ['read']
    },
    { 
      id: 3, 
      name: 'Bob Johnson', 
      email: 'bob@example.com', 
      role: 'Moderator', 
      status: 'Inactive', 
      lastLogin: '2024-01-10T08:20:00Z',
      permissions: ['read', 'write']
    }
  ];

  private selectedUsers: any[] = [];
  private tableInstance: any;

  constructor() {
    this.setupTable();
  }

  private setupTable() {
    this.tableInstance = dataTable({
      id: 'users-management-table',
      columns: [
        createColumn({
          id: 'id',
          title: 'ID',
          width: 80,
          alignment: ColumnAlignment.Right,
          sortable: true
        }),
        createColumn({
          id: 'name',
          title: 'Full Name',
          width: 200,
          sortable: true,
          filterable: true,
          renderer: (value, row) => {
            const statusIcon = row.status === 'Active' ? '🟢' : '🔴';
            return `${statusIcon} ${value}`;
          }
        }),
        createColumn({
          id: 'email',
          title: 'Email Address',
          width: 250,
          sortable: true,
          filterable: true
        }),
        createColumn({
          id: 'role',
          title: 'Role',
          width: 120,
          filterable: true,
          renderer: (value) => {
            const roleIcons = {
              'Admin': '👑 Admin',
              'Moderator': '🛡️ Moderator',
              'User': '👤 User'
            };
            return roleIcons[value] || value;
          },
          filter: (row, query) => {
            return row.role.toLowerCase().includes(query.toLowerCase());
          }
        }),
        createColumn({
          id: 'status',
          title: 'Status',
          width: 100,
          filterable: true,
          alignment: ColumnAlignment.Center,
          renderer: (value) => {
            return value === 'Active' ? '✅ Active' : '❌ Inactive';
          }
        }),
        createColumn({
          id: 'lastLogin',
          title: 'Last Login',
          width: 150,
          sortable: true,
          renderer: (value) => {
            const date = new Date(value);
            const now = new Date();
            const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));
            
            if (diffDays === 0) return '🕐 Today';
            if (diffDays === 1) return '📅 Yesterday';
            if (diffDays <= 7) return `📅 ${diffDays} days ago`;
            return `📅 ${date.toLocaleDateString()}`;
          },
          sorter: (a, b) => {
            return new Date(a.lastLogin).getTime() - new Date(b.lastLogin).getTime();
          }
        }),
        createColumn({
          id: 'permissions',
          title: 'Permissions',
          width: 150,
          renderer: (value) => {
            return value.map((perm: string) => {
              const permIcons = {
                'read': '👁️',
                'write': '✏️',
                'delete': '🗑️'
              };
              return permIcons[perm] || perm;
            }).join(' ');
          }
        })
      ],
      data: this.users,
      config: {
        sortable: true,
        filterable: true,
        selectable: true,
        multiSelect: true,
        paginated: true,
        striped: true,
        hover: true,
        bordered: true
      },
      callbacks: {
        onRowSelect: (rowId, row, selected) => {
          if (selected) {
            this.selectedUsers.push(row);
          } else {
            this.selectedUsers = this.selectedUsers.filter(u => u.id !== row.id);
          }
          this.updateSelectionUI();
        },
        onRowAction: (rowId, row, action) => {
          if (action === 'double-click') {
            this.editUser(row);
          }
        },
        onSort: (columnId, order) => {
          console.log(`Sorted by ${columnId} in ${order} order`);
          this.logUserAction(`Sorted users by ${columnId}`);
        },
        onFilter: (query) => {
          console.log(`Filtered users with query: ${query}`);
          this.logUserAction(`Filtered users: "${query}"`);
        },
        onPageChange: (page) => {
          console.log(`Navigated to page ${page + 1}`);
        }
      }
    });
  }

  // User management operations
  addUser(userData: any) {
    const newUser = {
      id: Math.max(...this.users.map(u => u.id)) + 1,
      ...userData,
      lastLogin: new Date().toISOString()
    };
    
    this.users.push(newUser);
    this.refreshTable();
    this.logUserAction(`Added user: ${newUser.name}`);
  }

  editUser(user: any) {
    // Open edit dialog/form
    console.log('Editing user:', user);
    this.logUserAction(`Opened edit form for user: ${user.name}`);
  }

  deleteUser(userId: number) {
    const user = this.users.find(u => u.id === userId);
    if (user) {
      this.users = this.users.filter(u => u.id !== userId);
      this.refreshTable();
      this.logUserAction(`Deleted user: ${user.name}`);
    }
  }

  bulkDelete() {
    const deletedNames = this.selectedUsers.map(u => u.name);
    const selectedIds = this.selectedUsers.map(u => u.id);
    
    this.users = this.users.filter(u => !selectedIds.includes(u.id));
    this.selectedUsers = [];
    this.refreshTable();
    this.logUserAction(`Bulk deleted users: ${deletedNames.join(', ')}`);
  }

  toggleUserStatus(userId: number) {
    const user = this.users.find(u => u.id === userId);
    if (user) {
      user.status = user.status === 'Active' ? 'Inactive' : 'Active';
      this.refreshTable();
      this.logUserAction(`Toggled status for user: ${user.name} to ${user.status}`);
    }
  }

  exportUsers(format: 'csv' | 'json') {
    if (format === 'csv') {
      this.exportToCSV();
    } else {
      this.exportToJSON();
    }
    this.logUserAction(`Exported users as ${format.toUpperCase()}`);
  }

  // UI update methods
  private updateSelectionUI() {
    const selectionCount = this.selectedUsers.length;
    const selectionInfo = document.getElementById('selection-info');
    if (selectionInfo) {
      selectionInfo.textContent = selectionCount > 0 
        ? `${selectionCount} user(s) selected`
        : 'No users selected';
    }
    
    // Enable/disable bulk actions
    const bulkDeleteBtn = document.getElementById('bulk-delete-btn');
    if (bulkDeleteBtn) {
      bulkDeleteBtn.disabled = selectionCount === 0;
    }
  }

  private refreshTable() {
    // Update table data (in a real implementation, this would trigger a re-render)
    console.log('Table refreshed with updated user data');
  }

  private logUserAction(action: string) {
    const timestamp = new Date().toLocaleString();
    console.log(`[${timestamp}] User Management: ${action}`);
  }

  private exportToCSV() {
    const headers = ['ID', 'Name', 'Email', 'Role', 'Status', 'Last Login'];
    const csvData = this.users.map(user => [
      user.id,
      user.name,
      user.email,
      user.role,
      user.status,
      new Date(user.lastLogin).toLocaleDateString()
    ]);
    
    const csvContent = [headers, ...csvData]
      .map(row => row.map(field => `"${field}"`).join(','))
      .join('\n');
    
    // Download CSV file
    this.downloadFile(csvContent, 'users.csv', 'text/csv');
  }

  private exportToJSON() {
    const jsonData = JSON.stringify(this.users, null, 2);
    this.downloadFile(jsonData, 'users.json', 'application/json');
  }

  private downloadFile(content: string, filename: string, mimeType: string) {
    const blob = new Blob([content], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  // Getters for UI integration
  getTable() {
    return this.tableInstance;
  }

  getSelectedUsers() {
    return this.selectedUsers;
  }

  getUserCount() {
    return this.users.length;
  }

  getActiveUserCount() {
    return this.users.filter(u => u.status === 'Active').length;
  }
}

// Usage
const userManagement = new UserManagementSystem();
const userTable = userManagement.getTable();
```

### E-commerce Product Catalog

```typescript
class ProductCatalogTable {
  private products = [
    {
      id: 'LAPTOP001',
      name: 'MacBook Pro 16"',
      category: 'Electronics',
      price: 2499.99,
      stock: 15,
      rating: 4.8,
      tags: ['laptop', 'apple', 'professional'],
      inStock: true,
      featured: true,
      lastUpdated: '2024-01-15T10:30:00Z'
    },
    {
      id: 'BOOK001',
      name: 'Clean Code',
      category: 'Books',
      price: 49.99,
      stock: 25,
      rating: 4.9,
      tags: ['programming', 'software-development'],
      inStock: true,
      featured: false,
      lastUpdated: '2024-01-14T15:45:00Z'
    }
    // ... more products
  ];

  constructor() {
    this.setupProductTable();
  }

  private setupProductTable() {
    return dataTable({
      id: 'product-catalog',
      columns: [
        createColumn({
          id: 'featured',
          title: '⭐',
          width: 40,
          sortable: true,
          renderer: (value) => value ? '⭐' : '',
          sorter: (a, b) => (b.featured ? 1 : 0) - (a.featured ? 1 : 0)
        }),
        createColumn({
          id: 'name',
          title: 'Product Name',
          width: 250,
          sortable: true,
          filterable: true,
          renderer: (value, row) => {
            const stockIcon = row.inStock ? '✅' : '❌';
            return `${stockIcon} ${value}`;
          }
        }),
        createColumn({
          id: 'category',
          title: 'Category',
          width: 120,
          filterable: true,
          renderer: (value) => {
            const categoryIcons = {
              'Electronics': '💻',
              'Books': '📚',
              'Clothing': '👕',
              'Home': '🏠'
            };
            return `${categoryIcons[value] || '📦'} ${value}`;
          }
        }),
        createColumn({
          id: 'price',
          title: 'Price',
          width: 100,
          sortable: true,
          alignment: ColumnAlignment.Right,
          renderer: (value) => `$${value.toFixed(2)}`,
          sorter: (a, b) => a.price - b.price,
          filter: (row, query) => {
            // Support price range filtering: "100-500"
            if (query.includes('-')) {
              const [min, max] = query.split('-').map(v => parseFloat(v.trim()));
              return row.price >= min && row.price <= max;
            }
            return row.price.toString().includes(query);
          }
        }),
        createColumn({
          id: 'stock',
          title: 'Stock',
          width: 80,
          sortable: true,
          alignment: ColumnAlignment.Right,
          renderer: (value, row) => {
            if (value === 0) return '❌ Out';
            if (value <= 5) return `⚠️ ${value}`;
            return `✅ ${value}`;
          },
          sorter: (a, b) => a.stock - b.stock
        }),
        createColumn({
          id: 'rating',
          title: 'Rating',
          width: 100,
          sortable: true,
          alignment: ColumnAlignment.Center,
          renderer: (value) => {
            const stars = '★'.repeat(Math.floor(value)) + '☆'.repeat(5 - Math.floor(value));
            return `${stars} ${value.toFixed(1)}`;
          },
          sorter: (a, b) => b.rating - a.rating
        }),
        createColumn({
          id: 'tags',
          title: 'Tags',
          width: 200,
          filterable: true,
          renderer: (value) => {
            return value.map((tag: string) => `#${tag}`).join(' ');
          },
          filter: (row, query) => {
            return row.tags.some((tag: string) => 
              tag.toLowerCase().includes(query.toLowerCase())
            );
          }
        })
      ],
      data: this.products,
      config: {
        sortable: true,
        filterable: true,
        selectable: true,
        multiSelect: true,
        paginated: true,
        virtualScrolling: false,
        striped: true,
        hover: true,
        exportable: true
      },
      callbacks: {
        onRowAction: (rowId, row, action) => {
          if (action === 'double-click') {
            this.viewProductDetails(row);
          }
        },
        onExport: (format, data) => {
          this.exportProducts(format, data);
        }
      }
    });
  }

  private viewProductDetails(product: any) {
    console.log('Viewing product details:', product);
    // Open product detail modal/page
  }

  private exportProducts(format: 'csv' | 'json', data: any[]) {
    console.log(`Exporting ${data.length} products as ${format}`);
    // Implement export logic
  }
}
```

## CSS Styling

```css
/* Table container */
.datatable {
  border: 1px solid #d1d5db;
  border-radius: 8px;
  overflow: hidden;
  background: white;
}

/* Table header */
.datatable-header {
  background: #f9fafb;
  border-bottom: 1px solid #d1d5db;
}

.datatable-header-cell {
  padding: 0.75rem;
  font-weight: 600;
  text-align: left;
  border-right: 1px solid #e5e7eb;
  cursor: pointer;
  user-select: none;
}

.datatable-header-cell:hover {
  background: #f3f4f6;
}

.datatable-header-cell.sortable {
  position: relative;
}

.datatable-sort-icon {
  margin-left: 0.5rem;
  opacity: 0.5;
}

.datatable-header-cell.sorted .datatable-sort-icon {
  opacity: 1;
}

/* Table body */
.datatable-body {
  max-height: 400px;
  overflow-y: auto;
}

.datatable-row {
  border-bottom: 1px solid #e5e7eb;
  transition: background-color 0.15s ease;
}

.datatable-row:hover {
  background: #f9fafb;
}

.datatable-row.selected {
  background: #eff6ff;
}

.datatable-cell {
  padding: 0.75rem;
  border-right: 1px solid #e5e7eb;
  vertical-align: middle;
}

/* Status indicators */
.status-active {
  color: #059669;
  font-weight: 500;
}

.status-inactive {
  color: #dc2626;
  font-weight: 500;
}

.role-admin {
  color: #7c3aed;
  font-weight: 500;
}

/* Pagination */
.datatable-pagination {
  display: flex;
  align-items: center;
  justify-content: between;
  padding: 1rem;
  background: #f9fafb;
  border-top: 1px solid #d1d5db;
}

.pagination-info {
  color: #6b7280;
  font-size: 0.875rem;
}

.pagination-controls {
  display: flex;
  gap: 0.5rem;
}
```

## Best Practices

### 1. Use Appropriate Column Types

```typescript
// ✅ Good - semantic column configuration
const columns = [
  createColumn('id', 'ID', { width: 80, alignment: 'right' }),
  createColumn('name', 'Name', { sortable: true, filterable: true }),
  createColumn('price', 'Price', { 
    alignment: 'right',
    formatter: (value) => `$${value.toFixed(2)}`
  })
]
```

### 2. Implement Virtual Scrolling for Large Datasets

```typescript
// ✅ Good - virtual scrolling for performance
const largeTable = dataTable({
  id: 'large-data',
  data: massiveDataset,
  virtualScrolling: true,
  rowHeight: 40
})
```

### 3. Provide Clear Action Feedback

```typescript
// ✅ Good - clear user feedback
const table = dataTable({
  id: 'feedback-table',
  callbacks: {
    onSelectionChange: (rows) => {
      showSelectionCount(rows.length)
    },
    onSort: (column, order) => {
      showSortingIndicator(column, order)
    }
  }
})
```

## Related Widgets

- **[Tree](./tree)** - Hierarchical data display
- **[ScrollableList](./scrollable-list)** - Simple list display
- **[Pagination](./pagination)** - Standalone pagination
- **[Input](./input)** - Table filtering inputs

## Examples

- **[User Management](../../examples/advanced/data-table)** - Complete CRUD interface
- **[Product Catalog](../../examples/apps/product-catalog)** - E-commerce table
- **[Log Viewer](../../examples/apps/log-viewer)** - Large dataset handling
