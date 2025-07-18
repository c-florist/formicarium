import { Assets } from "pixi.js";

export const WORLD_ASSETS = {
  WORKER_ANT: { alias: "worker-ant", src: "/characters/worker-ant.json" },
  FOOD_SOURCE: { alias: "food", src: "/food/food-1.json" },
  FOREST: { alias: "forest", src: "/background/forest-terrain.json" }, // TODO: Deprecate
  TERRAIN: { alias: "terrain", src: "/background/terrain.json" },
  NEST: { alias: "nest", src: "/nests/big-stump.json" },
} as const;

export const NEST_TEXTURES = {
  TREE: "big-tree-0",
} as const;

export const DEFAULT_ANT_TEXTURE = "ant-down-0";

export const initialiseWorldAssets = async () => {
  await Assets.load(Object.values(WORLD_ASSETS));

  const NEAREST_SCALED_ASSETS = [
    WORLD_ASSETS.WORKER_ANT,
    WORLD_ASSETS.FOOD_SOURCE,
    WORLD_ASSETS.NEST,
  ];

  // Set scaleMode on all textures in certain assets that need to be scaled significantly
  for (const asset of NEAREST_SCALED_ASSETS) {
    const loadedAsset = Assets.get(asset.alias);
    for (const key in loadedAsset.textures) {
      loadedAsset.textures[key].source.scaleMode = "nearest";
    }
  }
};
