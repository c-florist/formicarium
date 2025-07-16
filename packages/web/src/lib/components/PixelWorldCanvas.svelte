<script lang="ts">
import {
  Application,
  Assets,
  Container,
  Sprite,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount, type SvelteComponent } from "svelte";
import { calculateMovementDirection } from "../utils/maths";
import {
  createBackgroundContainer,
  createBoulderContainer,
  createNestContainer,
  createRandomisedTileTexture,
} from "../world/render";
import {
  ANIMATION_CONFIG,
  ANT_SPRITESHEET,
  type AntSprite,
  DEFAULT_ANT_TEXTURE,
  FOOD_SOURCE_CONFIG,
  FOOD_SPRITESHEET,
  LAYERS,
  SPRITE_CONFIG,
} from "../world/schema";
import { createSpriteWithConfig } from "../world/sprite";
import { worldStore } from "../world/world-store";

let { children } = $props<{
  children?: (container: Container) => SvelteComponent;
}>();
let app = $state<Application>();
let worldContainer = $state<Container>();
let uiContainer = $state<Container>();

let canvasContainer: HTMLDivElement;
let antSprites: Map<number, AntSprite> = new Map();
let antSpritesheet: UnresolvedAsset;
let foodSourceSpritesheet: UnresolvedAsset;
let foodSourceSprites: Map<number, Sprite> = new Map();
let animationInterval: NodeJS.Timeout;

const initialise = async () => {
  app = new Application();
  await app.init({
    width: window.innerWidth,
    height: window.innerHeight,
    roundPixels: true,
  });
  canvasContainer.appendChild(app.canvas);

  worldContainer = new Container();
  app.stage.addChild(worldContainer);

  uiContainer = new Container();
  app.stage.addChild(uiContainer);

  app.renderer.events.cursorStyles.default =
    "url('/ui/cursor/cursor-default.png'),auto";
  app.renderer.events.cursorStyles.hover =
    "url('/ui/cursor/cursor-hover.png'),auto";
  app.renderer.events.cursorStyles.drag =
    "url('/ui/cursor/cursor-drag.png'),auto";
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
  if (
    !app ||
    !worldContainer ||
    !$worldStore ||
    !antSpritesheet ||
    !foodSourceSpritesheet
  )
    return;

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
      worldContainer.addChild(foodSprite);
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

<div class="relative" bind:this={canvasContainer}>
  {#if uiContainer && children}
    {@render children(uiContainer)}
  {/if}
</div>
