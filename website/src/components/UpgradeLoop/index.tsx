import styles from './styles.module.css';

type UpgradeStep = {
  label: string;
  title: string;
  body: string;
  command?: string;
};

type UpgradeLoopProps = {
  steps: UpgradeStep[];
};

export default function UpgradeLoop({steps}: UpgradeLoopProps) {
  return (
    <div className={styles.wrap}>
      {steps.map((step, index) => (
        <article key={step.title} className={styles.card}>
          <div className={styles.head}>
            <span className={styles.count}>0{index + 1}</span>
            <p className={styles.label}>{step.label}</p>
          </div>
          <h3>{step.title}</h3>
          {step.command ? <code>{step.command}</code> : null}
          <p>{step.body}</p>
        </article>
      ))}
    </div>
  );
}
