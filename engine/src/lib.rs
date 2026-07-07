// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

use log::{info, warn, error, debug};
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;


pub mod scene;
pub mod object;

pub fn init() {
    info!("Initialising Glunzunk Engine...");

    game_loop()
}

fn game_loop() {}