use std::ops::DerefMut;
use std::sync::{Arc, Mutex, RwLock};
use speedy2d::color::Color;

use speedy2d::window::VirtualKeyCode::Mute;

use crate::game::{Entity, EntityBuilder, EntityEvent, EntityUpdate, Event};
use crate::vec::Vec3f;
use crate::World;

pub fn apply_debug(world: &Arc<RwLock<World>>) {
    {
        world.write().unwrap().add_entity(
            EntityBuilder::new()
                .position(Vec3f::new(100.0, 0.0, 100000.0))
                .size(Vec3f::new(100.0, 100.0, 1.0))
                .color(Color::CYAN)
                .build(Arc::clone(&world))
        )
    }
    {
        world.write().unwrap().add_entity(
            EntityBuilder::new()
                .position(Vec3f::new(0.0, 0.0, 0.0))
                .size(Vec3f::new(100.0, 100.0, 1.0))
                .color(Color::RED)
                .update_handler(DebugUpdate{})
                .event_handler(DebugEvent{})
                .build(Arc::clone(&world))
        )
    }
}

struct DebugUpdate {}

struct DebugEvent {}

impl EntityUpdate for DebugUpdate {
    fn update(&mut self, entity: &Entity, delta: &u128) {
        let speed = *delta as f32 / 200.0;
        entity.get_transform().lock().unwrap().position += Vec3f::new(speed, 0.0, speed);
    }
}

impl EntityEvent for DebugEvent {
    fn event(&mut self, entity: &Entity, event: &Event) {
        println!("{:?}", event)
    }
}