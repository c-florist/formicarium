export const SPRITE_CONFIG = {
  ANT: { scale: 1.8, anchor: { x: 0.5, y: 0 } },
  FOOD: { scale: 1.5, anchor: { x: 0.5, y: -1 } },
  NEST: { scale: 6, anchor: { x: 0.5, y: 0.5 } },
} as const;
export type SpriteConfig = (typeof SPRITE_CONFIG)[keyof typeof SPRITE_CONFIG];

export const ANIMATION_CONFIG = {
  antFrameCount: 4,
  hideSpriteRadius: 60,
} as const;

export const LAYER_INDEX = {
  BACKGROUND: 0,
  STATIC_OBJECTS: 1,
  ENTITIES: 2,
} as const;
