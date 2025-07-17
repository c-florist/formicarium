<script lang="ts">
import { uiStateStore } from "$lib/stores/ui-state-store";
import { worldStore } from "$lib/stores/world-store";
import { calculateMovementDirection } from "$lib/utils/maths";
import {
  createBackgroundContainer,
  createBoulderContainer,
  createNestContainer,
  createRandomisedTileTexture,
  createStatsBubble,
} from "$lib/world/render";
import {
  ANIMATION_CONFIG,
  ANT_SPRITESHEET,
  type AntSprite,
  DEFAULT_ANT_TEXTURE,
  FOOD_SOURCE_CONFIG,
  FOOD_SPRITESHEET,
  LAYERS,
  SPRITE_CONFIG,
} from "$lib/world/schema";
import { createSpriteWithConfig } from "$lib/world/sprite";
import {
  Application,
  Assets,
  Container,
  Sprite,
  Text,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";

let app = $state<Application>();
let worldContainer = $state<Container>();
let uiContainer = $state<Container>();

let canvasContainer: HTMLDivElement;
let animationInterval: NodeJS.Timeout;

let antSprites: Map<number, AntSprite> = new Map();
let antSpritesheet: UnresolvedAsset;

let foodSourceSpritesheet: UnresolvedAsset;
let foodSourceSprites: Map<number, Sprite> = new Map();
let foodSourceStats: Map<number, Container> = new Map();

const initialise = async () => {
  app = new Application();
  await app.init({
    resizeTo: canvasContainer,
    roundPixels: true,
  });
  canvasContainer.appendChild(app.canvas);

  worldContainer = new Container();
  app.stage.addChild(worldContainer);

  uiContainer = new Container();
  app.stage.addChild(uiContainer);
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

const loadGlobalAssets = async () => {
  const [antSheet, foodSheet] = await Promise.all([
    Assets.load(ANT_SPRITESHEET),
    Assets.load(FOOD_SPRITESHEET),
  ]);

  // Set scale mode for all loaded simulation assets
  for (const key in antSheet.textures) {
    antSheet.textures[key].source.scaleMode = "nearest";
  }
  for (const key in foodSheet.textures) {
    foodSheet.textures[key].source.scaleMode = "nearest";
  }

  antSpritesheet = antSheet;
  foodSourceSpritesheet = foodSheet;
};

onMount(async () => {
  if (!$worldStore) {
    return;
  }

  await initialise();
  await loadGlobalAssets();

  if (!app || !worldContainer) {
    return;
  }

  const backgroundTexture = await createRandomisedTileTexture(
    app.renderer,
    app.canvas.width,
    app.canvas.height,
  );
  const backgroundContainer = await createBackgroundContainer(
    backgroundTexture,
    app.canvas.width,
    app.canvas.height,
  );
  worldContainer.addChildAt(backgroundContainer, LAYERS.BACKGROUND);

  const boulderContainer = await createBoulderContainer(
    app.canvas.width,
    app.canvas.height,
  );
  worldContainer.addChildAt(boulderContainer, LAYERS.DECORATION);

  const nestContainer = await createNestContainer($worldStore.nest);
  worldContainer.addChildAt(nestContainer, LAYERS.DECORATION);

  animationInterval = setInterval(() => {
    // Update all ant sprites with their individual frame counters
    for (const [_, antData] of antSprites) {
      antData.animationFrame =
        (antData.animationFrame + 1) % ANIMATION_CONFIG.antFrameCount;
      const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
      antData.sprite.texture = antSpritesheet.textures[frameName];

      // Flip sprite for left direction
      const scale = SPRITE_CONFIG.ANT.scale;
      if (antData.direction === "left") {
        antData.sprite.scale.x = -scale;
      } else {
        antData.sprite.scale.x = scale;
      }
    }
  }, ANIMATION_CONFIG.antFrameRate);
});

$effect(() => {
  if (
    !app ||
    !worldContainer ||
    !$worldStore ||
    !antSpritesheet ||
    !foodSourceSpritesheet
  )
    return;

  const showStats = $uiStateStore.showStatsOverlay;

  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));
  const currentFoodSourceIds = new Set(
    $worldStore.foodSources.map((foodSource) => foodSource.id),
  );

  for (const [antId, antData] of antSprites) {
    if (!currentAntIds.has(antId)) {
      worldContainer.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }

  for (const [foodSourceId, foodSprite] of foodSourceSprites) {
    if (!currentFoodSourceIds.has(foodSourceId)) {
      worldContainer.removeChild(foodSprite);
      foodSourceSprites.delete(foodSourceId);

      const statsBubble = foodSourceStats.get(foodSourceId);
      if (statsBubble) {
        worldContainer.removeChild(statsBubble);
        foodSourceStats.delete(foodSourceId);
      }
    }
  }

  // Food source update loop
  for (const foodSource of $worldStore.foodSources) {
    let statsBubble = foodSourceStats.get(foodSource.id);
    let foodSprite = foodSourceSprites.get(foodSource.id);

    if (!statsBubble) {
      statsBubble = createStatsBubble(`Amount: ${foodSource.amount}`);
      foodSourceStats.set(foodSource.id, statsBubble);
      worldContainer.addChild(statsBubble);
    }

    const textObject: Text = statsBubble.getChildAt(1);
    textObject.text = `Amount: ${foodSource.amount}`;

    if (!foodSprite) {
      const textureNames = Object.keys(foodSourceSpritesheet.textures);
      const deterministicTextureIndex = foodSource.id % textureNames.length;
      const textureName = textureNames[deterministicTextureIndex];
      const texture = foodSourceSpritesheet.textures[textureName];

      foodSprite = createSpriteWithConfig(texture, SPRITE_CONFIG.FOOD);
      worldContainer.addChild(foodSprite);
      foodSourceSprites.set(foodSource.id, foodSprite);
    }

    foodSprite.x = foodSource.x;
    foodSprite.y = foodSource.y;
    statsBubble.x = foodSprite.x;
    statsBubble.y = foodSprite.y;

    statsBubble.visible = showStats;

    // Set alpha based on remaining amount
    const alpha = foodSource.amount / FOOD_SOURCE_CONFIG.maxAmount;
    foodSprite.alpha = Math.max(0.15, alpha);
  }

  // Ant sprite update loop
  for (const ant of $worldStore.ants) {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = createSpriteWithConfig(
        antSpritesheet.textures[DEFAULT_ANT_TEXTURE],
        SPRITE_CONFIG.ANT,
      );
      worldContainer.addChild(sprite);

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

<div class="relative w-full h-full" bind:this={canvasContainer}></div>
