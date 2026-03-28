import Link from '@docusaurus/Link';
import {magneticFieldEvents} from '@site/src/components/magneticField';

import styles from './styles.module.css';

const terms = [
  {
    plain: 'Objective',
    keel: 'Mission',
    body: 'The long-running outcome you are trying to achieve across multiple efforts.',
    href: '/docs/foundations/board-vocabulary#mission',
  },
  {
    plain: 'Strategic track',
    keel: 'Epic',
    body: 'A major problem or opportunity inside the mission that deserves its own planning surface.',
    href: '/docs/foundations/board-vocabulary#epic',
  },
  {
    plain: 'Tactical campaign',
    keel: 'Voyage',
    body: 'A planned delivery arc with SRS and SDD constraints before execution begins.',
    href: '/docs/foundations/board-vocabulary#voyage',
  },
  {
    plain: 'Executable slice',
    keel: 'Story',
    body: 'The smallest tracked unit that can be started, submitted, accepted, and evidenced.',
    href: '/docs/foundations/board-vocabulary#story',
  },
  {
    plain: 'Research vector',
    keel: 'Bearing',
    body: 'A discovery move used to reduce fog before you freeze requirements or architecture.',
    href: '/docs/foundations/board-vocabulary#bearing',
  },
  {
    plain: 'Scheduled contract',
    keel: 'Routine',
    body: 'Recurring work that pulse can materialize into the board without inventing new scope.',
    href: '/docs/foundations/board-vocabulary#routine',
  },
];

export default function TermRamp() {
  return (
    <div className={styles.grid}>
      {terms.map((term) => (
        <Link
          key={term.keel}
          className={styles.cardLink}
          to={term.href}
          {...magneticFieldEvents<HTMLAnchorElement>()}>
          <article className={styles.card}>
            <div className={styles.plainBlock}>
              <p className={styles.plainLabel}>Everyday language</p>
              <p className={styles.plain}>{term.plain}</p>
            </div>
            <div className={styles.arrow} aria-hidden="true" />
            <div className={styles.keelBlock}>
              <p className={styles.keelLabel}>Keel term</p>
              <h3>{term.keel}</h3>
              <p>{term.body}</p>
            </div>
          </article>
        </Link>
      ))}
    </div>
  );
}
