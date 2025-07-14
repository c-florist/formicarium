<script lang="ts">
import { Application, Assets, Sprite, type UnresolvedAsset } from "pixi.js";
import { onDestroy, onMount } from "svelte";
import { worldStore } from "./world-store";

type AntSprite = {
  sprite: Sprite;
  previousPosition: { x: number; y: number };
  direction: string;
  animationFrame: number;
};

const WORKER_ANT_SPRITESHEET = "/worker-ant-spritesheet.json";

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

  // Animation timer for walking frames
  setInterval(() => {
    // Update all ant sprites with their individual frame counters
    antSprites.forEach((antData) => {
      antData.animationFrame = (antData.animationFrame + 1) % 4;
      const frameName = `ant-${antData.direction}-${antData.animationFrame}`;
      antData.sprite.texture = spritesheet.textures[frameName];

      // Flip sprite for left direction
      if (antData.direction === "left") {
        antData.sprite.scale.x = -2;
      } else {
        antData.sprite.scale.x = 2;
      }
    });
  }, 150);
});

$effect(() => {
  if (!$worldStore || !spritesheet) return;

  // Create sprites for new ants, remove sprites for deleted ants
  const currentAntIds = new Set($worldStore.ants.map((ant) => ant.id));

  // Remove sprites for ants that no longer exist
  for (const [antId, antData] of antSprites) {
    if (!currentAntIds.has(antId)) {
      app.stage.removeChild(antData.sprite);
      antSprites.delete(antId);
    }
  }

  // Create or update sprites for each ant
  $worldStore.ants.forEach((ant) => {
    let antData = antSprites.get(ant.id);

    if (!antData) {
      // Create new sprite for this ant
      const sprite = new Sprite(spritesheet.textures["ant-down-0"]);
      sprite.anchor.set(0.5, 0);
      sprite.scale.set(2);
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

    // Determine direction based on larger movement axis
    if (Math.abs(deltaX) > Math.abs(deltaY)) {
      antData.direction = deltaX > 0 ? "right" : "left";
    } else if (Math.abs(deltaY) > 0) {
      antData.direction = deltaY > 0 ? "down" : "up";
    }

    // Update position
    antData.sprite.x = ant.x;
    antData.sprite.y = ant.y;

    // Store current position for next comparison
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
