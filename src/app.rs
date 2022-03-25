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
use crate::debug::apply_debug;
use crate::vec::Vec3f;

use crate::game::{Entity, Event, Transform, World};

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
        apply_debug(&world)
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
        previous_time = current_time;
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

impl AppWindowHandler {
    fn exec_event(&mut self, event: &Event) {
        for entity in self.world.read().unwrap().get_entities() {
            entity.event(event)
        };
    }
}

fn cmp_f32(first: &f32, second: &f32) -> std::cmp::Ordering {
    if *first > *second {
        return std::cmp::Ordering::Greater
    }
    if *first == *second {
        return std::cmp::Ordering::Equal
    }
    std::cmp::Ordering::Less
}

impl WindowHandler for AppWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let mut transforms: Vec<Transform> = Vec::new();
        for entity in self.world.read().unwrap().get_entities() {
            transforms.push(entity.copy_transform())
        }
        transforms.sort_by(|first, second| cmp_f32(&second.position.z, &first.position.z));
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

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vector2<f32>) {
        self.exec_event(&Event::MouseMove(position))
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        self.exec_event(&Event::MouseDown(button))
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        self.exec_event(&Event::MouseUp(button))
    }

    fn on_mouse_wheel_scroll(&mut self, helper: &mut WindowHelper<()>, distance: MouseScrollDistance) {
        self.exec_event(&Event::MouseScroll(distance))
    }

    fn on_key_down(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key_code) = virtual_key_code {
            self.exec_event(&Event::KeyDown(key_code))
        }
    }

    fn on_key_up(&mut self, helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key_code) = virtual_key_code {
            self.exec_event(&Event::KeyUp(key_code))
        }
    }

}
