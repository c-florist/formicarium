import { Assets, type AssetsManifest } from "pixi.js";

export const WORLD_MAP_CONFIG = {
  filePath: "/background/world-map-2.json",
  scale: 2.5,
};

export const ASSET_ALIASES = {
  WORKER_ANT: "worker-ant",
  WORLD_MAP: "world-map",
  NEST: "nest",
  GROUND_TILESET: "ground-tileset",
  PATH_TILESET: "path-tileset",
  GROUND_EDGE_TILESET: "ground-edge-tileset",
  TREE: "tree",
  DEAD_TREE: "dead-tree",
  SMALL_TREE: "small-tree",
  SMALL_STUMP: "small-stump",
  ROCK: "rock",
  SIGN: "sign",
  RED_FLOWER: "red-flower",
  BLUE_FLOWER: "blue-flower",
  ORANGE_FLOWER: "orange-flower",
  PINK_FLOWER: "pink-flower",
  FOOD_1: "food-1",
  FOOD_2: "food-2",
  FOOD_3: "food-3",
  FOOD_4: "food-4",
  FOOD_5: "food-5",
  FOOD_6: "food-6",
  FOOD_7: "food-7",
  FOOD_8: "food-8",
} as const;

export const FOOD_ASSET_ALIASES = [
  ASSET_ALIASES.FOOD_1,
  ASSET_ALIASES.FOOD_2,
  ASSET_ALIASES.FOOD_3,
  ASSET_ALIASES.FOOD_4,
  ASSET_ALIASES.FOOD_5,
  ASSET_ALIASES.FOOD_6,
  ASSET_ALIASES.FOOD_7,
  ASSET_ALIASES.FOOD_8,
];

export const CURSOR_DEFAULT = "url(/ui/cursor/cursor-default.png),auto";

const manifest: AssetsManifest = {
  bundles: [
    {
      name: "world",
      assets: [
        {
          alias: ASSET_ALIASES.WORKER_ANT,
          src: "/characters/worker-ant.json",
        },
        {
          alias: ASSET_ALIASES.FOOD_1,
          src: "/food/food-1.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_2,
          src: "/food/food-2.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_3,
          src: "/food/food-3.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_4,
          src: "/food/food-4.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_5,
          src: "/food/food-5.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_6,
          src: "/food/food-6.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_7,
          src: "/food/food-7.png",
        },
        {
          alias: ASSET_ALIASES.FOOD_8,
          src: "/food/food-8.png",
        },
        {
          alias: ASSET_ALIASES.WORLD_MAP,
          src: WORLD_MAP_CONFIG.filePath,
        },
        {
          alias: ASSET_ALIASES.NEST,
          src: "/nests/big-stump.png",
        },
        {
          alias: ASSET_ALIASES.GROUND_TILESET,
          src: "/background/ground-tileset.png",
        },
        {
          alias: ASSET_ALIASES.PATH_TILESET,
          src: "/background/path-tileset.png",
        },
        {
          alias: ASSET_ALIASES.GROUND_EDGE_TILESET,
          src: "/background/ground-edge-tileset.png",
        },
        { alias: ASSET_ALIASES.TREE, src: "/background/tree.png" },
        {
          alias: ASSET_ALIASES.DEAD_TREE,
          src: "/background/dead-tree.png",
        },
        {
          alias: ASSET_ALIASES.SMALL_TREE,
          src: "/background/small-tree.png",
        },
        {
          alias: ASSET_ALIASES.SMALL_STUMP,
          src: "/background/small-stump.png",
        },
        { alias: ASSET_ALIASES.ROCK, src: "/background/rock.png" },
        { alias: ASSET_ALIASES.SIGN, src: "/background/sign.png" },
        { alias: ASSET_ALIASES.RED_FLOWER, src: "/background/red-flower.png" },
        {
          alias: ASSET_ALIASES.BLUE_FLOWER,
          src: "/background/blue-flower.png",
        },
        {
          alias: ASSET_ALIASES.ORANGE_FLOWER,
          src: "/background/orange-flower.png",
        },
        {
          alias: ASSET_ALIASES.PINK_FLOWER,
          src: "/background/pink-flower.png",
        },
      ],
    },
  ],
};

export const initialiseWorldAssets = async () => {
  await Assets.init({ manifest });
  await Assets.loadBundle("world");

  // Assets can be spritesheets or textures, so setting the scale mode is different for each
  for (const asset of Object.values(ASSET_ALIASES)) {
    const loadedAsset = Assets.get(asset);
    if (loadedAsset.textures) {
      for (const key in loadedAsset.textures) {
        loadedAsset.textures[key].source.scaleMode = "nearest";
      }
    } else if (loadedAsset.isTexture) {
      loadedAsset.source.scaleMode = "nearest";
    }
  }
};
