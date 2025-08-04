import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  // Main documentation sidebar
  tutorialSidebar: [
    'intro',
    'core-concepts',
    'rust-guide',
    {
      type: 'category',
      label: 'Tutorial',
      items: [
        'tutorial/getting-started',
      ],
    },
  ],

  // API Reference sidebar
  apiSidebar: [
    'api/overview',
    {
      type: 'category',
      label: 'Rust API',
      items: [
        'api/rust/app',
        'api/rust/reactive',
        'api/rust/widgets',
        'api/rust/css',
        'api/rust/events',
        'api/rust/layout',
        'api/rust/components',
        'api/rust/rendering',
        'api/rust/themes',
        'api/rust/error',
        'api/rust/driver',
        'api/rust/display',
        'api/rust/screens',
        'api/rust/plugin',
        'api/rust/prelude',
        'api/rust/compat',
      ],
    },
    {
      type: 'category', 
      label: 'TypeScript API',
      items: [
        'api/widgets',
        'api/css',
        'api/state',
        'api/plugins',
      ],
    },
  ],
};

export default sidebars;
