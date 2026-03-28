import styles from './styles.module.css';

type TurnDividerProps = {
  arcSide?: 'left' | 'right';
  compact?: boolean;
  turns?: 2 | 3;
};

const ARC_HEXES = 4;

export default function TurnDivider({
  arcSide = 'right',
  compact = false,
  turns = 3,
}: TurnDividerProps) {
  const wrapClass = [
    styles.wrap,
    arcSide === 'left' ? styles.left : styles.right,
    compact ? styles.compact : '',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <div aria-hidden="true" className={wrapClass}>
      <div className={styles.track} />
      <div className={styles.arc}>
        {Array.from({length: ARC_HEXES}, (_, index) => (
          <span key={index} className={styles.arcHex} />
        ))}
      </div>
      <div className={styles.center}>
        {Array.from({length: turns}, (_, index) => (
          <span key={index} className={styles.centerHex} />
        ))}
      </div>
    </div>
  );
}
