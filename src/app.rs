use std::any::Any;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{current, JoinHandle, sleep, spawn};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo};
use crate::vec::Vec3f;

use crate::world::{Entity, Transform, World};

const UPDATE_MIN_DELTA: u128 = 10;

pub fn run(world: Arc<RwLock<World>>) {
    let running = Arc::new(AtomicBool::new(true));
    {
        let update_world = Arc::clone(&world);
        let update_running = Arc::clone(&running);
        let update_join = spawn(move ||
            update_thread(update_world, update_running)
        );
    }
    {
        let entity = Entity::new(
            Arc::clone(&world),
            Option::None,
        );
        entity.get_transform().lock().unwrap().size = Vec3f::new(100.0, 100.0, 0.0);
        entity.get_transform().lock().unwrap().color = Color::RED;
        entity.get_transform().lock().unwrap().position = Vec3f::new(100.0, 100.0, 0.0);
        world.write().unwrap().add_entity(Arc::new(entity));
    }
    let window = Window::new_centered("title", (640, 480)).unwrap();
    window.run_loop(AppWindowHandler { running, world })
}

fn update_thread(world: Arc<RwLock<World>>, running: Arc<AtomicBool>) {
    let mut previous_time: u128 = current_time();
    while running.load(Ordering::Relaxed) {
        let current_time = current_time();
        let mut delta = current_time - previous_time;
        if delta < UPDATE_MIN_DELTA {
            sleep(Duration::from_millis((UPDATE_MIN_DELTA - delta) as u64));
            delta = UPDATE_MIN_DELTA;
        }
        for entity in world.read().unwrap().get_entities() {
            entity.update(&delta);
        }
    }
}

fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

struct AppWindowHandler {
    running: Arc<AtomicBool>,
    world: Arc<RwLock<World>>,
}

impl WindowHandler for AppWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let mut transforms: Vec<Transform> = Vec::new();
        for entity in self.world.read().unwrap().get_entities() {
            transforms.push(entity.copy_transform())
        }
        graphics.clear_screen(Color::WHITE);
        for transform in transforms {
            graphics.draw_rectangle(
                Rectangle::new(
                    Vector2::new(transform.position.x, transform.position.y),
                    Vector2::new(transform.position.x + transform.size.x, transform.position.y + transform.size.y),
                ),
                transform.color,
            );
        }
        helper.request_redraw();
    }
}
