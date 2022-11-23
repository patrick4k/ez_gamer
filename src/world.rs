use std::fs;
use fxhash::FxHashMap;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;
use ggez::winit::dpi::Pixel;
use oorandom::Rand32;
use crate::entity::{Entity, Sprite};
use crate::position::GridPosition;

pub const GRID_SIZE: (i16, i16) = (60, 40);
pub const GRID_CELL_SIZE: (i16, i16) = (8, 8);

pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

pub const DESIRED_FPS: u32 = 4;

pub struct World {
    entities: Vec<Entity>,
    entity_resources: FxHashMap<String, Sprite>,
    rng: Rand32
}

impl World {
    pub fn new(_ctx: &mut Context) -> World {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        let rand_pos = GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1);

        let mut world = World {
            entities: vec![],
            entity_resources: FxHashMap::default(),
            rng
        };

        world.init_resources().expect("Error reading entity resources");
        let sprite = world.get_sprite("resources\\Entity\\TestEntity.json".to_string());
        world.entities.push(Entity::new_with_sprite(&world, rand_pos, sprite.clone()));
        world
    }

    fn init_resources(&mut self) -> GameResult {
        let files = glob::glob("resources/Entity/*").expect("Failed to read glob pattern");
        for file in files {
            let filename = file.as_ref().unwrap().to_str().unwrap();
            println!("Resource loading from: {}", filename);
            let file_content = fs::read_to_string(file.as_ref().unwrap())?;
            let sprite:Sprite = serde_json::from_str(file_content.as_str()).unwrap();
            self.entity_resources.insert(filename.parse().unwrap(), sprite);
        }
        Ok(())
    }

    fn get_sprite(&self, sprite_name: String) -> Sprite {
        self.entity_resources.get(&*sprite_name).unwrap().clone()
    }

    fn check_for_intersect(&self) -> Vec<(&str,&str)> {
        let mut occupied: (Vec<GridPosition>,Vec<&Entity>) = (vec![],vec![]);
        let mut collision_list: Vec<(&str, &str)> = vec![];
        for entity in &self.entities {
            for grid_space in entity.get_occupied_grids() {
                if !occupied.0.contains(&grid_space) {
                    occupied.0.push(grid_space);
                    occupied.1.push(entity);
                }
                else {
                    let index_of_pos = occupied.0.iter().position(|x| *x == grid_space).unwrap();
                    collision_list.push((entity.id(), occupied.1.get(index_of_pos).unwrap().id()));
                    println!("COLLIDE");
                }
            }
        }
        collision_list
    }

    fn handle_collisions(&mut self, collide_list: Vec<(&str, &str)>) {
        let mut entity1: &Entity = &Entity::default();
        let mut entity2: &Entity = &Entity::default();

        for collision in collide_list {
            for entity in &self.entities {
                if entity.id() == collision.0 {
                    entity1 = entity;
                }
                if entity.id() == collision.1 {
                    entity2 = entity;
                }
            }
        }

        self.kill(entity1);
        self.kill(entity2);
    }

    fn kill(&mut self, entity: &Entity) {
        let index_to_remove = self.entities.iter().position(|x| x == entity).unwrap();
        self.entities.remove(index_to_remove);
    }
}

impl EventHandler<GameError> for World {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while _ctx.time.check_update_time(DESIRED_FPS) {
            let rand_pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1);
            let sprite = self.get_sprite("resources\\Entity\\TestEntity.json".to_string());
            self.entities.push(Entity::new_with_sprite(self, rand_pos, sprite));
            let collide_list = self.check_for_intersect();

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for entity in &self.entities {
            entity.draw(&mut canvas);
        }
        canvas.finish(ctx)
    }
}

pub fn launch() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("world", "Patrick Kennedy")
        .window_setup(ggez::conf::WindowSetup::default().title("World"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    let world = World::new(&mut ctx);

    event::run(ctx, event_loop, world);
}
