mod sprites;
mod entity;
mod stats;
mod loot;
mod dungeon;
mod entities;
mod game;


use std::time::{SystemTime, UNIX_EPOCH};
use ruscii::app::{App, Config, State};
use ruscii::drawing::{Drawable, Pencil, RectCharset};
use ruscii::gui::FPSCounter;
use ruscii::keyboard::Key;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};
use crate::dungeon::Dungeon;
use crate::entity::Entity;

fn main() {
    let mut app = App::config(Config::new().fps(1000));
    let mut fps_counter = FPSCounter::new();
    let mut last_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to parse duration").as_millis();
    let mut dungeon = Dungeon::generate();
    app.run(|app_state: &mut State, window: &mut Window| {
        let delta_time = calculate_delta(&mut last_time);
        fps_counter.update();
        dungeon.update(delta_time, app_state.keyboard().last_key_events());
        Pencil::new(window.canvas_mut())
            .draw(&dungeon)
            .draw_rect(&RectCharset::simple_lines(), Vec2::zero(), Vec2::xy(80, 30));
    });
}

fn calculate_delta(last_time: &mut u128) -> f32 {
    const LOW_LIMIT: f32 = 0.0167f32;
    const HIGH_LIMIT: f32 = 0.1f32;
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to parse duration").as_millis();
    let mut delta_time: f32 = (current_time - *last_time) as f32 / 1000f32;
    if delta_time < LOW_LIMIT {
        delta_time = LOW_LIMIT;
    } else if delta_time > HIGH_LIMIT {
        delta_time = HIGH_LIMIT;
    }
    *last_time = current_time;
    return delta_time;
}