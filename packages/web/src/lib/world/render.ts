import type { NestDto } from "@formicarium/domain";
import {
  type Application,
  Assets,
  Container,
  RenderTexture,
  Sprite,
  TilingSprite,
} from "pixi.js";
import { seededRandom } from "../utils/maths";
import {
  BACKGROUND_CONFIG,
  BOULDER_CONFIG,
  FOREST_TILESET,
  LAYERS,
  NEST_SPRITESHEET,
  NEST_TEXTURES,
  SPRITE_CONFIG,
} from "../world/schema";

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

const createRandomisedTileTexture = async (
  app: Application,
  width: number,
  height: number,
) => {
  const forestTileset = await Assets.load(FOREST_TILESET);
  const { tileSize } = BACKGROUND_CONFIG;

  const renderTexture = RenderTexture.create({ width, height });

  const tileContainer = new Container();

  const tilesX = Math.ceil(app.screen.width / tileSize);
  const tilesY = Math.ceil(app.screen.height / tileSize);

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

  app.renderer.render({
    container: tileContainer,
    target: renderTexture,
  });

  return renderTexture;
};

export const createBackground = async (app: Application) => {
  const textureWidth = app.canvas.width * 2;
  const textureHeight = app.canvas.height * 2;

  const randomisedTexture = await createRandomisedTileTexture(
    app,
    textureWidth,
    textureHeight,
  );

  const bgTileSprite = new TilingSprite({
    texture: randomisedTexture,
    width: app.canvas.width,
    height: app.canvas.height,
  });

  app.stage.addChildAt(bgTileSprite, LAYERS.BACKGROUND);
};

export const createBoulders = async (app: Application) => {
  const boulderContainer = new Container();

  // Load boulder textures in parallel
  const boulderTextures = await Promise.all(
    BOULDER_CONFIG.textures.map((path) => Assets.load(path)),
  );

  for (let i = 0; i < BOULDER_CONFIG.count; i++) {
    const textureIndex = Math.floor(seededRandom(i) * boulderTextures.length);
    const boulderSprite = new Sprite(boulderTextures[textureIndex]);

    boulderSprite.x = seededRandom(i + 10) * app.screen.width;
    boulderSprite.y = seededRandom(i + 20) * app.screen.height;

    const scaleRange = BOULDER_CONFIG.maxScale - BOULDER_CONFIG.minScale;
    boulderSprite.scale.set(
      BOULDER_CONFIG.minScale + seededRandom(i + 30) * scaleRange,
    );

    boulderContainer.addChild(boulderSprite);
  }

  app.stage.addChildAt(boulderContainer, LAYERS.DECORATION);
};

export const createNest = async (app: Application, nestDto: NestDto) => {
  const nestContainer = new Container();
  const nestTexture = await Assets.load(NEST_SPRITESHEET);
  const nestSprite = new Sprite(nestTexture.textures[NEST_TEXTURES.TREE]);

  const { anchor, scale } = SPRITE_CONFIG.nest;
  nestSprite.anchor.set(anchor.x, anchor.y);
  nestSprite.x = nestDto.x;
  nestSprite.y = nestDto.y;
  nestSprite.scale.set(scale);

  nestContainer.addChild(nestSprite);
  app.stage.addChildAt(nestContainer, LAYERS.DECORATION);
};
