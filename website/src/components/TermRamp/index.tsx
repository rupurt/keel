import styles from './styles.module.css';

const terms = [
  {
    plain: 'Objective',
    keel: 'Mission',
    body: 'The long-running outcome you are trying to achieve across multiple efforts.',
  },
  {
    plain: 'Strategic track',
    keel: 'Epic',
    body: 'A major problem or opportunity inside the mission that deserves its own planning surface.',
  },
  {
    plain: 'Tactical campaign',
    keel: 'Voyage',
    body: 'A planned delivery arc with SRS and SDD constraints before execution begins.',
  },
  {
    plain: 'Executable slice',
    keel: 'Story',
    body: 'The smallest tracked unit that can be started, submitted, accepted, and evidenced.',
  },
  {
    plain: 'Research vector',
    keel: 'Bearing',
    body: 'A discovery move used to reduce fog before you freeze requirements or architecture.',
  },
  {
    plain: 'Scheduled contract',
    keel: 'Routine',
    body: 'Recurring work that pulse can materialize into the board without inventing new scope.',
  },
];

export default function TermRamp() {
  return (
    <div className={styles.grid}>
      {terms.map((term) => (
        <article key={term.keel} className={styles.card}>
          <p className={styles.plain}>{term.plain}</p>
          <div className={styles.arrow} />
          <h3>{term.keel}</h3>
          <p>{term.body}</p>
        </article>
      ))}
    </div>
  );
}
