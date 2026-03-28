import styles from './styles.module.css';

type SceneState = {
  label: string;
  detail: string;
};

type SceneSurface = {
  command: string;
  scene: string;
  lead: string;
  art: string;
  reads: string[];
  states: SceneState[];
};

type SceneGalleryProps = {
  scenes: SceneSurface[];
};

function toAnchorId(value: string) {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

export default function SceneGallery({scenes}: SceneGalleryProps) {
  return (
    <div className={styles.stack}>
      {scenes.map((entry) => (
        <article
          key={entry.command}
          id={toAnchorId(entry.scene)}
          className={styles.card}
        >
          <div className={styles.header}>
            <div className={styles.identity}>
              <p className={styles.scene}>{entry.scene}</p>
              <h3 className={styles.title}>{entry.scene}</h3>
              <code className={styles.command}>{entry.command}</code>
            </div>
            <a className={styles.backLink} href="#scene-atlas">
              Back to atlas
            </a>
            <p className={styles.lead}>{entry.lead}</p>
          </div>

          <div className={styles.terminalFrame}>
            <p className={styles.terminalLabel}>Representative Output</p>
            <pre className={styles.terminal}>
              <code>{entry.art}</code>
            </pre>
          </div>

          <div className={styles.metaGrid}>
            <section className={styles.metaCard}>
              <p className={styles.label}>What It Reads</p>
              <ul className={styles.list}>
                {entry.reads.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
            </section>

            <section className={styles.metaCard}>
              <p className={styles.label}>State Model</p>
              <ul className={styles.list}>
                {entry.states.map((state) => (
                  <li key={state.label}>
                    <strong>{state.label}</strong> {state.detail}
                  </li>
                ))}
              </ul>
            </section>
          </div>
        </article>
      ))}
    </div>
  );
}
