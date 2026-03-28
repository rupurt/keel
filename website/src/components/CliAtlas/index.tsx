import styles from './styles.module.css';

type CliFamily = {
  eyebrow: string;
  title: string;
  body: string;
  commands: string[];
};

type CliAtlasProps = {
  families: CliFamily[];
};

export default function CliAtlas({families}: CliAtlasProps) {
  return (
    <div className={styles.grid}>
      {families.map((family) => (
        <article key={family.title} className={styles.card}>
          <p className={styles.eyebrow}>{family.eyebrow}</p>
          <h3>{family.title}</h3>
          <p>{family.body}</p>
          <ul className={styles.commandList}>
            {family.commands.map((command) => (
              <li key={command} className={styles.commandPill}>
                <code>{command}</code>
              </li>
            ))}
          </ul>
        </article>
      ))}
    </div>
  );
}
