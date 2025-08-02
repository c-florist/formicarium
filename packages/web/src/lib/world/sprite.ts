import { Sprite, type Texture } from "pixi.js";

export type SpriteConfig = {
  scale: number;
  anchor: { x: number; y: number };
};

export type AntSprite = {
  sprite: Sprite;
  targetPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};

export const createSpriteWithConfig = (
  texture: Texture,
  config: SpriteConfig,
) => {
  const sprite = new Sprite(texture);
  sprite.anchor.set(config.anchor.x, config.anchor.y);
  sprite.scale.set(config.scale);

  return sprite;
};
