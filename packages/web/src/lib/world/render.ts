import { NEST_TEXTURES, WORLD_ASSETS } from "$lib/world/assets";
import { BACKGROUND_CONFIG, SPRITE_CONFIG } from "$lib/world/configs";
import type { NestDto } from "@formicarium/domain";
import {
  Assets,
  Container,
  Graphics,
  Sprite,
  Text,
  TilingSprite,
} from "pixi.js";

export const createBackgroundContainer = async (
  width: number,
  height: number,
) => {
  const terrainAssets = Assets.get(WORLD_ASSETS.TERRAIN.alias);
  const darkGreenTiledTexture =
    terrainAssets.textures[BACKGROUND_CONFIG.terrainTileNames[0]];
  const terrainContainer = new Container();

  const terrainSprite = new TilingSprite({
    texture: darkGreenTiledTexture,
    width,
    height,
  });
  terrainContainer.addChild(terrainSprite);

  return terrainContainer;
};

export const createNestContainer = async (nestDto: NestDto) => {
  const nestContainer = new Container();

  const nestAssets = Assets.get(WORLD_ASSETS.NEST.alias);
  const nestSprite = new Sprite(nestAssets.textures[NEST_TEXTURES.TREE]);

  const { anchor, scale } = SPRITE_CONFIG.NEST;
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
