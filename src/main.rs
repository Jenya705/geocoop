use std::sync::{Arc, RwLock};
use crate::app::run;
use crate::world::World;

mod vec;
mod world;
mod app;

fn main() {
    run(Arc::new(RwLock::new(World::new())))
}
