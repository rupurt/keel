import {useState} from 'react';
import type {CSSProperties, PointerEvent} from 'react';

import styles from './styles.module.css';

type TurnDividerProps = {
  arcSide?: 'left' | 'right';
  compact?: boolean;
  turns?: 2 | 3 | 4;
};

type StepPosition = {
  rotate: number;
  x: number;
  y: number;
};

type PointerState = {
  height: number;
  width: number;
  x: number;
  y: number;
};

const GRAVITY_RADIUS_FACTOR = 0.92;
const GRAVITY_SCALE_MAX = 1.9;

const STEP_PATTERNS: Record<2 | 3 | 4, StepPosition[]> = {
  2: [
    {x: 28, y: 22, rotate: 10},
    {x: 58, y: 68, rotate: 26},
  ],
  3: [
    {x: 24, y: 16, rotate: 8},
    {x: 56, y: 48, rotate: 20},
    {x: 36, y: 82, rotate: 30},
  ],
  4: [
    {x: 18, y: 12, rotate: 8},
    {x: 46, y: 32, rotate: 16},
    {x: 60, y: 58, rotate: 24},
    {x: 34, y: 84, rotate: 32},
  ],
};

export default function TurnDivider({
  arcSide = 'right',
  compact = false,
  turns = 3,
}: TurnDividerProps) {
  const [pointer, setPointer] = useState<PointerState | null>(null);
  const steps = STEP_PATTERNS[turns];
  const wrapClass = [
    styles.wrap,
    arcSide === 'left' ? styles.left : styles.right,
    compact ? styles.compact : '',
  ]
    .filter(Boolean)
    .join(' ');

  const handlePointerLeave = () => {
    setPointer(null);
  };

  const handlePointerMove = (event: PointerEvent<HTMLDivElement>) => {
    if (
      event.pointerType === 'touch' ||
      (typeof window !== 'undefined' &&
        window.matchMedia('(prefers-reduced-motion: reduce)').matches)
    ) {
      return;
    }

    const rect = event.currentTarget.getBoundingClientRect();

    setPointer({
      height: rect.height,
      width: rect.width,
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    });
  };

  return (
    <div aria-hidden="true" className={wrapClass}>
      <div
        className={styles.trail}
        onPointerLeave={handlePointerLeave}
        onPointerMove={handlePointerMove}>
        {steps.map((step, index) => {
          const x = arcSide === 'left' ? 100 - step.x : step.x;
          let cursorScale = 1;

          if (pointer) {
            const stepCenterX = (x / 100) * pointer.width;
            const stepCenterY = (step.y / 100) * pointer.height;
            const gravityRadius =
              Math.max(pointer.width, pointer.height) * GRAVITY_RADIUS_FACTOR;
            const distance = Math.hypot(
              pointer.x - stepCenterX,
              pointer.y - stepCenterY,
            );
            const proximity = Math.max(0, 1 - distance / gravityRadius);

            cursorScale =
              1 +
              Math.pow(proximity, 1.8) *
                (GRAVITY_SCALE_MAX - 1);
          }

          const style = {
            '--step-cursor-scale': cursorScale.toFixed(3),
            '--step-delay': `calc(${index} * var(--keel-motion-delay-turnstep))`,
            '--step-rotate': `${arcSide === 'left' ? -step.rotate : step.rotate}deg`,
            '--step-x': `${x}%`,
            '--step-y': `${step.y}%`,
          } as CSSProperties;

          return (
            <span
              key={`${turns}-${index}`}
              className={styles.step}
              data-step={index + 1}
              style={style}
            />
          );
        })}
      </div>
    </div>
  );
}
