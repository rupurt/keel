import Link from '@docusaurus/Link';

import styles from './styles.module.css';

const steps = [
  {
    label: 'Orient',
    command: 'keel health --scene',
    body: 'Read the board before acting. Check for drift, blocked work, and missing integrity.',
    href: '/docs/workflows/turn-loop#orient',
  },
  {
    label: 'Inspect',
    command: 'keel mission next --status',
    body: 'Understand what the system thinks matters now at the strategic and tactical level.',
    href: '/docs/workflows/turn-loop#inspect',
  },
  {
    label: 'Pull',
    command: 'keel next --role operator',
    body: 'Take one role-scoped move from the delivery lane instead of browsing an unbounded backlog.',
    href: '/docs/workflows/turn-loop#pull',
  },
  {
    label: 'Ship',
    command: 'keel story submit STORY-ID',
    body: 'Move the slice into review only when the implementation and its evidence are ready together.',
    href: '/docs/workflows/turn-loop#ship',
  },
  {
    label: 'Close',
    command: 'keel story accept --role manager STORY-ID',
    body: 'Accept the slice with an explicit role, then let the next turn begin from a clean board state.',
    href: '/docs/workflows/turn-loop#close',
  },
];

export default function TurnCycle() {
  return (
    <div className={styles.wrap}>
      {steps.map((step, index) => (
        <Link key={step.label} className={styles.cardLink} to={step.href}>
          <article className={styles.card}>
            <div className={styles.head}>
              <div className={styles.markerRow}>
                <span aria-hidden="true" className={styles.pulse} />
                <span className={styles.count}>0{index + 1}</span>
                <h3>{step.label}</h3>
              </div>
            </div>
            <code>{step.command}</code>
            <p>{step.body}</p>
          </article>
        </Link>
      ))}
    </div>
  );
}
