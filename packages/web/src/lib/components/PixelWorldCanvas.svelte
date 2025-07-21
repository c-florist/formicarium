<script lang="ts">
import { EMIT_EVENTS } from "$lib/core/events";
import { initialiseSimulation } from "$lib/core/query";
import { uiStateStore } from "$lib/stores/ui-state-store";
import { startWorldUpdates, worldStore } from "$lib/stores/world-store";
import {
  calculateIfHiddenInNest,
  calculateMovementDirection,
} from "$lib/utils/maths";
import { setupPanning } from "$lib/world/actions";
import {
  ASSET_ALIASES,
  CURSOR_DEFAULT,
  WORLD_MAP_CONFIG,
} from "$lib/world/assets";
import { LAYER_INDEX, SPRITE_CONFIGS } from "$lib/world/constants";
import { createNestContainer, createStatsBubble } from "$lib/world/render";
import { type AntSprite, createSpriteWithConfig } from "$lib/world/sprite";
import { TiledMapRenderer } from "$lib/world/tiled";
import type { WorldDto } from "@formicarium/domain";
import { event } from "@tauri-apps/api";
import { Application, Assets, Container, Sprite, Text } from "pixi.js";
import { AdjustmentFilter } from "pixi-filters";
import { onDestroy, onMount } from "svelte";
import SYSTEM_CONFIG from "../../../../domain/src/systemConfig.json";

const app = new Application();
const viewport = new Container();
const uiContainer = new Container();
const worldContainer = new Container();

let canvasContainer: HTMLDivElement;

const workerAntAssets = Assets.get(ASSET_ALIASES.WORKER_ANT);
const foodSourceAssets = Assets.get(ASSET_ALIASES.FOOD_SOURCE);

let antSprites: Map<number, AntSprite> = new Map();
let foodSourceSprites: Map<number, Sprite> = new Map();
let foodSourceStats: Map<number, Container> = new Map();

const initialisePixiApp = async () => {
  await app.init({
    resizeTo: canvasContainer,
    roundPixels: true,
    sharedTicker: true,
    premultipliedAlpha: false,
  });
  canvasContainer.appendChild(app.canvas);

  const adjustmentFilter = new AdjustmentFilter({
    gamma: 0.9,
    saturation: 1.05,
  });

  viewport.addChild(worldContainer);
  viewport.filters = [adjustmentFilter];

  app.stage.addChild(viewport);
  app.stage.addChild(uiContainer);
  app.stage.cursor = CURSOR_DEFAULT;
};

