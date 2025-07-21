import { ASSET_ALIASES } from "$lib/world/assets";
import { SPRITE_CONFIGS } from "$lib/world/constants";
import type { NestDto } from "@formicarium/domain";
import { Assets, Container, Graphics, Sprite, Text } from "pixi.js";

export const createNestContainer = async (nestDto: NestDto) => {
  const nestContainer = new Container();

  const nestTexture = Assets.get(ASSET_ALIASES.NEST);
  const nestSprite = new Sprite(nestTexture);

  const { anchor, scale } = SPRITE_CONFIGS.NEST;
  nestSprite.anchor.set(anchor.x, anchor.y);
  nestSprite.x = nestDto.x;
  nestSprite.y = nestDto.y;
  nestSprite.scale.set(scale);

  nestContainer.addChild(nestSprite);

  return nestContainer;
};

export const createStatsBubble = (initialText: string) => {
  const bubble = new Container();

  const text = new Text({
    text: initialText,
    style: {
      fontFamily: "Arial",
      fontSize: 12,
      fill: "white",
      align: "center",
    },
  });
  text.anchor.set(0.5);

  const background = new Graphics();
  const padding = 4;
  background
    .roundRect(
      text.x - text.width / 2 - padding,
      text.y - text.height / 2 - padding,
      text.width + padding * 2,
      text.height + padding * 2,
      4,
    )
    .fill({ color: 0x000000, alpha: 0.5 });

  bubble.addChild(background, text);

  return bubble;
};
