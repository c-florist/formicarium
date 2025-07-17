<script lang="ts">
import { FancyButton } from "@pixi/ui";
import { Assets, Container, Graphics } from "pixi.js";
import { onDestroy, onMount } from "svelte";

let { container, onClick, x, y, scale } = $props<{
  container: Container;
  onClick: () => void;
  x: number;
  y: number;
  scale?: number;
}>();

let buttonContainer: Container;
const BTN_SMALL_SPRITE = "/ui/button-small/button-small-2.png";
const RED_CROSS_ICON_SPRITE = "/ui/icons/red-cross.png";

onMount(async () => {
  const btnAssets = await Assets.load([
    BTN_SMALL_SPRITE,
    RED_CROSS_ICON_SPRITE,
  ]);

  btnAssets[BTN_SMALL_SPRITE].source.scaleMode = "nearest";

  const button = new FancyButton({
    icon: btnAssets[RED_CROSS_ICON_SPRITE],
    defaultIconAnchor: { x: 0.5, y: 0.5 },
    defaultView: btnAssets[BTN_SMALL_SPRITE],
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
