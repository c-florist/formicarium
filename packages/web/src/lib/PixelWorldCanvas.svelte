<script lang="ts">
import { Application, Assets, Sprite } from "pixi.js";
import { onDestroy, onMount } from "svelte";
import { worldStore } from "./world-store";

let canvasContainer: HTMLDivElement;
let app: Application;
let antSprite: Sprite;

onMount(async () => {
  app = new Application();

  await app.init({
    width: $worldStore?.width,
    height: $worldStore?.height,
    backgroundColor: 0xfef3c7,
  });

  canvasContainer.appendChild(app.canvas);

  const spritesheet = await Assets.load("/worker-ant-spritesheet.json");

  // Use the trimmed walking frames
  antSprite = new Sprite(spritesheet.textures["ant-0"]); // This is 32x96, much better!
  antSprite.scale.set(2); // Scale up to see it clearly
  app.stage.addChild(antSprite);

  // Test all frames
  const frames = ["ant-0", "ant-1", "ant-2", "ant-3"];
  let frameIndex = 0;
  setInterval(() => {
    antSprite.texture = spritesheet.textures[frames[frameIndex]];
    frameIndex = (frameIndex + 1) % frames.length;
  }, 125);
});

$effect(() => {
  if (!$worldStore || !antSprite) return;

  if ($worldStore.ants.length > 0) {
    antSprite.x = $worldStore.ants[0].x;
    antSprite.y = $worldStore.ants[0].y;
  }
});

onDestroy(() => {
  if (app) {
    app.destroy(true);
  }
});
</script>

<div bind:this={canvasContainer}></div>
