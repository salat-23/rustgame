use rand::Rng;
use ruscii::drawing::{Drawable, Pencil};
use ruscii::keyboard::KeyEvent;
use ruscii::spatial::Vec2;
use ruscii::terminal::{Canvas, Color};
use crate::entity::Sprite;
use crate::sprites::{CHEST, DRAGON, GAME_NAME_TEXT, SLUG};

struct EventSystem {
    accumulative_delta_time: f64,
    second_tick: bool,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem { accumulative_delta_time: 0f64, second_tick: false }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.second_tick = false;

        self.accumulative_delta_time += delta_time;

        if self.accumulative_delta_time > 1f64 {
            self.accumulative_delta_time = 0f64;
            self.second_tick = true;
        }
    }
}

enum GameState {
    MainMenu
}

//We use lifetime parameter to ensure
// that key events reference will be alive as long as context is
pub struct GameLoopContext<'context_lifetime> {
    pub delta_time: f64,
    pub key_events: &'context_lifetime Vec<KeyEvent>,
    pub pencil: Pencil<'context_lifetime>,
}

const SPRITE_ARR_SIZE: usize = 3;

struct MainMenuSpriteGen {
    used_index: usize,
    available_sprites: [Sprite; SPRITE_ARR_SIZE],
}

impl MainMenuSpriteGen {
    fn new() -> MainMenuSpriteGen {
        MainMenuSpriteGen {
            used_index: 0,
            available_sprites: [
                Sprite::new(SLUG, Color::Magenta).centered(),
                Sprite::new(CHEST, Color::Red).centered(),
                Sprite::new(DRAGON, Color::Blue).centered()
            ],
        }
    }

    fn change(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let generated_index = rng.gen_range(0..SPRITE_ARR_SIZE);
            if generated_index != self.used_index {
                self.used_index = generated_index;
                return;
            }
        }
    }

    fn current(&self) -> &Sprite {
        &self.available_sprites[self.used_index]
    }
}


//================
//UI

trait UIElement {
    fn draw_ui(&self, pencil: Pencil);

    fn is_focused(&self) -> bool;

    fn focus(&mut self);

    fn unfocus(&mut self);

    fn can_focus(&self) -> bool {
        false
    }
}

struct UICanvas {
    dimensions: Vec2,
    ui_elements: Vec<Box<dyn UIElement>>,
    focused_index: i32,
}

impl UICanvas {
    fn new() -> UICanvas {
        UICanvas {
            dimensions: Vec2::xy(15i32, 1i32),
            ui_elements: Vec::new(),
            focused_index: 0,
        }
    }

    fn focus_next(&mut self) {
        loop {
            let mut index = self.focused_index + 1;
            let old_ui_element: &Box<dyn UIElement> = &mut self.ui_elements[self.focused_index as usize];
            old_ui_element.unfocus();
            if index >= self.ui_elements.len() as i32 {
                self.focused_index = 0;
                index = self.focused_index;
            }
            let mut new_ui_element: &Box<dyn UIElement> = &mut self.ui_elements[index as usize];
            if new_ui_element.can_focus() {
                new_ui_element.focus();
                return;
            }
        }
    }
}

impl Drawable for Box<dyn UIElement> {
    fn draw(&self, pencil: Pencil) {
        self.draw_ui(pencil)
    }
}

impl Drawable for UICanvas {
    fn draw(&self, mut pencil: Pencil) {
        for (i, element) in self.ui_elements.iter().enumerate() {
            pencil.set_origin(Vec2::xy(0i32, i));
            pencil.draw(element);
        }
    }
}

struct UIButton {
    index: i32,
    cords: Vec2,
    is_focused: bool,
    text: String,
    color: Color,
    background: Color,
    active_color: Color,
    active_background: Color,
}


impl UIElement for UIButton {
    fn draw_ui(&self, mut pencil: Pencil) {
        pencil.set_foreground(if self.is_focused() { self.active_color } else { self.color });
        pencil.set_background(if self.is_focused() { self.active_background } else { self.background });
        pencil.draw_text(self.text.as_str(), Vec2::zero());
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn focus(&mut self) {
        self.is_focused = true;
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }

    fn can_focus(&self) -> bool {
        true
    }
}


//===============

pub struct Game {
    state: GameState,
    drawing_cords: Vec2,
    drawing_dimensions: Vec2,
    event_system: EventSystem,
    menu_sprite_gen: MainMenuSpriteGen,
    main_menu_canvas: UICanvas,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::MainMenu,
            drawing_cords: Vec2::xy(1, 1),
            drawing_dimensions: Vec2::xy(78, 28),
            event_system: EventSystem::new(),
            menu_sprite_gen: MainMenuSpriteGen::new(),
            main_menu_canvas: UICanvas::new(),
        }
    }

    pub fn update(&mut self, context: GameLoopContext) {
        self.event_system.update(context.delta_time);
        match self.state {
            GameState::MainMenu => self.process_main_menu(context),
        }
    }

    fn process_main_menu(&mut self, context: GameLoopContext) {
        if self.event_system.second_tick {
            self.menu_sprite_gen.change();
        }

        self.draw_main_menu(context.pencil);
    }

    fn draw_main_menu(&mut self, mut pencil: Pencil) {
        pencil
            .draw_at(&Sprite::new(GAME_NAME_TEXT, Color::Green), Vec2::xy(7, 7))
            .draw_at(self.menu_sprite_gen.current(), Vec2::xy(50, 20))
            .draw_at(&self.main_menu_canvas, Vec2::xy(7, 10));
    }
}