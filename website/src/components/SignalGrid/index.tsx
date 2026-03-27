import Link from '@docusaurus/Link';

import styles from './styles.module.css';

type SignalItem = {
  eyebrow?: string;
  title: string;
  body: string;
  href?: string;
  cta?: string;
};

type SignalGridProps = {
  items: SignalItem[];
  columns?: 'two' | 'three';
};

export default function SignalGrid({
  items,
  columns = 'three',
}: SignalGridProps) {
  return (
    <div
      className={`${styles.grid} ${
        columns === 'two' ? styles.twoColumns : styles.threeColumns
      }`}>
      {items.map((item) => {
        const content = (
          <>
            {item.eyebrow ? <p className={styles.eyebrow}>{item.eyebrow}</p> : null}
            <h3>{item.title}</h3>
            <p>{item.body}</p>
            {item.href ? (
              <span className={styles.linkText}>{item.cta ?? 'Read more'}</span>
            ) : null}
          </>
        );

        if (item.href) {
          return (
            <Link key={item.title} className={styles.cardLink} to={item.href}>
              <article className={styles.card}>{content}</article>
            </Link>
          );
        }

        return (
          <article key={item.title} className={styles.card}>
            {content}
          </article>
        );
      })}
    </div>
  );
}
