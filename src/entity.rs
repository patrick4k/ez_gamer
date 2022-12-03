use std::io::Error;
use std::fs;
use std::simd::i16x4;
use fxhash::FxHashMap;
use ggez::{GameResult, graphics};
use serde::{Serialize, Deserialize};
use crate::position::GridPosition;
use crate::world::World;

#[derive(PartialEq)]
pub struct Entity {
    pos: GridPosition,
    sprite: Sprite,
    state: EntityState
}

impl Entity {
    pub fn default() -> Self {
        Entity {
            pos: GridPosition::default(),
            sprite: Sprite::default(),
            state: EntityState::DEFAULT
        }
    }

    pub fn new(pos: GridPosition) -> Self {
        Entity {
            pos,
            sprite: Sprite::default(),
            state: EntityState::DEFAULT
        }
    }

    pub fn new_with_sprite(pos: GridPosition, sprite: Sprite) -> Self {
        Entity {
            pos,
            sprite,
            state: EntityState::DEFAULT
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        self.sprite.draw(self.pos, canvas);
    }

    pub fn get_occupied_grids(&self) -> Vec<GridPosition> {
        let mut occupied: Vec<GridPosition> = vec![];
        for pixel in &self.sprite.pixels {
            let pixel_pos = self.pos.new_from_offset(pixel.offset);
            if !occupied.contains(&pixel_pos) {
                occupied.push(pixel_pos)
            }
        }
        occupied
    }

    pub fn pos(&self) -> GridPosition {
        self.pos
    }
    pub fn state(&self) -> &EntityState {
        &self.state
    }
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Sprite {
    pixels: Vec<Pixel>
}

impl Sprite {
    fn default() -> Self {
        Sprite{ pixels: vec![] }
    }

    fn draw(&self, pos: GridPosition, canvas: &mut graphics::Canvas) {
        for pixel in &self.pixels {
            let pix_pos = pos.new_from_offset(pixel.offset);
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(pix_pos.into())
                    .color(pixel.color),
            );
        }
    }
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Sprite {
            pixels: self.pixels.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Pixel {
    color: [f32; 4],
    offset: [i16; 2]
}

#[derive(PartialEq)]
enum EntityState {
    DEFAULT,
}
