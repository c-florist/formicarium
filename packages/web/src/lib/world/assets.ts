import { Assets, type AssetsManifest } from "pixi.js";

export const ASSET_ALIASES = {
  WORKER_ANT: "worker-ant",
  FOOD_SOURCE: "food-source",
  WORLD_MAP: "world-map",
  NEST: "nest",
} as const;

export const CURSOR_DEFAULT = "url(/ui/cursor/cursor-default.png),auto";

const manifest: AssetsManifest = {
  bundles: [
    {
      name: "world",
      assets: [
        {
          alias: ASSET_ALIASES.WORKER_ANT,
          src: "/characters/worker-ant.json",
          data: {
            scaleMode: "nearest",
          },
        },
        {
          alias: ASSET_ALIASES.FOOD_SOURCE,
          src: "/food/food-1.json",
          data: {
            scaleMode: "nearest",
          },
        },
        {
          alias: ASSET_ALIASES.WORLD_MAP,
          src: "/background/world-map-2.json",
          data: {
            scaleMode: "nearest",
          },
        },
        {
          alias: ASSET_ALIASES.NEST,
          src: "/nests/big-stump.png",
          data: {
            scaleMode: "nearest",
          },
        },
      ],
    },
  ],
};

export const initialiseWorldAssets = async () => {
  await Assets.init({ manifest });
  await Assets.loadBundle("world");
};
