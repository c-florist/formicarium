<script lang="ts">
import { uiStateStore } from "$lib/stores/ui-state-store";
import { startWorldUpdates, worldStore } from "$lib/stores/world-store";
import { calculateMovementDirection } from "$lib/utils/maths";
import {
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
import { invoke } from "@tauri-apps/api/core";
import {
  Application,
  Assets,
  Container,
  Sprite,
  Text,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";

const viewport = new Container();

let app = $state<Application>(new Application());
let uiContainer = $state<Container>(new Container());
let worldContainer = $state<Container>(new Container());

let isSimulationInitialised = $state<boolean>(false);
let isWorldInitialised = $state<boolean>(false);

let canvasContainer: HTMLDivElement;
let animationInterval: NodeJS.Timeout;

let antSprites: Map<number, AntSprite> = new Map();
let antSpritesheet: UnresolvedAsset;

let foodSourceSpritesheet: UnresolvedAsset;
let foodSourceSprites: Map<number, Sprite> = new Map();
let foodSourceStats: Map<number, Container> = new Map();

const initialise = async () => {
  await app.init({
    resizeTo: canvasContainer,
    roundPixels: true,
  });
  canvasContainer.appendChild(app.canvas);

  viewport.addChild(worldContainer);

  app.stage.addChild(viewport);
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
  if (!isSimulationInitialised && canvasContainer) {
    const width = canvasContainer.clientWidth;
    const height = canvasContainer.clientHeight;

    console.log("[PixelWorldCanvas] Initialising simulation");
    await invoke("initialise_simulation", {
      deviceWidth: width,
      deviceHeight: height,
    });

    console.log("[PixelWorldCanvas] Simulation initialised");

    isSimulationInitialised = true;
    startWorldUpdates();
  }

  await initialise();
  await loadGlobalAssets();
});

$effect(() => {
  if (
    !app ||
    !worldContainer ||
    !$worldStore ||
    !antSpritesheet ||
    !foodSourceSpritesheet
  ) {
    return;
  }

  if (!isWorldInitialised) {
    const worldData = $worldStore;

    // Setup background and static elements based on world size
    createRandomisedTileTexture(
      app.renderer,
      worldData.width,
      worldData.height,
    ).then((texture) => {
      const background = new Container();
      background.addChild(new Sprite(texture));
      worldContainer.addChildAt(background, LAYERS.BACKGROUND);
    });
    createBoulderContainer(worldData.width, worldData.height).then(
      (boulders) => {
        worldContainer.addChildAt(boulders, LAYERS.DECORATION);
      },
    );
    createNestContainer(worldData.nest).then((nest) => {
      worldContainer.addChildAt(nest, LAYERS.DECORATION);
    });

    const viewport: Container = app.stage.getChildAt(0);

    // Make the STAGE interactive, not the viewport
    app.stage.eventMode = "static";
    app.stage.hitArea = app.screen;
    app.stage.cursor = "url(/ui/cursor/cursor-default.png),auto";

    let dragging = false;
    let dragStart = { x: 0, y: 0 };

    app.stage.on("pointerdown", (event) => {
      dragging = true;
      dragStart.x = event.global.x - viewport.x;
      dragStart.y = event.global.y - viewport.y;
      app.stage.cursor = "url(/ui/cursor/cursor-drag.png),auto";
    });

    app.stage.on("pointerup", () => {
      dragging = false;
      app.stage.cursor = "url(/ui/cursor/cursor-default.png),auto";
    });

    app.stage.on("pointerupoutside", () => {
      dragging = false;
      app.stage.cursor = "url(/ui/cursor/cursor-default.png),auto";
    });

    app.stage.on("pointermove", (event) => {
      if (dragging) {
        const newX = event.global.x - dragStart.x;
        const newY = event.global.y - dragStart.y;

        // Clamp the viewport's position
        const worldWidth = $worldStore.width;
        const worldHeight = $worldStore.height;
        const screenWidth = app.screen.width;
        const screenHeight = app.screen.height;

        viewport.x = Math.max(Math.min(newX, 0), screenWidth - worldWidth);
        viewport.y = Math.max(Math.min(newY, 0), screenHeight - worldHeight);
      }
    });

    // Start the animation interval after everything is set up
    animationInterval = setInterval(() => {
      for (const [_, antData] of antSprites) {
        antData.animationFrame =
          (antData.animationFrame + 1) % ANIMATION_CONFIG.antFrameCount;
        const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
        antData.sprite.texture = antSpritesheet.textures[frameName];
        const scale = SPRITE_CONFIG.ANT.scale;
        if (antData.direction === "left") {
          antData.sprite.scale.x = -scale;
        } else {
          antData.sprite.scale.x = scale;
        }
      }
    }, ANIMATION_CONFIG.antFrameRate);

    isWorldInitialised = true;
  }

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
