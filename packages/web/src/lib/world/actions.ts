import type { WorldDto } from "@formicarium/domain";
import type { Container, Rectangle } from "pixi.js";

export const setupPanning = ({
  appStage,
  hitArea,
  viewport,
  worldData,
}: {
  appStage: Container;
  hitArea: Rectangle;
  viewport: Container;
  worldData: WorldDto;
}) => {
  appStage.eventMode = "static";
  appStage.hitArea = hitArea;

  let dragging = false;
  const dragStart = { x: 0, y: 0 };

  appStage.on("pointerdown", (event) => {
    dragging = true;
    dragStart.x = event.global.x - viewport.x;
    dragStart.y = event.global.y - viewport.y;
  });

  appStage.on("pointerup", () => {
    dragging = false;
  });
  appStage.on("pointerupoutside", () => {
    dragging = false;
  });

  appStage.on("pointermove", (event) => {
    if (dragging) {
      const newX = event.global.x - dragStart.x;
      const newY = event.global.y - dragStart.y;
      const { width: worldWidth, height: worldHeight } = worldData;
      const { width: screenWidth, height: screenHeight } = hitArea;

      viewport.x = Math.max(Math.min(newX, 0), screenWidth - worldWidth);
      viewport.y = Math.max(Math.min(newY, 0), screenHeight - worldHeight);
    }
  });
};
