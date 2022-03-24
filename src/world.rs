use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};

use speedy2d::color::Color;
use speedy2d::window::VirtualKeyCode::Mute;

use crate::vec::Vec3f;

type UpdateHandler = Option<Arc<Mutex<dyn EntityUpdate + Send + Sync>>>;

#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vec3f,
    pub size: Vec3f,
    pub color: Color,
}

pub struct Entity {
    transform: Arc<Mutex<Transform>>,
    world: Arc<RwLock<World>>,
    update_handler: UpdateHandler,
}

pub trait EntityUpdate {
    fn update(&mut self, entity: &Entity, delta: &u128);
}

pub struct World {
    entities: Vec<Arc<Entity>>,
}

impl Entity {

    pub fn new(world: Arc<RwLock<World>>, update_handler: UpdateHandler) -> Entity {
        Entity {
            transform: Arc::new(Mutex::new(Transform {
                position: Vec3f::default(),
                size: Vec3f::default(),
                color: Color::BLACK,
            })),
            world,
            update_handler
        }
    }

    pub fn get_transform(&self) -> &Arc<Mutex<Transform>> {
        &self.transform
    }

    pub fn copy_transform(&self) -> Transform {
        self.transform.lock().unwrap().deref().clone()
    }

    pub fn get_world(&self) -> &Arc<RwLock<World>> {
        &self.world
    }

    pub fn update(&self, delta: &u128) {
        if let Some(update) = &self.update_handler {
            update.lock().unwrap().deref_mut().update(self, delta)
        }
    }

}

impl World {

    pub fn new() -> World {
        World {
            entities: Vec::new(),
        }
    }

    pub fn get_entities(&self) -> &Vec<Arc<Entity>> {
        &self.entities
    }

    pub fn add_entity(&mut self, entity: Arc<Entity>) {
        self.entities.push(entity)
    }

}