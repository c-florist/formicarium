import { seededRandom } from "$lib/utils/maths";
import {
  BACKGROUND_CONFIG,
  BOULDER_CONFIG,
  FOREST_TILESET,
  NEST_SPRITESHEET,
  NEST_TEXTURES,
  SPRITE_CONFIG,
} from "$lib/world/schema";
import type { NestDto } from "@formicarium/domain";
import {
  Assets,
  Container,
  Graphics,
  type Renderer,
  RenderTexture,
  Sprite,
  Text,
  TilingSprite,
} from "pixi.js";

const getRandomGrassTile = () => {
  const rng = Math.random();
  const { grassTiles, grassWeights } = BACKGROUND_CONFIG;

  for (let i = 0; i < grassWeights.length; i++) {
    if (rng < grassWeights[i]) {
      return grassTiles[i];
    }
  }
  return grassTiles[0];
};

export const createRandomisedTileTexture = async (
  renderer: Renderer,
  canvasWidth: number,
  canvasHeight: number,
) => {
  const textureWidth = canvasWidth * 2;
  const textureHeight = canvasHeight * 2;

  const forestTileset = await Assets.load(FOREST_TILESET);
  const { tileSize } = BACKGROUND_CONFIG;

  const renderTexture = RenderTexture.create({
    width: textureWidth,
    height: textureHeight,
  });

  const tileContainer = new Container();

  const tilesX = Math.ceil(canvasWidth / tileSize);
  const tilesY = Math.ceil(canvasHeight / tileSize);

  // Generate random tiles
  for (let y = 0; y < tilesY; y++) {
    for (let x = 0; x < tilesX; x++) {
      const randomTile = getRandomGrassTile();
      const tileSprite = new Sprite(forestTileset.textures[randomTile]);

      tileSprite.tint = BACKGROUND_CONFIG.tint;
      tileSprite.x = x * BACKGROUND_CONFIG.tileSize;
      tileSprite.y = y * BACKGROUND_CONFIG.tileSize;

      tileContainer.addChild(tileSprite);
    }
  }

  renderer.render({
    container: tileContainer,
    target: renderTexture,
  });

  return renderTexture;
};

export const createBackgroundContainer = async (
  texture: RenderTexture,
  canvasWidth: number,
  canvasHeight: number,
) => {
  const bgTileSprite = new TilingSprite({
    texture: texture,
    width: canvasWidth,
    height: canvasHeight,
  });

  return bgTileSprite;
};

export const createBoulderContainer = async (
  canvasWidth: number,
  canvasHeight: number,
) => {
  const boulderContainer = new Container();

  // Load boulder textures in parallel
  const boulderTextures = await Promise.all(
    BOULDER_CONFIG.textures.map((path) => Assets.load(path)),
  );

  for (let i = 0; i < BOULDER_CONFIG.count; i++) {
    const textureIndex = Math.floor(seededRandom(i) * boulderTextures.length);
    const boulderSprite = new Sprite(boulderTextures[textureIndex]);

    boulderSprite.x = seededRandom(i + 10) * canvasWidth;
    boulderSprite.y = seededRandom(i + 20) * canvasHeight;

    const scaleRange = BOULDER_CONFIG.maxScale - BOULDER_CONFIG.minScale;
    boulderSprite.scale.set(
      BOULDER_CONFIG.minScale + seededRandom(i + 30) * scaleRange,
    );

    boulderContainer.addChild(boulderSprite);
  }

  return boulderContainer;
};

export const createNestContainer = async (nestDto: NestDto) => {
  const nestContainer = new Container();
  const nestTextures = await Assets.load(NEST_SPRITESHEET);

  nestTextures.textures[NEST_TEXTURES.TREE].source.scaleMode = "nearest";
  const nestSprite = new Sprite(nestTextures.textures[NEST_TEXTURES.TREE]);

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
