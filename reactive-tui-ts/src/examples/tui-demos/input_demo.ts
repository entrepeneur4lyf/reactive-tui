#!/usr/bin/env bun
/**
 * Input Widget Demo - TypeScript
 * 
 * Demonstrates various input types, validation, and features
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import {
  input, textInput, passwordInput, emailInput, numberInput,
  searchInput, phoneInput, urlInput, inputGroup
} from '../../packages/tui-bun/src/widgets/input';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface FormData {
  username: string;
  password: string;
  email: string;
  age: number;
  phone: string;
  website: string;
  search: string;
  description: string;
}

class InputDemo implements Component {
  private formData: FormData = {
    username: '',
    password: '',
    email: '',
    age: 0,
    phone: '',
    website: '',
    search: '',
    description: ''
  };
  
  private errors: Partial<Record<keyof FormData, string>> = {};
  private touched: Partial<Record<keyof FormData, boolean>> = {};
  private focusedField: keyof FormData | null = null;
  private showPassword = false;

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        // Cycle through fields
        const fields = Object.keys(this.formData) as (keyof FormData)[];
        const currentIndex = this.focusedField ? fields.indexOf(this.focusedField) : -1;
        this.focusedField = fields[(currentIndex + 1) % fields.length];
        return true;
      
      case 'p':
        this.showPassword = !this.showPassword;
        return true;
      
      case 'c':
        // Clear all fields
        Object.keys(this.formData).forEach(key => {
          (this.formData as any)[key] = key === 'age' ? 0 : '';
        });
        this.errors = {};
        this.touched = {};
        return true;
      
      case 'v':
        // Validate all fields
        this.validateAll();
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  validateField(field: keyof FormData, value: any): string | undefined {
    switch (field) {
      case 'username':
        if (!value) return 'Username is required';
        if (value.length < 3) return 'Username must be at least 3 characters';
        if (value.length > 20) return 'Username must be less than 20 characters';
        if (!/^[a-zA-Z0-9_]+$/.test(value)) return 'Username can only contain letters, numbers, and underscores';
        break;
      
      case 'password':
        if (!value) return 'Password is required';
        if (value.length < 8) return 'Password must be at least 8 characters';
        if (!/[A-Z]/.test(value)) return 'Password must contain an uppercase letter';
        if (!/[a-z]/.test(value)) return 'Password must contain a lowercase letter';
        if (!/[0-9]/.test(value)) return 'Password must contain a number';
        break;
      
      case 'email':
        if (!value) return 'Email is required';
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return 'Invalid email format';
        break;
      
      case 'age':
        if (!value || value < 1) return 'Age must be greater than 0';
        if (value > 150) return 'Please enter a valid age';
        break;
      
      case 'phone':
        if (value && !/^[\d\s\-+()]+$/.test(value)) return 'Invalid phone number format';
        break;
      
      case 'website':
        if (value && !/^https?:\/\/.+\..+/.test(value)) return 'Invalid URL format';
        break;
    }
  }

  validateAll() {
    Object.keys(this.formData).forEach(field => {
      const key = field as keyof FormData;
      this.errors[key] = this.validateField(key, this.formData[key]);
      this.touched[key] = true;
    });
  }

  handleChange(field: keyof FormData, value: any) {
    this.formData[field] = value;
    if (this.touched[field]) {
      this.errors[field] = this.validateField(field, value);
    }
  }

  handleBlur(field: keyof FormData) {
    this.touched[field] = true;
    this.errors[field] = this.validateField(field, this.formData[field]);
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('📝 Input Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text('Comprehensive input types with validation', { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Basic Text Input
            div({ class: 'mb-6' })
              .children([
                text('Text Input', { class: 'text-lg font-bold mb-2' }),
                textInput({
                  id: 'username',
                  label: 'Username',
                  placeholder: 'Enter username',
                  value: this.formData.username,
                  error: this.errors.username,
                  required: true,
                  focused: this.focusedField === 'username',
                  onChange: (value) => this.handleChange('username', value),
                  onBlur: () => this.handleBlur('username')
                })
              ]),
            
            // Password Input
            div({ class: 'mb-6' })
              .children([
                text('Password Input', { class: 'text-lg font-bold mb-2' }),
                passwordInput({
                  id: 'password',
                  label: 'Password',
                  placeholder: 'Enter password',
                  value: this.formData.password,
                  error: this.errors.password,
                  required: true,
                  showPassword: this.showPassword,
                  focused: this.focusedField === 'password',
                  onChange: (value) => this.handleChange('password', value),
                  onBlur: () => this.handleBlur('password')
                }),
                text('Press [P] to toggle password visibility', { class: 'text-sm text-gray-400 mt-1' })
              ]),
            
            // Email Input
            div({ class: 'mb-6' })
              .children([
                text('Email Input', { class: 'text-lg font-bold mb-2' }),
                emailInput({
                  id: 'email',
                  label: 'Email Address',
                  placeholder: 'user@example.com',
                  value: this.formData.email,
                  error: this.errors.email,
                  required: true,
                  focused: this.focusedField === 'email',
                  onChange: (value) => this.handleChange('email', value),
                  onBlur: () => this.handleBlur('email')
                })
              ]),
            
            // Number Input
            div({ class: 'mb-6' })
              .children([
                text('Number Input', { class: 'text-lg font-bold mb-2' }),
                numberInput({
                  id: 'age',
                  label: 'Age',
                  placeholder: 'Enter age',
                  value: this.formData.age,
                  error: this.errors.age,
                  min: 1,
                  max: 150,
                  focused: this.focusedField === 'age',
                  onChange: (value) => this.handleChange('age', value),
                  onBlur: () => this.handleBlur('age')
                })
              ]),
            
            // Phone Input
            div({ class: 'mb-6' })
              .children([
                text('Phone Input', { class: 'text-lg font-bold mb-2' }),
                phoneInput({
                  id: 'phone',
                  label: 'Phone Number',
                  placeholder: '+1 (555) 123-4567',
                  value: this.formData.phone,
                  error: this.errors.phone,
                  focused: this.focusedField === 'phone',
                  onChange: (value) => this.handleChange('phone', value),
                  onBlur: () => this.handleBlur('phone')
                })
              ]),
            
            // URL Input
            div({ class: 'mb-6' })
              .children([
                text('URL Input', { class: 'text-lg font-bold mb-2' }),
                urlInput({
                  id: 'website',
                  label: 'Website',
                  placeholder: 'https://example.com',
                  value: this.formData.website,
                  error: this.errors.website,
                  focused: this.focusedField === 'website',
                  onChange: (value) => this.handleChange('website', value),
                  onBlur: () => this.handleBlur('website')
                })
              ]),
            
            // Search Input
            div({ class: 'mb-6' })
              .children([
                text('Search Input', { class: 'text-lg font-bold mb-2' }),
                searchInput({
                  id: 'search',
                  placeholder: 'Search...',
                  value: this.formData.search,
                  focused: this.focusedField === 'search',
                  onChange: (value) => this.handleChange('search', value),
                  onClear: () => this.handleChange('search', '')
                })
              ]),
            
            // Input Group Demo
            div({ class: 'mb-6' })
              .children([
                text('Input Group', { class: 'text-lg font-bold mb-2' }),
                inputGroup({
                  label: 'Contact Information',
                  inputs: [
                    textInput({
                      id: 'first-name',
                      placeholder: 'First Name',
                      size: 'sm'
                    }),
                    textInput({
                      id: 'last-name',
                      placeholder: 'Last Name',
                      size: 'sm'
                    })
                  ]
                })
              ]),
            
            // Different Sizes
            div({ class: 'mb-6' })
              .children([
                text('Input Sizes', { class: 'text-lg font-bold mb-2' }),
                div({ class: 'space-y-2' })
                  .children([
                    input({ id: 'size-xs', placeholder: 'Extra Small', size: 'xs' }),
                    input({ id: 'size-sm', placeholder: 'Small', size: 'sm' }),
                    input({ id: 'size-md', placeholder: 'Medium (default)', size: 'md' }),
                    input({ id: 'size-lg', placeholder: 'Large', size: 'lg' }),
                    input({ id: 'size-xl', placeholder: 'Extra Large', size: 'xl' })
                  ])
              ]),
            
            // Different States
            div({ class: 'mb-6' })
              .children([
                text('Input States', { class: 'text-lg font-bold mb-2' }),
                div({ class: 'space-y-2' })
                  .children([
                    input({ id: 'normal', placeholder: 'Normal state' }),
                    input({ id: 'focused', placeholder: 'Focused state', focused: true }),
                    input({ id: 'disabled', placeholder: 'Disabled state', disabled: true }),
                    input({ id: 'readonly', placeholder: 'Readonly state', readOnly: true }),
                    input({ id: 'error-state', placeholder: 'Error state', error: 'This field has an error' }),
                    input({ id: 'success', placeholder: 'Success state', status: 'success' })
                  ])
              ])
          ]),
        
        // Footer with form data
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            text('Form Data:', { class: 'text-sm font-bold mb-1' }),
            text(JSON.stringify(this.formData, null, 2), { class: 'text-xs text-gray-400 font-mono' }),
            div({ class: 'mt-2 text-sm text-gray-500' })
              .child(text('[Tab] Navigate | [P] Toggle Password | [C] Clear | [V] Validate All | [Q] Quit'))
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new InputDemo().render(),
    fullscreen: true
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}