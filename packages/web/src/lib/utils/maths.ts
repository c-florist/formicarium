import { SPRITE_CONFIGS } from "$lib/world/constants";

export const seededRandom = (seed: number) => {
  const x = Math.sin(seed) * 10000;
  return x - Math.floor(x);
};

export const calculateMovementDirection = (deltaX: number, deltaY: number) => {
  if (Math.abs(deltaX) > Math.abs(deltaY)) {
    return deltaX > 0 ? "right" : "left";
  } else if (Math.abs(deltaY) > 0) {
    return deltaY > 0 ? "down" : "up";
  }
  return "down";
};

export const calculateIfHiddenInNest = (
  antX: number,
  antY: number,
  nestX: number,
  nestY: number,
) => {
  const distanceToNest = Math.sqrt((antX - nestX) ** 2 + (antY - nestY) ** 2);
  return distanceToNest > SPRITE_CONFIGS.ANT.concealedRadius ? 1 : 0;
};
