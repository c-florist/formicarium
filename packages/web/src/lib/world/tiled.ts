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
  image?: string;
  imagewidth?: number;
  imageheight?: number;
  tilewidth: number;
  tileheight: number;
  tilecount: number;
  columns?: number;
  spacing?: number;
  tiles?: TiledTileData[];
};

export type TiledTileData = {
  id: number;
  type?: string;
  image?: string;
};

export class TiledMapRenderer {
  private map: TiledMap;
  private tilesetTextures: Map<number, Texture> = new Map();
  private tileTypes: Map<number, string> = new Map();

  constructor(map: TiledMap) {
    this.map = map;
  }

  static async fromFile(path: string) {
    const mapFile = await fetch(path);
    const map = await mapFile.json();
    return new TiledMapRenderer(map);
  }

  loadTilesets() {
    for (const tileset of this.map.tilesets) {
      if (tileset.image) {
        // Tileset with tiles embedded in one image
        const texture = Assets.get(tileset.name);

        for (let i = 0; i < tileset.tilecount; i++) {
          const localId = i;
          const globalId = tileset.firstgid + localId;
          const tileInfo = tileset.tiles?.find((t) => t.id === localId);
          if (tileInfo?.type) {
            this.tileTypes.set(globalId, tileInfo.type);
          }

          const { tilewidth, tileheight, columns = 1, spacing = 0 } = tileset;
          const col = i % columns;
          const row = Math.floor(i / columns);
          const x = col * (tilewidth + spacing);
          const y = row * (tileheight + spacing);

          const tileTexture = new Texture({
            source: texture.source,
            frame: new Rectangle(x, y, tilewidth, tileheight),
          });
          this.tilesetTextures.set(globalId, tileTexture);
        }
      } else if (tileset.tiles) {
        // Otherwise it's a collection of individual images
        for (const tileData of tileset.tiles) {
          if (tileData.image) {
            const textureName = tileData.image.replace(/\.png$/, "");
            const texture = Assets.get(textureName);
            const globalId = tileset.firstgid + tileData.id;
            this.tilesetTextures.set(globalId, texture);
            if (tileData.type) {
              this.tileTypes.set(globalId, tileData.type);
            }
          }
        }
      }
    }
  }

  renderMap(scale: number) {
    const background = new Container();
    const foreground = new Container();

    for (const layer of this.map.layers) {
      if (layer.data) {
        this.renderLayerData(
          layer.data,
          layer.width,
          background,
          foreground,
          scale,
        );
      }
    }

    return { background, foreground };
  }

  private renderLayerData(
    data: number[],
    width: number,
    background: Container,
    foreground: Container,
    scale: number,
  ) {
    for (let i = 0; i < data.length; i++) {
      const tileId = data[i];
      if (tileId === 0) continue;

      const texture = this.tilesetTextures.get(tileId);
      if (!texture) continue;

      const x = (i % width) * this.map.tilewidth * scale;
      const y = Math.floor(i / width) * this.map.tileheight * scale;

      const sprite = new Sprite(texture);
      sprite.x = x;
      sprite.y = y;
      sprite.scale.set(scale);

      const type = this.tileTypes.get(tileId);
      if (type === "foreground") {
        foreground.addChild(sprite);
      } else {
        background.addChild(sprite);
      }
    }
  }

  getMapDimensions() {
    return {
      width: this.map.width * this.map.tilewidth,
      height: this.map.height * this.map.tileheight,
    };
  }
}
