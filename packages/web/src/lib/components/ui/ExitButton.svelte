<script lang="ts">
import { FancyButton } from "@pixi/ui";
import { Assets, BitmapText, Container, Graphics, Text } from "pixi.js";
import { onDestroy, onMount } from "svelte";

let { container, onClick, x, y, scale } = $props<{
  container: Container;
  onClick: () => void;
  x: number;
  y: number;
  scale?: number;
}>();

let buttonContainer: Container;

onMount(async () => {
  const btnAssets = await Assets.load([
    "/ui/button-small/button-small-2.png",
    "/ui/icons/red-cross.png",
  ]);

  const button = new FancyButton({
    icon: btnAssets["/ui/icons/red-cross.png"],
    defaultIconAnchor: { x: 0.5, y: 0.5 },
    defaultView: btnAssets["/ui/button-small/button-small-2.png"],
    animations: {
      pressed: {
        props: { x: 0, y: 2 },
        duration: 50,
      },
    },
  });

  if (scale) {
    button.scale.set(scale);
  }
  button.anchor.set(0.5);

  button.onPress.connect(onClick);

  // Create a background plate for the button
  const background = new Graphics()
    .roundRect(0, 0, button.width + 3, button.height + 5, 6)
    .fill({ color: "#000", alpha: 0.35 });

  // Center the button on the background
  button.x = background.width / 2;
  button.y = background.height / 2.8;

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
