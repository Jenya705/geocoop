use std::sync::{Arc, RwLock};
use crate::app::run;
use crate::game::World;

mod vec;
mod game;
mod app;
mod debug;

fn main() {
    run(Arc::new(RwLock::new(World::new())))
}
