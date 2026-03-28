import type {PointerEventHandler} from 'react';

const MAGNETIC_MAX_PULL_PX = 16;
const MAGNETIC_FIELD_EXPONENT = 1.6;

function prefersReducedMotion() {
  return (
    typeof window !== 'undefined' &&
    window.matchMedia('(prefers-reduced-motion: reduce)').matches
  );
}

function resetMagneticField(element: HTMLElement) {
  element.style.setProperty('--magnetic-x', '0px');
  element.style.setProperty('--magnetic-y', '0px');
  element.style.setProperty('--magnetic-field', '0');
}

function updateMagneticField(
  element: HTMLElement,
  clientX: number,
  clientY: number,
) {
  const rect = element.getBoundingClientRect();

  if (rect.width === 0 || rect.height === 0) {
    resetMagneticField(element);
    return;
  }

  const localX = clientX - rect.left;
  const localY = clientY - rect.top;
  const centerX = rect.width / 2;
  const centerY = rect.height / 2;
  const offsetX = localX - centerX;
  const offsetY = localY - centerY;
  const normalizedX = offsetX / centerX;
  const normalizedY = offsetY / centerY;
  const radialDistance = Math.min(1, Math.hypot(normalizedX, normalizedY));
  const proximity = Math.max(0, 1 - radialDistance);
  const magneticField = Math.pow(
    Math.sin(proximity * Math.PI),
    MAGNETIC_FIELD_EXPONENT,
  );
  const pullX = normalizedX * MAGNETIC_MAX_PULL_PX * magneticField;
  const pullY = normalizedY * MAGNETIC_MAX_PULL_PX * magneticField;

  element.style.setProperty('--magnetic-x', `${pullX.toFixed(2)}px`);
  element.style.setProperty('--magnetic-y', `${pullY.toFixed(2)}px`);
  element.style.setProperty('--magnetic-field', magneticField.toFixed(3));
}

export function magneticFieldEvents<T extends HTMLElement>() {
  const handlePointerLeave: PointerEventHandler<T> = (event) => {
    resetMagneticField(event.currentTarget);
  };

  const handlePointerMove: PointerEventHandler<T> = (event) => {
    if (event.pointerType === 'touch' || prefersReducedMotion()) {
      resetMagneticField(event.currentTarget);
      return;
    }

    updateMagneticField(
      event.currentTarget,
      event.clientX,
      event.clientY,
    );
  };

  return {
    onPointerCancel: handlePointerLeave,
    onPointerLeave: handlePointerLeave,
    onPointerMove: handlePointerMove,
  };
}
