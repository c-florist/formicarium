import { Sprite, type Texture } from "pixi.js";
import type { SpriteConfig } from "./schema";

export const createSpriteWithConfig = (
  texture: Texture,
  config: SpriteConfig,
) => {
  const sprite = new Sprite(texture);
  sprite.anchor.set(config.anchor.x, config.anchor.y);
  sprite.scale.set(config.scale);
  return sprite;
};
