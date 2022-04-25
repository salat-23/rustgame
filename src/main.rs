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
use ruscii::spatial::Vec2;
use ruscii::terminal::{Window};
use crate::dungeon::Dungeon;
use crate::entity::Entity;
use chrono::{Datelike, Local, Timelike};
use crate::game::{Game, GameLoopContext};

fn main() {
    let mut app = App::config(Config::new().fps(60));
    let mut fps_counter = FPSCounter::new();
    let mut dungeon = Dungeon::generate();
    let mut game = Game::new();


    let mut last_time = get_current_time_epoch();
    app.run(|app_state: &mut State, window: &mut Window| {
        let delta_time = calculate_delta(&mut last_time);
        fps_counter.update();
        //Invoke update function and pass necessary data
        //Struct GameLoopContext is used instead of passing several arguments
        game.update(
            GameLoopContext {
                delta_time,
                key_events: app_state.keyboard().last_key_events(),
                pencil: Pencil::new(window.canvas_mut()),
            });

        Pencil::new(window.canvas_mut())
            .draw_rect(&RectCharset::simple_lines(), Vec2::zero(), Vec2::xy(80, 30))
            .draw_text(get_fps_string(fps_counter.count()).as_str(), Vec2::x(3))
            .draw_text(get_current_time_formatted().as_str(), Vec2::x(15));
    });
}

fn calculate_delta(last_time: &mut u128) -> f64 {
    const LOW_LIMIT: f64 = 0.0167f64;
    const HIGH_LIMIT: f64 = 0.1f64;
    let current_time = get_current_time_epoch();
    let mut delta_time: f64 = (current_time - *last_time) as f64 / 1000f64;
    if delta_time < LOW_LIMIT {
        delta_time = LOW_LIMIT;
    } else if delta_time > HIGH_LIMIT {
        delta_time = HIGH_LIMIT;
    }
    *last_time = current_time;
    return delta_time;
}

fn get_current_time_epoch() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to parse duration").as_millis()
}

fn get_current_time_formatted() -> String {
    let time = Local::now();

    String::from(
        std::format!(
            "{}:{}:{} - {} {}",
            time.hour(),
            time.minute(),
            time.second(),
            time.weekday(),
            time.date().format("%D")
        )
    )
}

fn get_fps_string(fps: u32) -> String {
    (String::from("Fps: ") + fps.to_string().as_str())
}