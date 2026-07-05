use log::{info, warn, error, debug};
use bevy_ecs::prelude::*;

struct Scene {
    name: String,
    is_loaded: bool,
}

struct SceneData {
    metadata: Scene,
    root: Vec<Entity>, 
}


impl Scene {
    pub fn create(scene_name: &str) -> Self {
        Self {
            name: scene_name.to_string(),
            is_loaded: false,
        }
        // info!("Created scene '{}'", scene_name);
    }
}