<script lang="ts">
import type { FoodSourceDto, NestDto } from "@formicarium/domain";
import {
  Application,
  Assets,
  Container,
  Sprite,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";
import Page from "../routes/world/+page.svelte";
import { worldStore } from "./world-store";

type AntSprite = {
  sprite: Sprite;
  previousPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};

const ANT_SCALE = 0.8;

let canvasContainer: HTMLDivElement;
let app: Application;
let antSprites: Map<number, AntSprite> = new Map();
let antSpritesheet: UnresolvedAsset;

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
  antSpritesheet = await Assets.load("/characters/worker-ant-spritesheet.json");
};

const createBackground = async () => {
  const backgroundTileSize = 16;
  const backgroundContainer = new Container();
  const forestTileset = await Assets.load("/background/forest-terrain.json");

  const tilesX = Math.ceil(app.screen.width / backgroundTileSize);
  const tilesY = Math.ceil(app.screen.height / backgroundTileSize);

  // Generate random tiles
  for (let y = 0; y < tilesY; y++) {
    for (let x = 0; x < tilesX; x++) {
      const randomTile = (() => {
        const rng = Math.random();
        if (rng < 0.75) {
          return "grass-plain";
        } else if (rng < 0.85) {
          return "grass-2";
        } else if (rng < 0.95) {
          return "grass-1";
        } else {
          return "grass-3";
        }
      })();

      const tileSprite = new Sprite(forestTileset.textures[randomTile]);

      // Dark green tint
      tileSprite.tint = 0x9caf88;

      // Position the tile
      tileSprite.x = x * backgroundTileSize;
      tileSprite.y = y * backgroundTileSize;

      backgroundContainer.addChild(tileSprite);
    }
  }

  app.stage.addChildAt(backgroundContainer, 0);
};

const createBoulders = async () => {
  const boulderContainer = new Container();
  const boulderCount = 6;

  const boulder1Texture = await Assets.load("/background/boulder-1.png");
  const boulder2Texture = await Assets.load("/background/boulder-2.png");
  const boulderTextures = [boulder1Texture, boulder2Texture];

  for (let i = 0; i < boulderCount; i++) {
    const randomTexture =
      boulderTextures[Math.floor(Math.random() * boulderTextures.length)];
    const boulderSprite = new Sprite(randomTexture);

    boulderSprite.x = Math.random() * app.screen.width;
    boulderSprite.y = Math.random() * app.screen.height;

    // Scale boulders
    boulderSprite.scale.set(0.4 + Math.random() * 0.4);

    boulderContainer.addChild(boulderSprite);
  }

  // Add boulders above background but below ants
  app.stage.addChildAt(boulderContainer, 1);
};

const createNest = async (nestDto: NestDto) => {
  const nestContainer = new Container();
  const nestTexture = await Assets.load("/nests/big-stump.json");
  const nestSprite = new Sprite(nestTexture.textures["big-tree-0"]);

  nestSprite.anchor.set(0.5, 0.5);
  nestSprite.x = nestDto.x;
  nestSprite.y = nestDto.y;

  nestSprite.scale.set(1.8);

  nestContainer.addChild(nestSprite);
  app.stage.addChildAt(nestContainer, 1);
};

const createFoodSources = async (foodSourceDtos: FoodSourceDto[]) => {
  const foodSourceContainer = new Container();
  const foodSourceTexture = await Assets.load("/food/food-1.json");
  const foodTextureNames = ["croissant-0", "croissant-1"];

  for (const foodSource of foodSourceDtos) {
    const randomTextureName =
      foodTextureNames[Math.floor(Math.random() * foodTextureNames.length)];
    const foodSprite = new Sprite(
      foodSourceTexture.textures[randomTextureName],
    );

    foodSprite.anchor.set(0);
    foodSprite.x = foodSource.x;
    foodSprite.y = foodSource.y;

    foodSprite.scale.set(1);

    foodSourceContainer.addChild(foodSprite);
  }

  app.stage.addChildAt(foodSourceContainer, 2);
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
  await createFoodSources($worldStore.foodSources);

  setInterval(() => {
    // Update all ant sprites with their individual frame counters
    antSprites.forEach((antData) => {
      antData.animationFrame = (antData.animationFrame + 1) % 4;
      const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
      antData.sprite.texture = antSpritesheet.textures[frameName];

      // Flip sprite for left direction
      if (antData.direction === "left") {
        antData.sprite.scale.x = -ANT_SCALE;
      } else {
        antData.sprite.scale.x = ANT_SCALE;
      }
    });
  }, 150);
});

$effect(() => {
  if (!$worldStore || !antSpritesheet) return;

  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));

  // Remove sprites for ants that no longer exist
  for (const [antId, antData] of antSprites) {
    if (!currentAntIds.has(antId)) {
      app.stage.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }

  // Main ant sprite update loop
  $worldStore.ants.forEach((ant) => {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = new Sprite(antSpritesheet.textures["ant-down-0"]);
      sprite.anchor.set(0.5, 0);
      sprite.scale.set(ANT_SCALE);
      app.stage.addChild(sprite);

      antData = {
        sprite,
        previousPosition: { x: ant.x, y: ant.y },
        direction: "down",
        // Random starting frame for variety
        animationFrame: Math.floor(Math.random() * 4),
      };
      antSprites.set(ant.id, antData);
    }

    // Calculate movement direction
    const deltaX = ant.x - antData.previousPosition.x;
    const deltaY = ant.y - antData.previousPosition.y;

    if (Math.abs(deltaX) > Math.abs(deltaY)) {
      antData.direction = deltaX > 0 ? "right" : "left";
    } else if (Math.abs(deltaY) > 0) {
      antData.direction = deltaY > 0 ? "down" : "up";
    }

    antData.sprite.x = ant.x;
    antData.sprite.y = ant.y;

    // Make ants fade when near nest
    const nestX = $worldStore.nest.x;
    const nestY = $worldStore.nest.y;
    const distanceToNest = Math.sqrt(
      (ant.x - nestX) ** 2 + (ant.y - nestY) ** 2,
    );
    const fadeRadius = 30;

    antData.sprite.alpha = distanceToNest > fadeRadius ? 1 : 0;

    antData.previousPosition = { x: ant.x, y: ant.y };
  });
});

onDestroy(() => {
  if (app) {
    app.destroy(true);
  }
});
</script>

<div bind:this={canvasContainer}></div>
