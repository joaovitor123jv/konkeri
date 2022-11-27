extern crate sdl2;

use tiled::Loader;
use tiled::Map as TmxMap;
use sdl2::image::LoadTexture;
use std::collections::HashMap;

use crate::globals::Global;
use crate::point::Point;

pub struct Map<'map> {
    pub loaded_map: TmxMap,
    pub offset: Point,
    pub loaded_images: Vec<sdl2::render::Texture<'map>>,
    pub tiles_images: HashMap<u32, u32>,
    zoomed_offset: Point,
    zoomed_width: i32,
    zoomed_heigth: i32
}

impl <'map>Map<'map> {
    pub fn load(tmx_path: &str, texture_creator: &'map sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Self {
        let mut loader = Loader::new();
        let map = match loader.load_tmx_map(tmx_path) {
            Ok(map) => map,
            Err(_) => panic!("Failed to load map. Cannot continue game")
        };

        let mut taken_pathes = 0;

        let mut loaded_images: Vec<sdl2::render::Texture> = Vec::new();
        let mut pathes: HashMap<String, u32> = HashMap::new();

        // <tile_id, image_id>
        let mut tiles_images: HashMap<u32, u32> = HashMap::new();

        for tileset in map.tilesets() {
            match &tileset.image {
                Some(image) => {
                    let key: String = image.source.to_str().unwrap().to_string();

                    if !pathes.contains_key(&key) {
                        let texture = match texture_creator.load_texture(key.as_str()) {
                            Ok(texture) => texture,
                            Err(_) => panic!("Cannot load texture for {:?}", image.source)
                        };

                        pathes.insert(key, taken_pathes);

                        loaded_images.push(texture);
                        taken_pathes += 1;
                    }
                },
                None => {}
            };

            for (tile_id, tile) in tileset.tiles() {
                match &tile.image {
                    Some(image) => {
                        let key: String = image.source.to_str().unwrap().to_string();

                        if !pathes.contains_key(&key) {
                            let texture = match texture_creator.load_texture(key.as_str()) {
                                Ok(texture) => texture,
                                Err(_) => panic!("Cannot load texture for {:?}", image.source)
                            };

                            pathes.insert(key, taken_pathes);

                            loaded_images.push(texture);
                            tiles_images.insert(tile_id, taken_pathes);
                            taken_pathes += 1;

                        } else {
                            let image_id = pathes.get(&key).unwrap();
                            tiles_images.insert(tile_id, *image_id);
                        }
                    },
                    None => {
                        tiles_images.insert(tile_id, taken_pathes);
                    }
                };
                // println!("Tile {} type = {:?}", tile_id, tile.image);
            }
        }

        // println!("Loaded Images = {:?}", pathes);
        // println!("Tiles Images = {:?}", tiles_images);

        // let mut loaded_tiles: HashMap<u32, sdl2::render::Texture> = HashMap::new();

        Self {
            loaded_map: map,
            loaded_images,
            offset: Point::zeroed(),
            tiles_images,
            zoomed_offset: Point::zeroed(),
            zoomed_width: 0,
            zoomed_heigth: 0
        }
    }

    fn get_x_pos_iso(&self, row: i32, col: i32) -> i32 {
         (row *(self.zoomed_width)) - (col * (self.zoomed_width))
    }

    fn get_y_pos_iso(&self, row: i32, col: i32) -> i32 {
        (col + row) * (self.zoomed_heigth)
    }

    fn get_pos_iso(&self, row: i32, col: i32) -> Point {
        Point::new(
            self.get_x_pos_iso(row, col),
            self.get_y_pos_iso(row, col))
    }

    #[allow(dead_code)]
    fn get_x_pos_cart(&self, _row: i32, col: i32) -> i32 {
        col * (self.loaded_map.tile_width as i32)
    }

    #[allow(dead_code)]
    fn get_y_pos_cart(&self, row: i32, _col: i32) -> i32 {
        row * (self.loaded_map.tile_height as i32)
    }


    fn draw_tile(&self,
                 point: &Point,
                 tile: &tiled::LayerTile,
                 canvas: &mut sdl2::render::WindowCanvas,
                 globals: &Global) {
        let tileset = tile.get_tileset();
        // let tilecount = tileset.tilecount;

        let image_id = self.tiles_images.get(&tile.id()).unwrap();

        match self.loaded_images.get(*image_id as usize) {
            Some(texture) => {
                let _res = if tileset.tilecount == 1 {
                    canvas.copy(texture,
                                None,
                                sdl2::rect::Rect::new(point.x + self.zoomed_offset.x,
                                                      point.y + self.zoomed_offset.y,
                                                      ((tileset.tile_width as f32) * globals.zoom) as u32,
                                                      ((tileset.tile_height as f32) * globals.zoom) as u32))
                } else {
                    canvas.copy(texture,
                                None,
                                sdl2::rect::Rect::new(point.x + self.zoomed_offset.x,
                                                      point.y + self.zoomed_offset.y,
                                                      ((tileset.tile_width as f32) * globals.zoom) as u32,
                                                      ((tileset.tile_height as f32) * globals.zoom) as u32))
                };
            },
            None => {}
        };
    }


    fn draw_layer(&self,
                  layer: tiled::TileLayer,
                  canvas: &mut sdl2::render::WindowCanvas,
                  globals: &Global) {

        let height = layer.height().unwrap();
        let width = layer.width().unwrap();

        for row in 0..height as i32 {
            for col in 0..width as i32 {
                let pos_iso = self.get_pos_iso(row, col);

                if globals.viewport.contains(&pos_iso) {
                    match layer.get_tile(row, col) {
                        Some(tile) => self.draw_tile(&pos_iso, &tile, canvas, globals),
                        None => { }
                    };
                }
            }
        }


        // layer.map.orientation
    }

    pub fn calc_zoomed_values(&mut self, globals: &Global) {
        self.zoomed_offset.x = (self.offset.x as f32 * globals.zoom) as i32 - globals.viewport.point.x;
        self.zoomed_offset.y = (self.offset.y as f32 * globals.zoom) as i32 - globals.viewport.point.y;

        self.zoomed_width = (self.loaded_map.tile_width as f32 * globals.zoom) as i32 / 2;
        self.zoomed_heigth = (self.loaded_map.tile_height as f32 * globals.zoom) as i32 / 2;
    }

    pub fn render(&self, 
              canvas: &mut sdl2::render::WindowCanvas, 
              globals: &Global,
              ) {
        let layers = self.loaded_map.layers();
        for layer in layers {
            if layer.visible {
                let layer_type = layer.layer_type();
                match layer_type {
                    tiled::LayerType::TileLayer(layer) => self.draw_layer(layer, canvas, globals),
                    tiled::LayerType::ImageLayer(_) => {},
                    tiled::LayerType::GroupLayer(_) => {},
                    tiled::LayerType::ObjectLayer(_) => {}
                };
            }
        }
    }
}
