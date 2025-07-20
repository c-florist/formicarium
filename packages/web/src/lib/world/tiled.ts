import { Assets, Container, Rectangle, Sprite, Texture } from "pixi.js";

export type TiledMap = {
  width: number;
  height: number;
  tilewidth: number;
  tileheight: number;
  layers: TiledLayer[];
  tilesets: TiledTileset[];
};

export type TiledLayer = {
  name: string;
  type: string;
  width: number;
  height: number;
  chunks?: TiledChunk[];
  data?: number[];
  x?: number;
  y?: number;
};

export type TiledChunk = {
  data: number[];
  height: number;
  width: number;
  x: number;
  y: number;
};

export type TiledTileset = {
  firstgid: number;
  name: string;
  image: string;
  imagewidth: number;
  imageheight: number;
  tilewidth: number;
  tileheight: number;
  tilecount: number;
  columns: number;
};

export class TiledMapRenderer {
  private map: TiledMap;
  private tilesetTextures: Map<number, Texture> = new Map();

  constructor(map: TiledMap) {
    this.map = map;
  }

  async loadTilesets(basePath = "/background/") {
    for (const tileset of this.map.tilesets) {
      const texture = await Assets.load(basePath + tileset.image);
      texture.source.scaleMode = "nearest";
      // texture.source.alphaMode = "no-premultiply-alpha";

      // Create individual tile textures from the tileset
      const { tilewidth, tileheight, columns, tilecount } = tileset;

      for (let i = 0; i < tilecount; i++) {
        const col = i % columns;
        const row = Math.floor(i / columns);

        const x = col * tilewidth;
        const y = row * tileheight;

        const tileTexture = new Texture({
          source: texture.source,
          frame: new Rectangle(x, y, tilewidth, tileheight),
        });

        // Store with global tile ID (firstgid + local tile index)
        this.tilesetTextures.set(tileset.firstgid + i, tileTexture);
      }
    }
  }

  renderMap(scale = 1) {
    const mapContainer = new Container();

    for (const layer of this.map.layers) {
      if (layer.type !== "tilelayer") continue;

      const layerContainer = new Container();
      layerContainer.name = layer.name;

      if (layer.chunks) {
        // Handle infinite maps with chunks
        for (const chunk of layer.chunks) {
          this.renderChunk(chunk, layerContainer, scale);
        }
      } else if (layer.data) {
        // Handle fixed-size maps
        this.renderLayerData(
          layer.data,
          layer.width,
          layer.height,
          layerContainer,
          scale,
        );
      }

      mapContainer.addChild(layerContainer);
    }

    return mapContainer;
  }

  private renderChunk(chunk: TiledChunk, container: Container, scale = 1) {
    const { data, width, height, x: chunkX, y: chunkY } = chunk;

    for (let i = 0; i < data.length; i++) {
      const tileId = data[i];
      if (tileId === 0) continue; // Empty tile

      const texture = this.tilesetTextures.get(tileId);
      if (!texture) continue;

      const localX = i % width;
      const localY = Math.floor(i / width);

      const worldX = (chunkX + localX) * this.map.tilewidth * scale;
      const worldY = (chunkY + localY) * this.map.tileheight * scale;

      const sprite = new Sprite(texture);
      sprite.x = worldX;
      sprite.y = worldY;
      sprite.scale.set(scale);

      container.addChild(sprite);
    }
  }

  private renderLayerData(
    data: number[],
    width: number,
    height: number,
    container: Container,
    scale = 1,
  ) {
    for (let i = 0; i < data.length; i++) {
      const tileId = data[i];
      if (tileId === 0) continue; // Empty tile

      const texture = this.tilesetTextures.get(tileId);
      if (!texture) continue;

      const x = (i % width) * this.map.tilewidth * scale;
      const y = Math.floor(i / width) * this.map.tileheight * scale;

      const sprite = new Sprite(texture);
      sprite.x = x;
      sprite.y = y;
      sprite.scale.set(scale);

      container.addChild(sprite);
    }
  }

  getMapDimensions() {
    return {
      width: this.map.width * this.map.tilewidth,
      height: this.map.height * this.map.tileheight,
    };
  }
}

export async function loadTiledMap(mapPath: string) {
  const mapData = (await fetch(mapPath).then((res) => res.json())) as TiledMap;
  return new TiledMapRenderer(mapData);
}
