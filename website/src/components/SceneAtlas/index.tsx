import {magneticFieldEvents} from '@site/src/components/magneticField';

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

function toAnchorId(value: string) {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

export default function SceneAtlas({scenes}: SceneAtlasProps) {
  return (
    <div className={styles.grid}>
      {scenes.map((entry) => (
        <a
          key={entry.command}
          className={styles.card}
          href={`#${toAnchorId(entry.scene)}`}
          {...magneticFieldEvents<HTMLAnchorElement>()}
        >
          <div className={styles.topline}>
            <p className={styles.scene}>{entry.scene}</p>
            <span className={styles.jump}>Jump to detail</span>
          </div>
          <code className={styles.command}>{entry.command}</code>
          <div className={styles.meta}>
            <span className={styles.label}>Reads</span>
            <p>{entry.signal}</p>
          </div>
          <div className={styles.meta}>
            <span className={styles.label}>Use When</span>
            <p>{entry.when}</p>
          </div>
        </a>
      ))}
    </div>
  );
}
