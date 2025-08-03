/**
 * Theme Validation Demo
 * 
 * Demonstrates the comprehensive validation system for JSON themes.
 * Shows various error scenarios and how validation helps catch issues.
 */

import { 
    validateJSONTheme, validateJSONThemeCollection, validateJSONColor,
    sanitizeThemeName, loadThemeFromFile
} from '../../packages/tui-bun/src/index';

// Test data with various validation scenarios
const validTheme = {
    name: "test-theme",
    description: "A valid test theme",
    mode: "rgb",
    palette: {
        primary: { hex: "#007ACC" },
        background: { r: 40, g: 44, b: 52 },
        text: { rgb: [255, 255, 255] },
        border: { ansi: 8 }
    },
    semantic: {
        panelBackground: "background",
        panelBorder: "border", 
        panelTitle: "text",
        panelContent: "text"
    }
};

const invalidThemes = [
    {
        name: "missing-description",
        // Missing description
        palette: {
            primary: { hex: "#007ACC" }
        }
    },
    {
        name: "invalid name with spaces!",
        description: "Theme with invalid name",
        palette: {
            primary: { hex: "INVALID_HEX" }, // Invalid hex
            background: { r: 300, g: -50, b: 500 }, // Invalid RGB values
            text: { ansi: 300 } // Invalid ANSI code
        }
    },
    {
        name: "missing-required-colors",
        description: "Theme missing required colors",
        palette: {
            secondary: { hex: "#00FF00" }
            // Missing primary, background, text, border
        }
    },
    {
        name: "invalid-semantic-mappings",
        description: "Theme with invalid semantic mappings",
        palette: {
            primary: { hex: "#007ACC" },
            background: { hex: "#282C34" },
            text: { hex: "#FFFFFF" },
            border: { hex: "#444444" }
        },
        semantic: {
            panelBackground: "nonexistent-color", // References unknown color
            panelBorder: 123, // Wrong type
            // Missing required mappings
        }
    },
    {
        name: "conflicting-color-formats",
        description: "Theme with conflicting color formats",
        palette: {
            primary: { 
                hex: "#007ACC",
                rgb: [0, 122, 204], // Multiple formats - not allowed
                r: 0, g: 122, b: 204
            },
            background: { hex: "#282C34" },
            text: { hex: "#FFFFFF" },
            border: { hex: "#444444" }
        }
    }
];

const validCollection = {
    version: "1.0.0",
    namedColors: {
        "brand-blue": { hex: "#007ACC" },
        "brand-red": { hex: "#FF0000" }
    },
    themes: [
        {
            name: "theme-1",
            description: "First theme in collection",
            palette: {
                primary: { name: "brand-blue" },
                background: { hex: "#282C34" },
                text: { hex: "#FFFFFF" },
                border: { hex: "#444444" }
            }
        },
        {
            name: "theme-2", 
            description: "Second theme in collection",
            palette: {
                primary: { name: "brand-red" },
                background: { hex: "#1E1E1E" },
                text: { hex: "#FFFFFF" },
                border: { hex: "#666666" }
            }
        }
    ]
};

const invalidCollections = [
    {
        // Missing themes array
        version: "1.0.0"
    },
    {
        themes: [] // Empty themes array
    },
    {
        themes: "not-an-array" // Wrong type
    },
    {
        themes: [
            validTheme,
            { ...validTheme, name: validTheme.name } // Duplicate names
        ]
    },
    {
        namedColors: {
            "invalid-color": { hex: "NOT_HEX" }
        },
        themes: [validTheme]
    }
];

