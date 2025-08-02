<script lang="ts">
import SimulationService from "$lib/services/simulation";
import { userOptions } from "$lib/state/input.svelte";
import { uiState } from "$lib/state/ui.svelte";
import {
  startWorldUpdates,
  stopWorldUpdates,
  worldStore,
} from "$lib/stores/world";
import {
  calculateIfHiddenInNest,
  calculateMovementDirection,
} from "$lib/utils/maths";
import { setupPanning } from "$lib/world/actions";
import {
  ASSET_ALIASES,
  CURSOR_DEFAULT,
  FOOD_ASSET_ALIASES,
  WORLD_MAP_CONFIG,
} from "$lib/world/assets";
import {
  CLIENT_CONFIG,
  LAYER_INDEX,
  SPRITE_CONFIGS,
} from "$lib/world/constants";
import { createNestContainer, createStatsBubble } from "$lib/world/render";
import { type AntSprite, createSpriteWithConfig } from "$lib/world/sprite";
import { TiledMapRenderer } from "$lib/world/tiled";
import { Application, Assets, Container, Sprite, Text } from "pixi.js";
import { AdjustmentFilter } from "pixi-filters";
import { onDestroy, onMount } from "svelte";

const app = new Application();
const viewport = new Container();
const uiContainer = new Container();
const worldContainer = new Container();

let canvasContainer: HTMLDivElement;
let lastSelectedAntId: number | null = null;

const workerAntAssets = Assets.get(ASSET_ALIASES.WORKER_ANT);
const foodSourceAssets = Assets.get(FOOD_ASSET_ALIASES);

const antSprites: Map<number, AntSprite> = new Map();
const foodSourceSprites: Map<number, Sprite> = new Map();
const foodSourceStats: Map<number, Container> = new Map();
const statsBubblePool: Container[] = [];

const initialisePixiApp = async () => {
  await app.init({
    resizeTo: canvasContainer,
    autoDensity: true,
    resolution: window.devicePixelRatio || 1,
  });
  canvasContainer.appendChild(app.canvas);

  viewport.addChild(worldContainer);
  worldContainer.sortableChildren = true;

  app.stage.addChild(viewport);
  app.stage.addChild(uiContainer);
  app.stage.cursor = CURSOR_DEFAULT;
};

const initialiseWorld = async () => {
  if (!$worldStore) {
    throw new Error("Tried to initialise world without world store data");
  }

  // Load and render Tiled map
  const tiledRenderer = await TiledMapRenderer.fromFile(
    WORLD_MAP_CONFIG.filePath,
  );
  tiledRenderer.loadTilesets();
  const { background, foreground } = tiledRenderer.renderMap(
    WORLD_MAP_CONFIG.scale,
  );
  background.zIndex = LAYER_INDEX.BACKGROUND;
  foreground.zIndex = LAYER_INDEX.FOREGROUND;
  worldContainer.addChild(background, foreground);

  const nest = await createNestContainer($worldStore.nest);
  nest.zIndex = LAYER_INDEX.STATIC_OBJECTS;
  worldContainer.addChild(nest);

  setupPanning({
    appStage: app.stage,
    hitArea: app.screen,
    viewport: viewport,
    worldData: $worldStore,
  });

  // Setup animation tickers
  let frameCounter = 0;
  const animationSpeed = CLIENT_CONFIG.ANIMATION_SPEED;

  app.ticker.add(() => {
    frameCounter++;
    const animateThisFrame = frameCounter >= animationSpeed;
    if (animateThisFrame) {
      frameCounter = 0;
    }

    for (const [, antData] of antSprites) {
      // Animate ants
      if (animateThisFrame && antData.sprite.alpha > 0.9) {
        antData.animationFrame =
          (antData.animationFrame + 1) % SPRITE_CONFIGS.WORKER_ANT.frameCount;
        const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
        antData.sprite.texture = workerAntAssets.textures[frameName];

        const scale = SPRITE_CONFIGS.WORKER_ANT.scale;
        if (antData.direction === "left") {
          antData.sprite.scale.x = -scale;
        } else {
          antData.sprite.scale.x = scale;
        }
      }

      // Smoothly move ants
      const { sprite, targetPosition } = antData;
      if (targetPosition) {
        const dx = targetPosition.x - sprite.x;
        const dy = targetPosition.y - sprite.y;
        sprite.x += dx * 0.1;
        sprite.y += dy * 0.1;
      }
    }
  });
};

onMount(async () => {
  await initialisePixiApp();

  const { clientWidth, clientHeight } = canvasContainer;
  SimulationService.init({
    width: clientWidth,
    height: clientHeight,
    ...userOptions,
  });

  await initialiseWorld();
  startWorldUpdates();
});

