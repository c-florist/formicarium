<script lang="ts">
import type { NestDto } from "@formicarium/domain";
import {
  Application,
  Assets,
  Container,
  RenderTexture,
  Sprite,
  type Texture,
  TilingSprite,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";
import { seededRandom } from "../utils";
import {
  ANIMATION_CONFIG,
  ANT_SPRITESHEET,
  type AntSprite,
  BACKGROUND_CONFIG,
  BOULDER_CONFIG,
  DEFAULT_ANT_TEXTURE,
  FOOD_SOURCE_CONFIG,
  FOOD_SPRITESHEET,
  FOREST_TILESET,
  LAYERS,
  NEST_SPRITESHEET,
  NEST_TEXTURES,
  SPRITE_CONFIG,
  type SpriteConfig,
} from "../world/schema";
import { worldStore } from "../world/world-store";

let canvasContainer: HTMLDivElement;
let app: Application;
let antSprites: Map<number, AntSprite> = new Map();
let antSpritesheet: UnresolvedAsset;
let foodSourceSpritesheet: UnresolvedAsset;
let foodSourceSprites: Map<number, Sprite> = new Map();
let animationInterval: NodeJS.Timeout;

const initialise = async () => {
  app = new Application();
  await app.init({
    width: window.innerWidth - 50,
    height: window.innerHeight - 50,
    roundPixels: true,
  });
  canvasContainer.appendChild(app.canvas);
};

const loadGlobalAssets = async () => {
  const [antSheet, foodSheet] = await Promise.all([
    Assets.load(ANT_SPRITESHEET),
    Assets.load(FOOD_SPRITESHEET),
  ]);

  antSpritesheet = antSheet;
  foodSourceSpritesheet = foodSheet;
};

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

const createRandomisedTileTexture = async (width: number, height: number) => {
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

const createBackground = async () => {
  const textureWidth = app.canvas.width * 2;
  const textureHeight = app.canvas.height * 2;

  const randomisedTexture = await createRandomisedTileTexture(
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

const createBoulders = async () => {
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

const createNest = async (nestDto: NestDto) => {
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

const createSpriteWithConfig = (texture: Texture, config: SpriteConfig) => {
  const sprite = new Sprite(texture);
  sprite.anchor.set(config.anchor.x, config.anchor.y);
  sprite.scale.set(config.scale);
  return sprite;
};

const calculateMovementDirection = (deltaX: number, deltaY: number) => {
  if (Math.abs(deltaX) > Math.abs(deltaY)) {
    return deltaX > 0 ? "right" : "left";
  } else if (Math.abs(deltaY) > 0) {
    return deltaY > 0 ? "down" : "up";
  }
  return "down";
};

const calculateIfHiddenInNest = (
  antX: number,
  antY: number,
  nestX: number,
  nestY: number,
) => {
  const distanceToNest = Math.sqrt((antX - nestX) ** 2 + (antY - nestY) ** 2);
  return distanceToNest > ANIMATION_CONFIG.hideSpriteRadius ? 1 : 0;
};

onMount(async () => {
  if (!$worldStore) {
    return;
  }

  await initialise();
  await loadGlobalAssets();

  await createBackground();
  await createBoulders();

  await createNest($worldStore.nest);

  animationInterval = setInterval(() => {
    // Update all ant sprites with their individual frame counters
    for (const [_, antData] of antSprites) {
      antData.animationFrame =
        (antData.animationFrame + 1) % ANIMATION_CONFIG.antFrameCount;
      const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
      antData.sprite.texture = antSpritesheet.textures[frameName];

      // Flip sprite for left direction
      const scale = SPRITE_CONFIG.ant.scale;
      if (antData.direction === "left") {
        antData.sprite.scale.x = -scale;
      } else {
        antData.sprite.scale.x = scale;
      }
    }
  }, ANIMATION_CONFIG.antFrameRate);
});

$effect(() => {
  if (!$worldStore || !antSpritesheet || !foodSourceSpritesheet) return;

  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));
  const currentFoodSourceIds = new Set(
    $worldStore.foodSources.map((foodSource) => foodSource.id),
  );

  for (const [antId, antData] of antSprites) {
    if (!currentAntIds.has(antId)) {
      app.stage.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }

  for (const [foodSourceId, foodSprite] of foodSourceSprites) {
    if (!currentFoodSourceIds.has(foodSourceId)) {
      app.stage.removeChild(foodSprite);
      foodSourceSprites.delete(foodSourceId);
    }
  }

  // Food source update loop
  for (const foodSource of $worldStore.foodSources) {
    let foodSprite = foodSourceSprites.get(foodSource.id);

    if (!foodSprite) {
      const textureNames = Object.keys(foodSourceSpritesheet.textures);
      const deterministicTextureIndex = foodSource.id % textureNames.length;
      const textureName = textureNames[deterministicTextureIndex];
      const texture = foodSourceSpritesheet.textures[textureName];

      foodSprite = createSpriteWithConfig(texture, SPRITE_CONFIG.food);
      app.stage.addChild(foodSprite);
      foodSourceSprites.set(foodSource.id, foodSprite);
    }

    foodSprite.x = foodSource.x;
    foodSprite.y = foodSource.y;

    // Scale sprite based on remaining amount
    const baseScale = SPRITE_CONFIG.food.scale;
    const scaleRatio = foodSource.amount / FOOD_SOURCE_CONFIG.maxAmount;
    const newScale = Math.max(0, baseScale * scaleRatio);
    foodSprite.scale.set(newScale);
  }

  // Ant sprite update loop
  for (const ant of $worldStore.ants) {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = createSpriteWithConfig(
        antSpritesheet.textures[DEFAULT_ANT_TEXTURE],
        SPRITE_CONFIG.ant,
      );
      app.stage.addChild(sprite);

      antData = {
        sprite,
        previousPosition: { x: ant.x, y: ant.y },
        direction: "down",
        animationFrame: Math.floor(
          Math.random() * ANIMATION_CONFIG.antFrameCount,
        ),
      };
      antSprites.set(ant.id, antData);
    }

    // Calculate movement direction
    const deltaX = ant.x - antData.previousPosition.x;
    const deltaY = ant.y - antData.previousPosition.y;
    antData.direction = calculateMovementDirection(deltaX, deltaY);

    // Update position
    antData.sprite.x = ant.x;
    antData.sprite.y = ant.y;

    // Apply nest fade effect
    antData.sprite.alpha = calculateIfHiddenInNest(
      ant.x,
      ant.y,
      $worldStore.nest.x,
      $worldStore.nest.y,
    );

    antData.previousPosition = { x: ant.x, y: ant.y };
  }
});

onDestroy(() => {
  if (animationInterval) {
    clearInterval(animationInterval);
  }

  if (app) {
    app.destroy(true);
  }
});
</script>

<div bind:this={canvasContainer}></div>