const initialiseWorld = async (worldData: WorldDto) => {
  // Load and render Tiled map
  const tiledRenderer = await TiledMapRenderer.fromFile(
    WORLD_MAP_CONFIG.filePath,
  );
  tiledRenderer.loadTilesets();
  const background = tiledRenderer.renderMap(WORLD_MAP_CONFIG.scale);
  worldContainer.addChildAt(background, LAYER_INDEX.BACKGROUND);

  const nest = await createNestContainer(worldData.nest);
  worldContainer.addChildAt(nest, LAYER_INDEX.STATIC_OBJECTS);

  setupPanning({
    appStage: app.stage,
    hitArea: app.screen,
    viewport: viewport,
    worldData: worldData,
  });

  // Setup animation tickers
  let frameCounter = 0;
  const animationSpeed = SYSTEM_CONFIG.rendering.animationSpeed;

  app.ticker.add((ticker) => {
    frameCounter++;
    if (frameCounter >= animationSpeed) {
      frameCounter = 0;
      for (const [, antData] of antSprites) {
        if (antData.sprite.alpha > 0.9) {
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
      }
    }

    // Smoothly move ants
    for (const [, antData] of antSprites) {
      const { sprite, targetPosition } = antData;
      if (targetPosition) {
        const dx = targetPosition.x - sprite.x;
        const dy = targetPosition.y - sprite.y;
        sprite.x += dx * ticker.deltaTime * 0.1;
        sprite.y += dy * ticker.deltaTime * 0.1;
      }
    }
  });
};

onMount(async () => {
  await initialisePixiApp();

  const unlisten = await event.once<WorldDto>(
    EMIT_EVENTS.SIM_INITIALISED,
    (event) => {
      initialiseWorld(event.payload);
      unlisten();
    },
  );

  const { clientWidth, clientHeight } = canvasContainer;
  await initialiseSimulation(clientWidth, clientHeight);

  startWorldUpdates();
});

$effect(() => {
  if (!$worldStore) {
    return;
  }

  const showStats = $uiStateStore.showStatsOverlay;
  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));
  const currentFoodSourceIds = new Set(
    $worldStore.foodSources.map((foodSource) => foodSource.id),
  );

  // Cleanup removed sprites
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

  // Update/Create Food Sources
  for (const foodSource of $worldStore.foodSources) {
    let statsBubble = foodSourceStats.get(foodSource.id);
    let foodSprite = foodSourceSprites.get(foodSource.id);

    if (!statsBubble) {
      statsBubble = createStatsBubble(`Amount: ${foodSource.amount}`);
      foodSourceStats.set(foodSource.id, statsBubble);
      worldContainer.addChild(statsBubble);
    }

    statsBubble.getChildAt<Text>(1).text = `Amount: ${foodSource.amount}`;

    if (!foodSprite) {
      const textureNames = Object.keys(foodSourceAssets.textures);
      const deterministicTextureIndex = foodSource.id % textureNames.length;
      const textureName = textureNames[deterministicTextureIndex];
      const texture = foodSourceAssets.textures[textureName];

      foodSprite = createSpriteWithConfig(texture, SPRITE_CONFIGS.FOOD);
      worldContainer.addChild(foodSprite);
      foodSourceSprites.set(foodSource.id, foodSprite);
    }

    foodSprite.x = foodSource.x;
    foodSprite.y = foodSource.y;
    statsBubble.x = foodSprite.x;
    statsBubble.y = foodSprite.y;
    statsBubble.visible = showStats;
    foodSprite.alpha = Math.max(
      0.15,
      foodSource.amount / SYSTEM_CONFIG.world.maxFoodSources,
    );
  }

  // Update/Create Ant Sprites
  for (const ant of $worldStore.ants) {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = createSpriteWithConfig(
        workerAntAssets.textures[SPRITE_CONFIGS.WORKER_ANT.defaultTextureName],
        SPRITE_CONFIGS.WORKER_ANT,
      );
      sprite.x = ant.x;
      sprite.y = ant.y;
      worldContainer.addChild(sprite);

      antData = {
        sprite,
        previousPosition: { x: ant.x, y: ant.y },
        targetPosition: { x: ant.x, y: ant.y },
        direction: "down",
        animationFrame: Math.floor(
          Math.random() * SPRITE_CONFIGS.WORKER_ANT.frameCount,
        ),
      };
      antSprites.set(ant.id, antData);
    }

    if (ant.state.type === "dying") {
      antData.sprite.alpha =
        ant.state.ticks / SYSTEM_CONFIG.ant.deathAnimationTicks;
    } else {
      const deltaX = ant.x - antData.previousPosition.x;
      const deltaY = ant.y - antData.previousPosition.y;
      antData.direction = calculateMovementDirection(deltaX, deltaY);
      antData.targetPosition = { x: ant.x, y: ant.y };
      antData.sprite.alpha = calculateIfHiddenInNest(
        ant.x,
        ant.y,
        $worldStore.nest.x,
        $worldStore.nest.y,
      );
    }

    antData.previousPosition = { x: ant.x, y: ant.y };
  }
});

onDestroy(() => {
  if (app.canvas) {
    app.destroy(true);
  }
});
</script>

<div class="relative w-full h-full" bind:this={canvasContainer}></div>
