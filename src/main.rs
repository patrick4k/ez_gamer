#![feature(portable_simd)]
#![windows_subsystem = "windows"]

use std::fmt::Error;
use ggez::GameResult;

mod snake_example;
mod world;
mod entity;
mod position;

fn main() -> GameResult {
    // snake_example::launch()
    world::launch()
}
