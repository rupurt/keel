import styles from './styles.module.css';

const backbone = [
  {
    lane: 'Direction',
    title: 'Mission',
    artifact: 'CHARTER.md',
    body: 'Holds the long-running objective and defines why the rest of the board exists.',
  },
  {
    lane: 'Problem framing',
    title: 'Epic',
    artifact: 'PRD.md',
    body: 'Turns the mission into a defined problem or opportunity with expected value.',
  },
  {
    lane: 'Planned passage',
    title: 'Voyage',
    artifact: 'SRS.md + SDD.md',
    body: 'Locks requirements and design constraints before execution is allowed to start.',
  },
  {
    lane: 'Verified move',
    title: 'Story',
    artifact: 'README.md',
    body: 'Carries the smallest executable slice that can be started, reviewed, accepted, and proved.',
  },
];

const supportSystems = [
  {
    title: 'Bearing',
    body: 'Discovery work that reduces fog before you commit to hard planning or implementation.',
  },
  {
    title: 'ADR',
    body: 'Architecture decisions that constrain the route so later stories do not re-litigate core choices.',
  },
  {
    title: 'Routine',
    body: 'Recurring board contracts that materialize work on schedule without inventing new scope.',
  },
];

export default function BoardSystemDiagram() {
  return (
    <section className={styles.wrap} aria-labelledby="board-system-map-title">
      <div className={styles.backboneRail}>
        <p className={styles.kicker}>Board system map</p>
        <h3 id="board-system-map-title" className={styles.title}>
          Direction, planning, execution, and proof stay on one connected path.
        </h3>
        <div className={styles.spine}>
          {backbone.map((entry) => (
            <article key={entry.title} className={styles.node}>
              <div className={styles.topline}>
                <p className={styles.lane}>{entry.lane}</p>
                <span className={styles.artifact}>{entry.artifact}</span>
              </div>
              <h4>{entry.title}</h4>
              <p>{entry.body}</p>
            </article>
          ))}
        </div>
      </div>

      <aside className={styles.supportDock}>
        <p className={styles.kicker}>Support systems</p>
        <p className={styles.supportLead}>
          These objects keep the main lane honest without flattening research,
          architecture, or scheduled upkeep into delivery stories.
        </p>
        <div className={styles.supportGrid}>
          {supportSystems.map((entry) => (
            <article key={entry.title} className={styles.supportCard}>
              <h4>{entry.title}</h4>
              <p>{entry.body}</p>
            </article>
          ))}
        </div>
      </aside>
    </section>
  );
}
