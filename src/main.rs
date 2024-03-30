extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use rand::Rng;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, PressEvent};
use piston::window::WindowSettings;
use piston::{Button, Key};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELL_WIDTH: usize = 5;

const NO_CELLS_WIDTH: usize = WIDTH / CELL_WIDTH;
const NO_CELLS_HEIGHT: usize = HEIGHT / CELL_WIDTH;

pub struct App {
    gl: GlGraphics,
    cells: [[bool; NO_CELLS_HEIGHT]; NO_CELLS_WIDTH]
}

impl App {
    fn update(&mut self, _args: &UpdateArgs) {
        let mut generation = self.cells.clone();

        for i in 0..NO_CELLS_WIDTH {
            for j in 0..NO_CELLS_HEIGHT {
                let mut no_neighbors = 0;

                for n in -1..2 {
                    for m in -1..2 {
                        if n == 0 && m == 0 {
                            continue;
                        }
                        let cell_i = i as i32 + n;
                        let cell_j = j as i32 + m;
                        if cell_i < 0 || cell_i > NO_CELLS_WIDTH as i32 - 1 || cell_j < 0 || cell_j > NO_CELLS_HEIGHT as i32 - 1 {
                            continue;
                        }

                        if self.cells[cell_i as usize][cell_j as usize] {
                            no_neighbors += 1;
                        }
                    }
                }

                if self.cells[i][j] {
                    if !(no_neighbors == 2 || no_neighbors == 3) {
                        generation[i][j] = false;
                    }
                } else {
                    if no_neighbors == 3 {
                        generation[i][j] = true;
                    }
                }
            }
        }

        self.cells = generation;
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let cells = self.cells;  // For some reason I cannot use self in the closure

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for i in 0..NO_CELLS_WIDTH {
                for j in 0..NO_CELLS_HEIGHT {
                    if cells[i][j] {
                        let transform = c.transform.trans((i * CELL_WIDTH) as f64, (j * CELL_WIDTH) as f64);
                        rectangle([1.0, 1.0, 1.0, 1.0],
                                  rectangle::square(0.0, 0.0, CELL_WIDTH as f64 - 1.0),
                                  transform,
                                  gl);
                    }
                }
            }
        });
    }

    fn init_cells(&mut self) {
        for i in 0..NO_CELLS_WIDTH{
            for j in 0..NO_CELLS_HEIGHT {
                self.cells[i][j] = rand::thread_rng().gen_bool(0.5);
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Rust Game Of Life", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        cells: [[false; NO_CELLS_HEIGHT]; NO_CELLS_WIDTH]
    };

    app.init_cells();

    let mut event_settings = EventSettings::new();
    event_settings.set_max_fps(30);
    event_settings.set_ups(30);
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
            app.init_cells();
        }
    }
}
