use log::{info, warn, error, debug};
use bevy_ecs::prelude::*;
use crate::object::primitive;

pub struct Scene {
    name: String,
    world: World,
    is_loaded: bool,
}

impl Scene {
    pub fn create(scene_name: &str) -> Self {
        let world = World::default();

        let scene = Self {
            name: scene_name.to_string(),
            world,
            is_loaded: false,
        };
        info!("Created scene: {}", scene_name);
        return scene;
    }

    pub fn add(&mut self, object: primitive) -> Entity {

        #[derive(Component)]
        struct Position { x: f32, y: f32 }

        #[derive(Component)]
        struct MeshPrimitive { mesh: String }

        let entity = self.world.spawn(
            ( Position { x: 0.0, y: 0.0 },
            MeshPrimitive { mesh: object.shape } )
        ).id();

        info!("Added `{}` to scene: {}", object.name, self.name);
        // return self.world.entity(entity);
        entity
    }
}

pub fn create(scene_name: &str) -> Scene {
    Scene::create(scene_name)
}
