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

export default function SceneGallery({scenes}: SceneGalleryProps) {
  return (
    <div className={styles.stack}>
      {scenes.map((entry) => (
        <article key={entry.command} className={styles.card}>
          <div className={styles.header}>
            <div className={styles.identity}>
              <p className={styles.scene}>{entry.scene}</p>
              <code className={styles.command}>{entry.command}</code>
            </div>
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
