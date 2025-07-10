<script lang="ts">
import { worldStore } from "./world-store";

let canvas: HTMLCanvasElement;

$effect(() => {
  if (!canvas || !$worldStore) return;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  // Set canvas size
  canvas.width = $worldStore.width;
  canvas.height = $worldStore.height;

  // Clear the canvas
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  // Draw the nest
  ctx.fillStyle = "#6b4226";
  ctx.fillRect(
    $worldStore.nest.position.x,
    $worldStore.nest.position.y,
    10,
    10,
  );

  // Draw food sources
  ctx.fillStyle = "#4ade80";
  $worldStore.food.forEach((food) => {
    ctx.fillRect(food.position.x, food.position.y, 5, 5);
  });

  // Draw ants
  ctx.fillStyle = "#8a3324";
  Object.values($worldStore.ants).forEach((ant) => {
    ctx.fillRect(ant.position.x, ant.position.y, 2, 2);
  });
});
</script>

<canvas
  bind:this={canvas}
  class="border border-stone-400 bg-amber-100 w-full h-auto"
  style="aspect-ratio: 1000 / 600;"
></canvas>
