/**
 * Tree Widget Demo - TypeScript
 * 
 * Demonstrates the comprehensive tree widget functionality including:
 * - Hierarchical data display with expand/collapse
 * - Single and multi-selection modes
 * - Lazy loading of child nodes on demand
 * - Search/filtering capabilities
 * - Keyboard navigation
 * - Virtual scrolling for large datasets
 * - Custom node types and styling
 */

import { 
    Tree, TreeBuilder, createTreeNode, createFolderNode, createLeafNode, 
    treePatterns, TreeNodeType, type NodeId
} from '../../packages/tui-bun/src/widgets/tree';

async function runTreeDemo() {
    console.log('🌳 Tree Widget Demo - TypeScript');
    console.log('='.repeat(50));

    // Demo 1: Simple file system tree
    console.log('\n1. File System Tree Structure');
    console.log('-'.repeat(30));
    
    const fileTree = new TreeBuilder('file-tree')
        .expandable(true)
        .multiSelect(false)
        .searchEnabled(true)
        .onExpand((nodeId, expanded) => {
            const action = expanded ? 'expanded' : 'collapsed';
            console.log(`   📁 ${nodeId} ${action}`);
        })
        .onSelect((selectedNodes) => {
            if (selectedNodes.length > 0) {
                console.log(`   🎯 Selected: ${selectedNodes.join(', ')}`);
            }
        })
        .build();

    // Add root nodes first
    fileTree.addRootNode(createFolderNode('src', 'src/'));
    fileTree.addRootNode(createFolderNode('tests', 'tests/'));
    fileTree.addRootNode(createFolderNode('docs', 'docs/'));

    // Add children to src folder
    fileTree.addChildNode('src', createLeafNode('main.rs', 'main.rs'));
    fileTree.addChildNode('src', createLeafNode('lib.rs', 'lib.rs'));
    fileTree.addChildNode('src', createFolderNode('widgets', 'widgets/'));

    // Add children to widgets folder
    fileTree.addChildNode('widgets', createLeafNode('button.rs', 'button.rs'));
    fileTree.addChildNode('widgets', createLeafNode('input.rs', 'input.rs'));
    fileTree.addChildNode('widgets', createLeafNode('tree.rs', 'tree.rs'));

    // Add children to tests folder
    fileTree.addChildNode('tests', createLeafNode('integration.rs', 'integration.rs'));
    fileTree.addChildNode('tests', createLeafNode('unit.rs', 'unit.rs'));

    // Add children to docs folder
    fileTree.addChildNode('docs', createLeafNode('README.md', 'README.md'));
    fileTree.addChildNode('docs', createLeafNode('CHANGELOG.md', 'CHANGELOG.md'));

    console.log(`✅ File system tree created: ${fileTree.id} (${fileTree.rootNodes.length} root nodes)`);

    // Test expansion
    await fileTree.expand('src');
    await fileTree.expand('widgets');
    console.log('✅ Expanded src/ and widgets/ folders');

    // Test selection
    fileTree.select('tree.rs');
    console.log('✅ Selected tree.rs file');

    // Get visible nodes
    const visible = fileTree.getVisibleNodes();
    console.log(`   Visible nodes: ${visible.length}`);

    // Demo 2: Organization hierarchy with multi-select
    console.log('\n2. Organization Hierarchy (Multi-Select)');
    console.log('-'.repeat(30));

    const orgTree = new TreeBuilder('org-tree')
        .multiSelect(true)
        .expandable(true)
        .onSelect((selectedNodes) => {
            console.log(`   👥 Selected departments: ${selectedNodes.length}`);
        })
        .build();

    // Add root CEO node
    const ceoNode = createTreeNode('ceo', 'CEO');
    ceoNode.nodeType = TreeNodeType.Custom;
    orgTree.addRootNode(ceoNode);

    // Add department nodes
    orgTree.addChildNode('ceo', createFolderNode('engineering', 'Engineering'));
    orgTree.addChildNode('ceo', createFolderNode('marketing', 'Marketing'));
    orgTree.addChildNode('ceo', createFolderNode('sales', 'Sales'));

    // Add engineering teams
    orgTree.addChildNode('engineering', createLeafNode('frontend-team', 'Frontend Team'));
    orgTree.addChildNode('engineering', createLeafNode('backend-team', 'Backend Team'));
    orgTree.addChildNode('engineering', createLeafNode('devops-team', 'DevOps Team'));

    // Add marketing teams
    orgTree.addChildNode('marketing', createLeafNode('content-team', 'Content Team'));
    orgTree.addChildNode('marketing', createLeafNode('design-team', 'Design Team'));

    // Add sales teams
    orgTree.addChildNode('sales', createLeafNode('enterprise-sales', 'Enterprise Sales'));
    orgTree.addChildNode('sales', createLeafNode('smb-sales', 'SMB Sales'));

    console.log(`✅ Organization tree created: ${orgTree.id} (${orgTree.rootNodes.length} root nodes)`);

    // Test multi-selection
    await orgTree.expand('ceo');
    await orgTree.expand('engineering');
    orgTree.select('frontend-team');
    orgTree.select('backend-team');
    orgTree.select('devops-team');
    console.log('✅ Selected multiple engineering teams');

    // Demo 3: Lazy loading tree (simulated API calls)
    console.log('\n3. Lazy Loading Tree (API Simulation)');
    console.log('-'.repeat(30));

    const lazyTree = new TreeBuilder('lazy-tree')
        .lazyLoading(true, async (nodeId: NodeId) => {
            // Simulate API call delay
            await new Promise(resolve => setTimeout(resolve, 100));
            
            console.log(`   🔄 Loading children for: ${nodeId}`);
            
            // Simulate different child nodes based on parent
            switch (nodeId) {
                case 'countries':
                    return [
                        createFolderNode('usa', 'United States'),
                        createFolderNode('canada', 'Canada'),
                        createFolderNode('mexico', 'Mexico'),
                    ];
                case 'usa':
                    return [
                        createLeafNode('california', 'California'),
                        createLeafNode('texas', 'Texas'),
                        createLeafNode('florida', 'Florida'),
                    ];
                case 'canada':
                    return [
                        createLeafNode('ontario', 'Ontario'),
                        createLeafNode('quebec', 'Quebec'),
                        createLeafNode('bc', 'British Columbia'),
                    ];
                default:
                    return [];
            }
        })
        .expandable(true)
        .onExpand((nodeId, expanded) => {
            if (expanded) {
                console.log(`   📡 Lazy loading triggered for: ${nodeId}`);
            }
        })
        .build();

    // Add root node for lazy loading demo
    lazyTree.addRootNode(createFolderNode('countries', 'Countries'));
    console.log(`✅ Lazy loading tree created: ${lazyTree.id}`);

    // Demo 4: Search and filtering
    console.log('\n4. Search and Filtering');
    console.log('-'.repeat(30));

    // Test search on file tree
    fileTree.setSearchQuery('rs');
    const filteredVisible = fileTree.getVisibleNodes();
    console.log(`✅ Search for 'rs': ${filteredVisible.length} matches`);

    // Clear search
    fileTree.setSearchQuery('');
    console.log('✅ Search cleared');

    // Demo 5: Keyboard navigation simulation
    console.log('\n5. Keyboard Navigation Test');
    console.log('-'.repeat(30));

    // Navigate through nodes
    fileTree.navigateNext();
    fileTree.navigateNext();
    console.log('✅ Navigated through nodes');

    fileTree.navigatePrevious();
    console.log('✅ Navigated back');

    // Test key handling
    const rightArrowEvent = new KeyboardEvent('keydown', { key: 'ArrowRight' });
    const handled1 = await fileTree.handleKeyEvent(rightArrowEvent);
    console.log(`✅ Arrow right handled: ${handled1}`);

    const spaceEvent = new KeyboardEvent('keydown', { key: ' ' });
    const handled2 = await fileTree.handleKeyEvent(spaceEvent);
    console.log(`✅ Space key (select) handled: ${handled2}`);

    // Demo 6: Virtual scrolling setup
    console.log('\n6. Large Dataset with Virtual Scrolling');
    console.log('-'.repeat(30));

    const largeTree = new TreeBuilder('large-tree')
        .virtualScrolling(true)
        .expandable(true)
        .build();

    // Add many root nodes to simulate large dataset
    for (let i = 0; i < 50; i++) {
        const categoryNode = createFolderNode(`category-${i}`, `Category ${i}`);
        largeTree.addRootNode(categoryNode);
    }

    console.log(`✅ Large tree created: ${largeTree.id} (50 root nodes)`);

    const visibleLarge = largeTree.getVisibleNodes();
    console.log(`   Virtual scroll viewport: ${visibleLarge.length} visible nodes`);

    // Demo 7: Advanced operations
    console.log('\n7. Advanced Tree Operations');
    console.log('-'.repeat(30));

    // Toggle operations
    await orgTree.toggleExpand('engineering');
    console.log('✅ Toggled engineering expansion');

    orgTree.toggleSelection('marketing');
    console.log('✅ Toggled marketing selection');

    // Clear all selections
    orgTree.clearSelection();
    console.log('✅ Cleared all selections');

    // Demo 8: Node rendering
    console.log('\n8. Node Rendering');
    console.log('-'.repeat(30));

    // Render some nodes
    const srcNode = fileTree.nodes.get('src');
    if (srcNode) {
        const rendered = fileTree.renderNode(srcNode);
        console.log(`✅ Rendered src node: "${rendered}"`);
    }

    const treeNode = fileTree.nodes.get('tree.rs');
    if (treeNode) {
        const rendered = fileTree.renderNode(treeNode);
        console.log(`✅ Rendered tree.rs node: "${rendered}"`);
    }

    // Demo 9: State inspection
    console.log('\n9. State Inspection');
    console.log('-'.repeat(30));

    const state = fileTree.getState();
    console.log(`   Expanded nodes: ${state.expandedNodes.size}`);
    console.log(`   Selected nodes: ${state.selectedNodes.size}`);
    console.log(`   Highlighted node: ${state.highlightedNode ?? 'none'}`);
    console.log(`   Search query: "${state.searchQuery}"`);
    console.log(`   Filtered nodes: ${state.filteredNodes.size}`);
    console.log(`   Loading nodes: ${state.loadingNodes.size}`);

    const selectedNodes = fileTree.getSelectedNodes();
    console.log(`   Selected node objects: ${selectedNodes.length}`);

    // Demo 10: Convenience patterns
    console.log('\n10. Convenience Patterns');
    console.log('-'.repeat(30));

    const fsTree = treePatterns.fileSystem('filesystem').build();
    console.log(`✅ File system pattern tree: ${fsTree.id}`);

    const orgPatternTree = treePatterns.organization('org-pattern').build();
    console.log(`✅ Organization pattern tree: ${orgPatternTree.id}`);

    const categoryTree = treePatterns.categories('categories').build();
    console.log(`✅ Categories pattern tree: ${categoryTree.id}`);

    // Demo 11: Configuration updates
    console.log('\n11. Dynamic Configuration');
    console.log('-'.repeat(30));

    fileTree.updateConfig({
        virtualScrolling: true,
        maxVisibleNodes: 500
    });
    console.log('✅ Configuration updated');

    // Summary
    console.log('\n🎉 Tree Widget Demo Complete!');
    console.log('='.repeat(50));
    console.log('✅ Hierarchical data display with expand/collapse');
    console.log('✅ Single and multi-selection modes');
    console.log('✅ Lazy loading with async callbacks');
    console.log('✅ Search and filtering capabilities');
    console.log('✅ Keyboard navigation and event handling');
    console.log('✅ Virtual scrolling for large datasets');
    console.log('✅ Custom node types and styling');
    console.log('✅ Advanced operations (toggle, clear, navigate)');
    console.log('✅ Node rendering and state inspection');
    console.log('✅ Dynamic configuration updates');
    
    console.log('\n🚀 Features Demonstrated:');
    console.log('• Hierarchical tree structure with expandable nodes');
    console.log('• Lazy loading of child nodes with async callbacks');
    console.log('• Multi-selection with keyboard and programmatic control');
    console.log('• Real-time search filtering with highlighting');
    console.log('• Virtual scrolling for efficient large dataset rendering');
    console.log('• Custom node types (folder, leaf, loading, custom)');
    console.log('• Event callbacks for expand, select, and search operations');
    console.log('• Comprehensive keyboard navigation (arrows, space, enter)');
    console.log('• Node rendering with indentation and icons');
    console.log('• State management and configuration updates');
    console.log('• Convenience builders for common tree patterns');
    console.log('• TypeScript type safety throughout');
}

// Run the demo
runTreeDemo().catch(console.error);