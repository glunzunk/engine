use env_logger::{Builder, Env};
use log::{info, warn, error, debug};
use glunzunk_renderer::run;

use glunzunk_engine::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))] {
        let env = Env::default().default_filter_or("info");
        Builder::from_env(env).init();
    }    

    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    info!("Starting Glunzunk Editor");
    glunzunk_engine::init();
    info!("Starting Renderer...");
    // glunzunk_renderer::run().unwrap();

    info!("Creating Scene");
    // glunzunk_engine::scene::create("main");
    let mut scene = glunzunk_engine::scene::create("main");
}
