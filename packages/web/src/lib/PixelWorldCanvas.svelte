<script lang="ts">
import {
  Application,
  Assets,
  Container,
  Sprite,
  type UnresolvedAsset,
} from "pixi.js";
import { onDestroy, onMount } from "svelte";
import { worldStore } from "./world-store";

type AntSprite = {
  sprite: Sprite;
  previousPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};

const WORKER_ANT_SPRITESHEET = "/worker-ant-spritesheet.json";
const ANT_SCALE = 1.4;
const ANT_STARTING_FRAME = "ant-down-0";

const BACKGROUND_TILE_SIZE = 16;

let canvasContainer: HTMLDivElement;
let app: Application;
let antSprites: Map<number, AntSprite> = new Map();
let spritesheet: UnresolvedAsset;

onMount(async () => {
  app = new Application();

  await app.init({
    width: 1200,
    height: 700,
    backgroundColor: 0xfef3c7,
  });

  canvasContainer.appendChild(app.canvas);

  spritesheet = await Assets.load(WORKER_ANT_SPRITESHEET);

  // Add background
  const forestTileset = await Assets.load("/forest-terrain.json");
  const tileNames = ["grass-plain", "grass-1", "grass-2", "grass-3"];

  const tilesX = Math.ceil(app.screen.width / BACKGROUND_TILE_SIZE);
  const tilesY = Math.ceil(app.screen.height / BACKGROUND_TILE_SIZE);

  const backgroundContainer = new Container();

  // Generate random tiles
  for (let y = 0; y < tilesY; y++) {
    for (let x = 0; x < tilesX; x++) {
      // Pick a random tile
      const randomTileName =
        tileNames[Math.floor(Math.random() * tileNames.length)];
      const tileSprite = new Sprite(forestTileset.textures[randomTileName]);

      // Position the tile
      tileSprite.x = x * BACKGROUND_TILE_SIZE;
      tileSprite.y = y * BACKGROUND_TILE_SIZE;

      backgroundContainer.addChild(tileSprite);
    }
  }

  app.stage.addChildAt(backgroundContainer, 0);

  const boulder1Texture = await Assets.load("/boulder-1.png");
  const boulder2Texture = await Assets.load("/boulder-2.png");
  const boulderTextures = [boulder1Texture, boulder2Texture];

  const boulderContainer = new Container();
  const BOULDER_COUNT = 4; // Adjust number of boulders

  for (let i = 0; i < BOULDER_COUNT; i++) {
    // Pick random boulder texture
    const randomTexture =
      boulderTextures[Math.floor(Math.random() * boulderTextures.length)];
    const boulderSprite = new Sprite(randomTexture);

    // Random position
    boulderSprite.x = Math.random() * app.screen.width;
    boulderSprite.y = Math.random() * app.screen.height;

    // Optional: scale boulders
    boulderSprite.scale.set(0.8 + Math.random() * 0.4); // Random scale 0.8-1.2

    boulderContainer.addChild(boulderSprite);
  }

  // Add boulders above background but below ants
  app.stage.addChildAt(boulderContainer, 1);

  setInterval(() => {
    // Update all ant sprites with their individual frame counters
    antSprites.forEach((antData) => {
      antData.animationFrame = (antData.animationFrame + 1) % 4;
      const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
      antData.sprite.texture = spritesheet.textures[frameName];

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
  if (!$worldStore || !spritesheet) return;

  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));

  // Remove sprites for ants that no longer exist
  for (const [antId, antData] of antSprites) {
    if (!currentAntIds.has(antId)) {
      app.stage.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }

  // Main sprite update loop
  $worldStore.ants.forEach((ant) => {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      const sprite = new Sprite(spritesheet.textures[ANT_STARTING_FRAME]);
      sprite.anchor.set(0.5, 0);
      sprite.scale.set(ANT_SCALE);
      app.stage.addChild(sprite);

      antData = {
        sprite,
        previousPosition: { x: ant.x, y: ant.y },
        direction: "down",
        animationFrame: Math.floor(Math.random() * 4), // Random starting frame for variety
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
