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

  renderMap() {
    const background = new Container();
    const foreground = new Container();

    for (const layer of this.map.layers) {
      if (layer.data) {
        this.renderLayerData(layer.data, layer.width, background, foreground);
      }
    }

    return { background, foreground };
  }

  private extractTileIdAndFlags(tileId: number) {
    const FLIPPED_HORIZONTALLY_FLAG = 0x80000000;
    const FLIPPED_VERTICALLY_FLAG = 0x40000000;
    const FLIPPED_DIAGONALLY_FLAG = 0x20000000;

    const flippedHorizontally = (tileId & FLIPPED_HORIZONTALLY_FLAG) !== 0;
    const flippedVertically = (tileId & FLIPPED_VERTICALLY_FLAG) !== 0;
    const flippedDiagonally = (tileId & FLIPPED_DIAGONALLY_FLAG) !== 0;

    // Clear the flags to get the actual tile ID
    const id =
      tileId &
      ~(
        FLIPPED_HORIZONTALLY_FLAG |
        FLIPPED_VERTICALLY_FLAG |
        FLIPPED_DIAGONALLY_FLAG
      );

    return {
      id,
      flippedHorizontally,
      flippedVertically,
      flippedDiagonally,
    };
  }

  private renderLayerData(
    data: number[],
    width: number,
    background: Container,
    foreground: Container,
  ) {
    for (let i = 0; i < data.length; i++) {
      const rawTileId = data[i];
      if (rawTileId === 0) {
        continue;
      }

      const { id, flippedHorizontally, flippedVertically, flippedDiagonally } =
        this.extractTileIdAndFlags(rawTileId);

      const texture = this.tilesetTextures.get(id);
      if (!texture) {
        continue;
      }

      const x = (i % width) * this.map.tilewidth;
      const y = Math.floor(i / width) * this.map.tileheight;

      const sprite = new Sprite(texture);
      sprite.x = x;
      sprite.y = y;

      // Apply transformations based on the flags
      if (flippedDiagonally) {
        sprite.anchor.set(0.5);
        sprite.rotation = -Math.PI / 2;
        sprite.x += this.map.tilewidth * 0.5;
        sprite.y += this.map.tileheight * 0.5;
        if (flippedHorizontally) {
          sprite.scale.y *= -1;
        }
        if (flippedVertically) {
          sprite.scale.x *= -1;
        }
      } else {
        if (flippedHorizontally) {
          sprite.scale.x *= -1;
          sprite.x += this.map.tilewidth;
        }
        if (flippedVertically) {
          sprite.scale.y *= -1;
          sprite.y += this.map.tileheight;
        }
      }

      const type = this.tileTypes.get(id);
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