$effect(() => {
  if (!$worldStore) {
    return;
  }

  // --- Ant Management ---
  const seenAntIds = new Set(antSprites.keys());

  // Update/Create Ant Sprites
  for (const ant of $worldStore.ants) {
    seenAntIds.delete(ant.id); // Mark this ant as "seen" for this frame
    let antData = antSprites.get(ant.id);

    // If ant doesn't exist, create it
    if (!antData) {
      const sprite = createSpriteWithConfig(
        workerAntAssets.textures[SPRITE_CONFIGS.WORKER_ANT.defaultTextureName],
        SPRITE_CONFIGS.WORKER_ANT,
      );
      sprite.x = ant.x;
      sprite.y = ant.y;
      sprite.zIndex = LAYER_INDEX.ENTITIES;

      sprite.eventMode = "static";
      sprite.on("pointerdown", (e) => {
        e.stopPropagation();
        uiState.selectedAntId = ant.id;
      });

      worldContainer.addChild(sprite);

      antData = {
        sprite,
        targetPosition: { x: ant.x, y: ant.y },
        direction: "down",
        animationFrame: Math.floor(
          Math.random() * SPRITE_CONFIGS.WORKER_ANT.frameCount,
        ),
      };
      antSprites.set(ant.id, antData);
    }

    // Update ant state
    if (ant.state.type === "dying") {
      antData.sprite.alpha =
        ant.state.ticks / CLIENT_CONFIG.ANT_DEATH_ANIMATION_TICKS;
    } else {
      const deltaX = ant.x - antData.targetPosition.x;
      const deltaY = ant.y - antData.targetPosition.y;
      antData.direction = calculateMovementDirection(deltaX, deltaY);
      antData.targetPosition.x = ant.x;
      antData.targetPosition.y = ant.y;
      antData.sprite.alpha = calculateIfHiddenInNest(
        ant.x,
        ant.y,
        $worldStore.nest.x,
        $worldStore.nest.y,
      );
    }
  }

  // Cleanup removed ant sprites
  for (const antId of seenAntIds) {
    const antData = antSprites.get(antId);
    if (antData) {
      worldContainer.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }
  // --- End of Ant Management ---

  // --- Selection Management ---
  if (uiState.selectedAntId !== lastSelectedAntId) {
    if (lastSelectedAntId !== null) {
      const lastSelectedAnt = antSprites.get(lastSelectedAntId);
      if (lastSelectedAnt) {
        lastSelectedAnt.sprite.tint = 0xffffff;
      }
    }

    if (uiState.selectedAntId !== null) {
      const newSelectedAnt = antSprites.get(uiState.selectedAntId);
      if (newSelectedAnt) {
        newSelectedAnt.sprite.tint = 0x00ffff;
      }
    }

    lastSelectedAntId = uiState.selectedAntId;
  }
  // --- End of Selection Management ---

  // --- Food Source Management ---
  const seenFoodSourceIds = new Set(foodSourceSprites.keys());

  // Update/Create Food Sources
  for (const foodSource of $worldStore.foodSources) {
    seenFoodSourceIds.delete(foodSource.id); // Mark as seen
    let statsBubble = foodSourceStats.get(foodSource.id);
    let foodSprite = foodSourceSprites.get(foodSource.id);

    if (!statsBubble) {
      const pooledBubble = statsBubblePool.pop();
      if (pooledBubble) {
        statsBubble = pooledBubble;
        statsBubble.visible = true;
        foodSourceStats.set(foodSource.id, statsBubble);
      } else {
        statsBubble = createStatsBubble(`Amount: ${foodSource.amount}`);
        statsBubble.zIndex = LAYER_INDEX.ENTITIES;
        foodSourceStats.set(foodSource.id, statsBubble);
        worldContainer.addChild(statsBubble);
      }
    }

    statsBubble.getChildAt<Text>(1).text = `Amount: ${foodSource.amount}`;

    if (!foodSprite) {
      const deterministicTextureIndex =
        foodSource.id % FOOD_ASSET_ALIASES.length;
      const texture = foodSourceAssets[deterministicTextureIndex];

      foodSprite = createSpriteWithConfig(texture, SPRITE_CONFIGS.FOOD);
      foodSprite.zIndex = LAYER_INDEX.ENTITIES;
      worldContainer.addChild(foodSprite);
      foodSourceSprites.set(foodSource.id, foodSprite);
    }

    foodSprite.x = foodSource.x;
    foodSprite.y = foodSource.y;
    statsBubble.x = foodSprite.x;
    statsBubble.y = foodSprite.y + SPRITE_CONFIGS.FOOD.statsBubbleYOffset;
    statsBubble.visible = uiState.showStatsOverlay;
    foodSprite.alpha = Math.max(
      0.15,
      foodSource.amount / CLIENT_CONFIG.FOOD_SOURCE_MAX_AMOUNT,
    );
  }

  // Cleanup removed food source sprites
  for (const foodSourceId of seenFoodSourceIds) {
    const foodSprite = foodSourceSprites.get(foodSourceId);
    if (foodSprite) {
      worldContainer.removeChild(foodSprite);
      foodSourceSprites.delete(foodSourceId);
    }

    const statsBubble = foodSourceStats.get(foodSourceId);
    if (statsBubble) {
      statsBubble.visible = false;
      statsBubblePool.push(statsBubble);
      foodSourceStats.delete(foodSourceId);
    }
  }
  // --- End of Food Source Management ---
});

onDestroy(() => {
  stopWorldUpdates();
  if (app.canvas) {
    app.destroy(true);
  }
});
</script>

<div class="relative w-full h-full" bind:this={canvasContainer}></div>
