import styles from './styles.module.css';

type SceneSurface = {
  command: string;
  scene: string;
  signal: string;
  when: string;
};

type SceneAtlasProps = {
  scenes: SceneSurface[];
};

export default function SceneAtlas({scenes}: SceneAtlasProps) {
  return (
    <div className={styles.grid}>
      {scenes.map((entry) => (
        <article key={entry.command} className={styles.card}>
          <p className={styles.scene}>{entry.scene}</p>
          <code className={styles.command}>{entry.command}</code>
          <div className={styles.meta}>
            <span className={styles.label}>Reads</span>
            <p>{entry.signal}</p>
          </div>
          <div className={styles.meta}>
            <span className={styles.label}>Use When</span>
            <p>{entry.when}</p>
          </div>
        </article>
      ))}
    </div>
  );
}