function runValidationDemo() {
    console.log('Theme Validation Demo');
    console.log('====================');
    console.log('');

    // Test individual color validation
    console.log('1. Color Validation Tests');
    console.log('-------------------------');
    
    const colorTests = [
        { name: "valid-hex", color: { hex: "#FF5733" } },
        { name: "valid-rgb-array", color: { rgb: [255, 87, 51] } },
        { name: "valid-rgb-object", color: { r: 255, g: 87, b: 51 } },
        { name: "valid-ansi", color: { ansi: 196 } },
        { name: "valid-named", color: { name: "brand-blue" } },
        { name: "invalid-hex", color: { hex: "ZZZ" } },
        { name: "invalid-rgb-range", color: { r: 300, g: -50, b: 500 } },
        { name: "invalid-ansi-range", color: { ansi: 300 } },
        { name: "conflicting-formats", color: { hex: "#FF0000", rgb: [255, 0, 0] } },
        { name: "empty-color", color: {} }
    ];
    
    colorTests.forEach(test => {
        const errors = validateJSONColor(test.color, test.name);
        if (errors.length === 0) {
            console.log(`✓ ${test.name}: Valid`);
        } else {
            console.log(`✗ ${test.name}: ${errors.join(', ')}`);
        }
    });
    
    console.log('');

    // Test theme validation
    console.log('2. Theme Validation Tests');
    console.log('-------------------------');
    
    console.log('Valid theme:');
    const validErrors = validateJSONTheme(validTheme);
    if (validErrors.length === 0) {
        console.log('✓ Theme is valid');
    } else {
        console.log(`✗ Validation errors: ${validErrors.join(', ')}`);
    }
    
    console.log('');
    console.log('Invalid themes:');
    invalidThemes.forEach((theme, index) => {
        console.log(`\nTheme ${index + 1} (${theme.name}):`);
        const errors = validateJSONTheme(theme);
        if (errors.length === 0) {
            console.log('✓ Unexpectedly valid');
        } else {
            errors.forEach(error => console.log(`  ✗ ${error}`));
        }
    });
    
    console.log('');

    // Test collection validation
    console.log('3. Collection Validation Tests');
    console.log('------------------------------');
    
    console.log('Valid collection:');
    const validCollectionErrors = validateJSONThemeCollection(validCollection);
    if (validCollectionErrors.length === 0) {
        console.log('✓ Collection is valid');
    } else {
        console.log(`✗ Validation errors: ${validCollectionErrors.join(', ')}`);
    }
    
    console.log('');
    console.log('Invalid collections:');
    invalidCollections.forEach((collection, index) => {
        console.log(`\nCollection ${index + 1}:`);
        const errors = validateJSONThemeCollection(collection);
        if (errors.length === 0) {
            console.log('✓ Unexpectedly valid');
        } else {
            errors.forEach(error => console.log(`  ✗ ${error}`));
        }
    });
    
    console.log('');

    // Test theme name sanitization
    console.log('4. Theme Name Sanitization Tests');
    console.log('--------------------------------');
    
    const nameTests = [
        "Valid-Theme_Name",
        "Invalid Theme Name!",
        "  spaced   name  ",
        "UPPERCASE-THEME",
        "theme.with.dots",
        "theme@with#symbols$",
        "",
        "   ",
        "123-numeric-start"
    ];
    
    nameTests.forEach(name => {
        try {
            const sanitized = sanitizeThemeName(name);
            console.log(`"${name}" → "${sanitized}"`);
        } catch (error) {
            console.log(`"${name}" → Error: ${error}`);
        }
    });
    
    console.log('');

    // Test real file validation
    console.log('5. File Loading Validation');
    console.log('---------------------------');
    
    const testFiles = [
        '../themes/dracula.json',
        '../themes/collection.json'
    ];
    
    testFiles.forEach(file => {
        try {
            console.log(`Loading ${file}...`);
            const theme = loadThemeFromFile(file);
            console.log(`✓ Successfully loaded: ${theme.name}`);
        } catch (error) {
            console.log(`✗ Failed to load: ${error}`);
        }
    });
    
    console.log('');
    console.log('Validation Demo Complete');
    console.log('========================');
    console.log('');
    console.log('Key validation features:');
    console.log('• Comprehensive color format validation (hex, rgb, ansi, named)');
    console.log('• Required field checking');
    console.log('• Type safety for all fields');
    console.log('• Semantic mapping validation');
    console.log('• Collection integrity checks');
    console.log('• Theme name sanitization');
    console.log('• Detailed error messages with context');
    console.log('• Graceful error handling with partial loading');
}

// Example of creating a validation utility function
export function validateThemeFile(filePath: string): { isValid: boolean; errors: string[]; warnings: string[] } {
    const result = {
        isValid: false,
        errors: [] as string[],
        warnings: [] as string[]
    };
    
    try {
        const fs = require('fs');
        const fileContent = fs.readFileSync(filePath, 'utf-8');
        
        let jsonData: any;
        try {
            jsonData = JSON.parse(fileContent);
        } catch (parseError) {
            result.errors.push(`Invalid JSON syntax: ${parseError}`);
            return result;
        }
        
        // Determine if it's a single theme or collection
        if ('themes' in jsonData) {
            const errors = validateJSONThemeCollection(jsonData);
            result.errors.push(...errors);
        } else {
            const errors = validateJSONTheme(jsonData);
            result.errors.push(...errors);
        }
        
        // Add warnings for missing optional fields
        if (!('version' in jsonData)) {
            result.warnings.push('Consider adding a version field');
        }
        
        if (!('author' in jsonData)) {
            result.warnings.push('Consider adding an author field');
        }
        
        result.isValid = result.errors.length === 0;
        
    } catch (error) {
        result.errors.push(`Failed to read file: ${error}`);
    }
    
    return result;
}

// Run the demo
if (require.main === module) {
    runValidationDemo();
}