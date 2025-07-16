<script lang="ts">
import { FancyButton } from "@pixi/ui";
import { Assets, BitmapText, Container, Graphics, Text } from "pixi.js";
import { onDestroy, onMount } from "svelte";

let { container, text, onClick, x, y } = $props<{
  container: Container;
  text: string;
  onClick: () => void;
  x: number;
  y: number;
}>();

let buttonContainer: Container;

onMount(async () => {
  const btnAssets = await Assets.load([
    "/ui/button-small/button-small-3.png",
    "/ui/button-small/button-small-2.png",
    "/ui/button-small/button-small-1.png",
    "/ui/icons/red-cross.png",
  ]);
  await Assets.load("/ui/fonts/pixeloid-mono.fnt");

  const button = new FancyButton({
    icon: btnAssets["/ui/icons/red-cross.png"],
    defaultIconAnchor: { x: 0.5, y: 0.5 },
    defaultView: btnAssets["/ui/button-small/button-small-2.png"],
    // hoverView: btnAssets["/ui/button-small/button-small-1.png"],
    pressedView: btnAssets["/ui/button-small/button-small-1.png"],
    animations: {
      hover: {
        props: {},
        duration: 60,
      },
    },
  });

  button.anchor.set(0.5);
  button.onPress.connect(onClick);

  // Create a background plate for the button
  const background = new Graphics()
    .roundRect(0, 0, button.width + 15, button.height + 15, 8)
    .fill({ color: "#000", alpha: 0.45 });

  // Center the button on the background
  button.x = background.width / 2;
  button.y = background.height / 2;

  // Create a container for the button and its background
  buttonContainer = new Container();
  buttonContainer.addChild(background);
  buttonContainer.addChild(button);

  // Position the entire container
  buttonContainer.x = x;
  buttonContainer.y = y;

  container.addChild(buttonContainer);
});

onDestroy(() => {
  if (buttonContainer) {
    buttonContainer.destroy();
  }
});
</script>
