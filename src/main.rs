use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use std::time::Duration;

mod test;

// is it possible to pass width and height as compile-time constants?
#[derive(Clone)]
struct Life {
    width: u32,
    height: u32,
    wrap_indexes: bool,
    data: Vec<bool>,
    next_value: fn(u32, u32, &Life) -> Result<bool, String>,
}

struct Walker {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl Walker {
    fn new(width:u32, height:u32) -> Walker {
        Walker{width, height, x:0, y:0}
    }
}

impl Iterator for Walker {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.x, self.y);

        self.x += 1;

        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        if value.1 == self.height {
            None
        } else {
            Some(value)
        }
    }
}

impl Life {
    #[allow(unused)]
    fn draw_blinker(&mut self, x: u32, y: u32) {
        self.set(x + 1, y, true);
        self.set(x + 1, y + 1, true);
        self.set(x + 1, y + 2, true);
    }

    #[allow(unused)]
    fn draw_glider(&mut self, x: u32, y: u32) {
        self.set(x + 1, y, true);
        self.set(x + 2, y + 1, true);
        self.set(x, y + 2, true);
        self.set(x + 1, y + 2, true);
        self.set(x + 2, y + 2, true);
    }

    fn walk(&self) -> Walker {
        Walker::new(self.width, self.height)
    }

    fn randomize(&mut self) {
        for (x, y) in self.walk() {
            self.set(x, y, rand::random::<bool>());
        }
    }

    fn wrap_coord(value: i64, max_value: u32) -> u32 {
        if value < 0 {
            (max_value as i64 + (value % (max_value as i64))) as u32
        } else {
            (value % (max_value as i64)) as u32
        }
    }

    fn count_neighbors(&self, x: u32, y: u32) -> i32 {
        let mut count = 0;

        for rel_x in -1..=1 {
            for rel_y in -1..=1 {
                if !(rel_x == 0 && rel_y == 0)
                    && self.at_wrap(x as i64 + rel_x, y as i64 + rel_y)
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn next(&self) -> Result<Life, String> {
        let mut result: Life = self.clone();

        for (x, y) in self.walk() {
            let next = (self.next_value)(x, y, self);
            result.set(x,y, next?)
        }

        Ok(result)
    }

    fn set(&mut self, x: u32, y: u32, value: bool) {
        *self.at_mut(x, y) = value;
    }

    fn at_wrap(&self, x: i64, y: i64) -> bool {
        if !self.wrap_indexes
            && (x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64))
        {
            return false;
        }

        // x and y should both, mathematically, be >= 0 and < width or height when this is called
        *self
            .data
            .get(
                (Life::wrap_coord(y, self.height) * self.width + Life::wrap_coord(x, self.width))
                    as usize,
            )
            .expect("Index out of bounds in at_wrap, internal programming error")
    }

    fn at(&self, x: u32, y: u32) -> Result<&bool, String> {
        self.data
            .get((y * self.width + x) as usize)
            .ok_or_else(|| "D'oh!".to_string())
    }

    fn at_mut(&mut self, x: u32, y: u32) -> &mut bool {
        self.data.get_mut((y * self.width + x) as usize).unwrap()
    }
}

fn conway_rules(x: u32, y: u32, life: &Life) -> Result<bool, String> {
    let neighbors = life.count_neighbors(x, y);
    let alive = life.at(x, y)?;

    Ok(match (alive, neighbors) {
        (true, 2) => true,
        (_, 3) => true,
        _ => false,
    })
}

// google says rust doesn't have constructors?
fn make_life(width: u32, height: u32) -> Life {
    Life {
        width,
        height,
        wrap_indexes: true,
        data: vec![false; (width * height) as usize],
        next_value: conway_rules,
    }
}

fn draw(canvas: &mut Canvas<Window>, life: &Life, greyscale: u8) {
    canvas.set_draw_color(Color::RGB(greyscale, greyscale, greyscale));

    for x in 0..life.width {
        for y in 0..life.height {
            if *life.at(x, y).expect("programmer error in simulation") {
                let _ = canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
    }
}

fn main() -> Result<(), String> {
    let context = sdl2::init();

    let sdl_context = match context {
        Ok(result) => result,
        Err(message) => {
            println!("SDL reported error: '{}'", message);
            return Ok(());
        }
    };

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(16.0, 16.0)?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame = 0;

    let mut history_of_life = std::collections::VecDeque::<Life>::new();

    history_of_life.push_back({
        let mut life = make_life(50, 50);
        life.randomize();
        life
    });

    'running: loop {
        frame += 1;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut grey = 125 - (10 * history_of_life.len());

        for life in &history_of_life {
            grey += 10;
            if grey >= 125 {
                grey = 255;
            }
            draw(&mut canvas, &life, grey as u8);
        }

        if frame % 3 == 0 {
            history_of_life.push_back(history_of_life.back().unwrap().next()?);

            if history_of_life.len() > 10 {
                history_of_life.pop_front();
            }
        }

        if frame % 100 == 0 {
            history_of_life.back_mut().unwrap().draw_glider(25, 20);
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
