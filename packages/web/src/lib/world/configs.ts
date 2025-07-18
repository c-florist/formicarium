import type { Sprite } from "pixi.js";

export const SPRITE_CONFIG = {
  ANT: { scale: 1.8, anchor: { x: 0.5, y: 0 } },
  FOOD: { scale: 1.5, anchor: { x: 0.5, y: -1 } },
  NEST: { scale: 3.5, anchor: { x: 0.5, y: 0.5 } },
} as const;
export type SpriteConfig = (typeof SPRITE_CONFIG)[keyof typeof SPRITE_CONFIG];

export const ANIMATION_CONFIG = {
  antFrameRate: 100,
  antFrameCount: 4,
  hideSpriteRadius: 60,
} as const;

export const BACKGROUND_CONFIG = {
  tileSize: 16,
  tint: 0x9caf88,
  terrainTileNames: [
    "grass-dark-0",
    "grass-dark-1",
    "grass-dark-2",
    "grass-dark-3",
  ],
  grassTiles: ["grass-plain", "grass-2", "grass-1", "grass-3"],
  grassWeights: [0.75, 0.85, 0.95, 1.0],
} as const;

export const BOULDER_CONFIG = {
  count: 6,
  textures: ["/background/boulder-1.png", "/background/boulder-2.png"],
  minScale: 0.4,
  maxScale: 0.8,
} as const;

export const FOOD_SOURCE_CONFIG = {
  maxAmount: 100,
} as const;

export const LAYERS = {
  BACKGROUND: 0,
  DECORATION: 1,
  ENTITIES: 2,
} as const;

export type AntSprite = {
  sprite: Sprite;
  previousPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};
