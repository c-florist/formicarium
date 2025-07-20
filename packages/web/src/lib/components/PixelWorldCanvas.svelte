<script lang="ts">
import { EMIT_EVENTS } from "$lib/core/events";
import { initialiseSimulation } from "$lib/core/query";
import { uiStateStore } from "$lib/stores/ui-state-store";
import { startWorldUpdates, worldStore } from "$lib/stores/world-store";
import {
  calculateIfHiddenInNest,
  calculateMovementDirection,
} from "$lib/utils/maths";
import {
  CURSOR_DEFAULT,
  DEFAULT_ANT_TEXTURE,
  WORLD_ASSETS,
} from "$lib/world/assets";
import {
  ANIMATION_CONFIG,
  type AntSprite,
  FOOD_SOURCE_CONFIG,
  LAYERS,
  SPRITE_CONFIG,
} from "$lib/world/configs";
import { createNestContainer, createStatsBubble } from "$lib/world/render";
import { createSpriteWithConfig } from "$lib/world/sprite";
import { loadTiledMap } from "$lib/world/tiled";
import type { WorldDto } from "@formicarium/domain";
import { event } from "@tauri-apps/api";
import { Application, Assets, Container, Sprite, Text } from "pixi.js";
import { AdjustmentFilter, GodrayFilter } from "pixi-filters";
import { onDestroy, onMount } from "svelte";
import config from "../../../../domain/src/systemConfig.json";

const app = new Application();
const viewport = new Container();
const uiContainer = new Container();
const worldContainer = new Container();

let godrayFilter = $state<GodrayFilter>();

let canvasContainer: HTMLDivElement;

const workerAntAssets = Assets.get(WORLD_ASSETS.WORKER_ANT.alias);
const foodSourceAssets = Assets.get(WORLD_ASSETS.FOOD_SOURCE.alias);

let antSprites: Map<number, AntSprite> = new Map();
let foodSourceSprites: Map<number, Sprite> = new Map();
let foodSourceStats: Map<number, Container> = new Map();

const initialisePixiApp = async () => {
  await app.init({
    resizeTo: canvasContainer,
    roundPixels: true,
  });
  canvasContainer.appendChild(app.canvas);

  const adjustmentFilter = new AdjustmentFilter({
    gamma: 0.85,
    brightness: 1.1,
    saturation: 1.1,
  });

  godrayFilter = new GodrayFilter({
    angle: 30,
    gain: 0.4,
    lacunarity: 5,
    parallel: true,
    time: 0,
    // Top-left light source
    // center: [0.4, 2],
    alpha: 0.4,
  });

  viewport.addChild(worldContainer);
  viewport.filters = [adjustmentFilter, godrayFilter];

  app.stage.addChild(viewport);
  app.stage.addChild(uiContainer);
  app.stage.cursor = CURSOR_DEFAULT;
};

const initialiseWorld = async (worldData: WorldDto) => {
  // Load and render Tiled map
  const tiledRenderer = await loadTiledMap("/background/world-map-1.json");
  await tiledRenderer.loadTilesets();
  const mapScale = 3;
  const background = tiledRenderer.renderMap(mapScale);
  worldContainer.addChildAt(background, LAYERS.BACKGROUND);

  const nest = await createNestContainer(worldData.nest);
  worldContainer.addChildAt(nest, LAYERS.STATIC_OBJECTS);

  // Setup viewport dragging
  app.stage.eventMode = "static";
  app.stage.hitArea = app.screen;

  let dragging = false;
  let dragStart = { x: 0, y: 0 };

  app.stage.on("pointerdown", (event) => {
    dragging = true;
    dragStart.x = event.global.x - viewport.x;
    dragStart.y = event.global.y - viewport.y;
  });

  app.stage.on("pointerup", () => {
    dragging = false;
  });
  app.stage.on("pointerupoutside", () => {
    dragging = false;
  });

  app.stage.on("pointermove", (event) => {
    if (dragging) {
      const newX = event.global.x - dragStart.x;
      const newY = event.global.y - dragStart.y;
      const { width: worldWidth, height: worldHeight } = worldData;
      const { width: screenWidth, height: screenHeight } = app.screen;

      viewport.x = Math.max(Math.min(newX, 0), screenWidth - worldWidth);
      viewport.y = Math.max(Math.min(newY, 0), screenHeight - worldHeight);
    }
  });

  // Setup animation ticker
  let frameCounter = 0;
  const animationSpeed = config.rendering.animationSpeed;

  app.ticker.add(() => {
    frameCounter++;
    if (frameCounter < animationSpeed) return;
    frameCounter = 0;

    for (const [, antData] of antSprites) {
      if (antData.sprite.alpha > 0.9) {
        antData.animationFrame =
          (antData.animationFrame + 1) % ANIMATION_CONFIG.antFrameCount;
        const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
        antData.sprite.texture = workerAntAssets.textures[frameName];

        const scale = SPRITE_CONFIG.ANT.scale;
        if (antData.direction === "left") {
          antData.sprite.scale.x = -scale;
        } else {
          antData.sprite.scale.x = scale;
        }
      }
    }
  });

  app.ticker.add(() => {
    if (!godrayFilter) {
      return;
    }
    godrayFilter.time += 0.003;
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

      foodSprite = createSpriteWithConfig(texture, SPRITE_CONFIG.FOOD);
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
      foodSource.amount / FOOD_SOURCE_CONFIG.maxAmount,
    );
  }

  // Update/Create Ant Sprites
  for (const ant of $worldStore.ants) {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = createSpriteWithConfig(
        workerAntAssets.textures[DEFAULT_ANT_TEXTURE],
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

    if (ant.state.type === "dying") {
      antData.sprite.alpha = ant.state.ticks / config.ant.deathAnimationTicks;
    } else {
      const deltaX = ant.x - antData.previousPosition.x;
      const deltaY = ant.y - antData.previousPosition.y;
      antData.direction = calculateMovementDirection(deltaX, deltaY);
      antData.sprite.x = ant.x;
      antData.sprite.y = ant.y;
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
