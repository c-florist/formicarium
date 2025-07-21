export const SPRITE_CONFIGS = {
  WORKER_ANT: {
    scale: 1.8,
    anchor: { x: 0.5, y: 0 },
    frameCount: 4,
    concealedRadius: 60,
    defaultTextureName: "ant-down-0",
  },
  FOOD: { scale: 1.5, anchor: { x: 0.5, y: -1 } },
  NEST: { scale: 4, anchor: { x: 0.5, y: 0.5 } },
} as const;

export const LAYER_INDEX = {
  BACKGROUND: 0,
  STATIC_OBJECTS: 1,
  ENTITIES: 2,
} as const;
