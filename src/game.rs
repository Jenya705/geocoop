use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::current;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{MouseButton, MouseScrollDistance, VirtualKeyCode};

use crate::vec::Vec3f;

type UpdateHandler = Option<Arc<Mutex<dyn EntityUpdate + Send + Sync>>>;
type EventHandler = Option<Arc<Mutex<dyn EntityEvent + Send + Sync>>>;

#[derive(Copy, Clone)]
pub struct Transform {
    pub position: Vec3f,
    pub size: Vec3f,
    pub color: Color,
}

pub struct Entity {
    id: AtomicU64,
    transform: Arc<Mutex<Transform>>,
    world: Arc<RwLock<World>>,
    update_handler: UpdateHandler,
    event_handler: EventHandler,
}

pub trait EntityUpdate {
    fn update(&mut self, entity: &Entity, delta: &u128);
}

#[derive(Debug)]
pub enum Event {
    KeyDown(VirtualKeyCode),
    KeyUp(VirtualKeyCode),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove(Vector2<f32>),
    MouseScroll(MouseScrollDistance),
}

pub trait EntityEvent {
    fn event(&mut self, entity: &Entity, event: &Event);
}

pub struct World {
    current_entity_id: AtomicU64,
    entities: Vec<Arc<Entity>>,
}

#[derive(Clone)]
pub struct EntityBuilder {
    transform: Transform,
    update_handler: UpdateHandler,
    event_handler: EventHandler,
}

impl Transform {

    pub fn for_render(&self) -> Transform {
        
    }

}

impl Entity {

    pub fn get_transform(&self) -> &Arc<Mutex<Transform>> {
        &self.transform
    }

    pub fn copy_transform(&self) -> Transform {
        self.transform.lock().unwrap().deref().clone()
    }

    pub fn get_world(&self) -> &Arc<RwLock<World>> {
        &self.world
    }

    pub fn set_id(&self, id: u64) {
        self.id.store(id, Ordering::Relaxed);
    }

    pub fn update(&self, delta: &u128) {
        if let Some(update) = &self.update_handler {
            update.lock().unwrap().deref_mut().update(self, delta)
        }
    }

    pub fn event(&self, event: &Event) {
        if let Some(handler) = &self.event_handler {
            handler.lock().unwrap().deref_mut().event(self, event)
        }
    }
}

impl World {

    pub fn new() -> World {
        World {
            current_entity_id: AtomicU64::new(1),
            entities: Vec::new(),
        }
    }

    pub fn get_entities(&self) -> &Vec<Arc<Entity>> {
        &self.entities
    }

    pub fn add_entity(&mut self, entity: Arc<Entity>) {
        entity.set_id(self.current_entity_id.fetch_add(1, Ordering::Relaxed));
        self.entities.push(entity)
    }

}

impl EntityBuilder {
    pub fn new() -> EntityBuilder {
        EntityBuilder {
            transform: Transform {
                position: Vec3f::default(),
                size: Vec3f::default(),
                color: Color::TRANSPARENT,
            },
            event_handler: Option::None,
            update_handler: Option::None,
        }
    }

    pub fn position(mut self, position: Vec3f) -> EntityBuilder {
        self.transform.position = position;
        self
    }

    pub fn size(mut self, size: Vec3f) -> EntityBuilder {
        self.transform.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> EntityBuilder {
        self.transform.color = color;
        self
    }

    pub fn update_handler<H>(mut self, update_handler: H) -> EntityBuilder
        where H: EntityUpdate + Send + Sync + 'static
    {
        self.update_handler = Option::Some(Arc::new(Mutex::new(update_handler)));
        self
    }

    pub fn event_handler<H>(mut self, event_handler: H) -> EntityBuilder
        where H: EntityEvent + Send + Sync + 'static
    {
        self.event_handler = Option::Some(Arc::new(Mutex::new(event_handler)));
        self
    }

    pub fn build(self, world: &Arc<RwLock<World>>) -> Arc<Entity> {
        Arc::new(Entity {
            id: AtomicU64::new(0),
            transform: Arc::new(Mutex::new(self.transform)),
            update_handler: self.update_handler.clone(),
            event_handler: self.event_handler.clone(),
            world: Arc::clone(world),
        })
    }
}