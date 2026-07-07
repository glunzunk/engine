use log::{info, warn, error, debug};
use bevy_ecs::prelude::*;

pub struct primitive {
    pub name: String,
    pub shape: String,
}

impl primitive {
    pub const CUBE: &'static str = "Cube";
    pub const SPHERE: &'static str = "Sphere";
    pub const PYRAMID: &'static str = "Pyramid";

    pub fn create(obj_name: &str, shape: &str) -> Self {
        let object = Self {
            name: obj_name.to_string(),
            shape: shape.to_string(),
        };
        info!("Created object `{}` with primitive mesh: {}", obj_name, shape);
        return object;
    }
}

// pub fn create(scene_name: &str) -> Scene {
//     Scene::create(scene_name)
// }