<script lang="ts">
import { FancyButton } from "@pixi/ui";
import { Assets, Container, Text } from "pixi.js";
import { onDestroy, onMount } from "svelte";

let { container, text, onClick, x, y } = $props<{
  container: Container;
  text: string;
  onClick: () => void;
  x: number;
  y: number;
}>();

let button: FancyButton;

onMount(async () => {
  const assets = await Assets.load([
    "/ui/button-small/button-small-3.png",
    "/ui/button-small/button-small-2.png",
    "/ui/button-small/button-small-1.png",
  ]);

  button = new FancyButton({
    defaultView: assets["/ui/button-small/button-small-3.png"],
    hoverView: assets["/ui/button-small/button-small-2.png"],
    pressedView: assets["/ui/button-small/button-small-1.png"],
    text: new Text({
      text,
      style: {
        fill: "white",
        fontSize: 18,
        fontWeight: "bold",
      },
    }),
    animations: {
      pressed: {
        props: {
          scale: { x: 0.9, y: 0.9 },
        },
        duration: 100,
      },
    },
  });

  button.x = x;
  button.y = y;

  button.anchor.set(0.5);
  button.onPress.connect(onClick);

  container.addChild(button);
});

onDestroy(() => {
  if (button) {
    button.destroy();
  }
});
</script>
