import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Rust Performance',
    Svg: require('@site/static/img/undraw_docusaurus_mountain.svg').default,
    description: (
      <>
        Built on a high-performance Rust core with NAPI bindings for lightning-fast 
        terminal rendering and responsive interactions.
      </>
    ),
  },
  {
    title: 'CSS-Styled TUIs',
    Svg: require('@site/static/img/undraw_docusaurus_tree.svg').default,
    description: (
      <>
        Use familiar CSS styling with flexbox, grid layouts, animations, and themes 
        to create beautiful terminal interfaces.
      </>
    ),
  },
  {
    title: 'Rich Widget Library',
    Svg: require('@site/static/img/undraw_docusaurus_react.svg').default,
    description: (
      <>
        25+ pre-built components including DataTable, Modal, Tabs, Forms, and more
        with full TypeScript support and reactive state management.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
