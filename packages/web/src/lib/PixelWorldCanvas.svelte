<script lang="ts">
import type { FoodSourceDto, NestDto } from "@formicarium/domain";
import {
  Application,
  Assets,
  Container,
  Sprite,
  type Texture,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";
import { seededRandom } from "./utils";
import { worldStore } from "./world-store";

const SPRITE_CONFIG = {
  ant: { scale: 1, anchor: { x: 0.5, y: 0 } },
  food: { scale: 1.25, anchor: { x: 0.5, y: 0 } },
  nest: { scale: 1.8, anchor: { x: 0.5, y: 0.5 } },
};
type SpriteConfig = (typeof SPRITE_CONFIG)[keyof typeof SPRITE_CONFIG];

const ANIMATION_CONFIG = {
  antFrameRate: 150,
  antFrameCount: 4,
  hideSpriteRadius: 30,
};

const BACKGROUND_CONFIG = {
  tileSize: 16,
  tint: 0x9caf88,
  grassTiles: ["grass-plain", "grass-2", "grass-1", "grass-3"],
  grassWeights: [0.75, 0.85, 0.95, 1.0],
};

const BOULDER_CONFIG = {
  count: 6,
  textures: ["/background/boulder-1.png", "/background/boulder-2.png"],
  minScale: 0.4,
  maxScale: 0.8,
};

const LAYERS = {
  BACKGROUND: 0,
  DECORATION: 1,
  ENTITIES: 2,
};

type AntSprite = {
  sprite: Sprite;
  previousPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};

type SpriteManager<T> = {
  sprites: Map<number, T>;
  cleanup: (id: number) => void;
  update: (items: any[]) => void;
};

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
    Assets.load("/characters/worker-ant-spritesheet.json"),
    Assets.load("/food/food-1.json"),
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

const createTileSprite = (texture: Texture, x: number, y: number) => {
  const sprite = new Sprite(texture);
  sprite.tint = BACKGROUND_CONFIG.tint;
  sprite.x = x * BACKGROUND_CONFIG.tileSize;
  sprite.y = y * BACKGROUND_CONFIG.tileSize;
  return sprite;
};

const createBackground = async () => {
  const backgroundContainer = new Container();
  const forestTileset = await Assets.load("/background/forest-terrain.json");

  const tilesX = Math.ceil(app.screen.width / BACKGROUND_CONFIG.tileSize);
  const tilesY = Math.ceil(app.screen.height / BACKGROUND_CONFIG.tileSize);

  // Generate random tiles
  for (let y = 0; y < tilesY; y++) {
    for (let x = 0; x < tilesX; x++) {
      const randomTile = getRandomGrassTile();
      const tileSprite = createTileSprite(
        forestTileset.textures[randomTile],
        x,
        y,
      );
      backgroundContainer.addChild(tileSprite);
    }
  }

  app.stage.addChildAt(backgroundContainer, LAYERS.BACKGROUND);
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
  const nestTexture = await Assets.load("/nests/big-stump.json");
  const nestSprite = new Sprite(nestTexture.textures["big-tree-0"]);

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

  // Store interval ID for cleanup
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
  }

  // Main ant sprite update loop
  for (const ant of $worldStore.ants) {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = createSpriteWithConfig(
        antSpritesheet.textures["ant-down-0"],
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
      $worldStore.nest.x,
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
