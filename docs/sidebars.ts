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
        {
          type: 'category',
          label: 'Widgets',
          items: [
            'api/rust/widgets',
            'api/rust/widgets/animation',
            'api/rust/widgets/accordion',
            'api/rust/widgets/autocomplete',
            'api/rust/widgets/bar',
            'api/rust/widgets/button',
            'api/rust/widgets/checkbox',
            'api/rust/widgets/datatable',
            'api/rust/widgets/form-validation',
            'api/rust/widgets/image',
            'api/rust/widgets/input',
            'api/rust/widgets/link',
            'api/rust/widgets/menu',
            'api/rust/widgets/modal',
            'api/rust/widgets/mouse',
            'api/rust/widgets/overlay',
            'api/rust/widgets/progress',
            'api/rust/widgets/radio',
            'api/rust/widgets/scrollable-list',
            'api/rust/widgets/select',
            'api/rust/widgets/slider',
            'api/rust/widgets/spinner',
            'api/rust/widgets/switch',
            'api/rust/widgets/tabs',
            'api/rust/widgets/textarea',
            'api/rust/widgets/toast',
            'api/rust/widgets/tree',
            'api/rust/widgets/viewport',
          ],
        },
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
        'api/typescript/overview',
        {
          type: 'category',
          label: 'Core',
          items: [
            'api/typescript/core/create-app',
            'api/typescript/core/element-builder',
            'api/typescript/core/widget-factory',
          ],
        },
        {
          type: 'category',
          label: 'Widgets',
          items: [
            'api/typescript/widgets/overview',
            {
              type: 'category',
              label: 'Layout',
              items: [
                'api/typescript/widgets/grid',
                'api/typescript/widgets/bar',
                'api/typescript/widgets/tabs',
                'api/typescript/widgets/modal',
                'api/typescript/widgets/accordion',
                'api/typescript/widgets/panel',
              ],
            },
            {
              type: 'category',
              label: 'Form Controls',
              items: [
                'api/typescript/widgets/button',
                'api/typescript/widgets/input',
                'api/typescript/widgets/checkbox',
                'api/typescript/widgets/radio',
                'api/typescript/widgets/select',
                'api/typescript/widgets/slider',
                'api/typescript/widgets/switch',
                'api/typescript/widgets/autocomplete',
              ],
            },
            {
              type: 'category',
              label: 'Data Display',
              items: [
                'api/typescript/widgets/datatable',
                'api/typescript/widgets/tree',
                'api/typescript/widgets/scrollable_list',
                'api/typescript/widgets/progress',
                'api/typescript/widgets/spinner',
              ],
            },
            {
              type: 'category',
              label: 'Content',
              items: [
                'api/typescript/widgets/rich_text',
                'api/typescript/widgets/viewport',
                'api/typescript/widgets/image',
                'api/typescript/widgets/link',
              ],
            },
            {
              type: 'category',
              label: 'Advanced',
              items: [
                'api/typescript/widgets/animation',
                'api/typescript/widgets/toast',
                'api/typescript/widgets/form_validation',
                'api/typescript/widgets/mouse',
                'api/typescript/widgets/hot_reload',
                'api/typescript/widgets/menu',
              ],
            },
          ],
        },
      ],
    },
  ],
};

export default sidebars;
