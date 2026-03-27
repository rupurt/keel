import styles from './styles.module.css';

type BoundaryColumn = {
  eyebrow: string;
  title: string;
  items: string[];
};

type ContractBoundaryProps = {
  upstream: BoundaryColumn;
  downstream: BoundaryColumn;
  seamTitle?: string;
  seamBody?: string;
};

function BoundaryColumnCard({column}: {column: BoundaryColumn}) {
  return (
    <article className={styles.column}>
      <p className={styles.eyebrow}>{column.eyebrow}</p>
      <h3>{column.title}</h3>
      <ul className={styles.list}>
        {column.items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </article>
  );
}

export default function ContractBoundary({
  upstream,
  downstream,
  seamTitle = 'Adapt at the seam',
  seamBody = 'Copy upstream guidance first, then change only the parts that need to describe your repo, command wrappers, proof surfaces, and local operating constraints.',
}: ContractBoundaryProps) {
  return (
    <div className={styles.wrap}>
      <BoundaryColumnCard column={upstream} />
      <aside className={styles.seam}>
        <p className={styles.seamLabel}>Downstream seam</p>
        <h3>{seamTitle}</h3>
        <p>{seamBody}</p>
      </aside>
      <BoundaryColumnCard column={downstream} />
    </div>
  );
}
