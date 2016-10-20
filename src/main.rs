extern crate sdl2;
extern crate rand;


use std::{thread, time};

use rand::Rng;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;


const MAX_X: usize = 49;
const MAX_Y: usize = 49;
const CELL_WIDTH: isize = 25;
const CELL_HEIGHT: isize = 25;
const NCELLS: usize = (MAX_Y + 1) * (MAX_X + 1);
const FRAMETIME: u64 = 100;


fn init<'a>() -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life",
                ((MAX_X + 1) * CELL_WIDTH as usize) as u32,
                ((MAX_Y + 1) * CELL_HEIGHT as usize) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

fn get_cell(cells: &Vec<bool>, i: usize) -> Option<bool> {
    let (x, y) = get_coords(i);
    if let Some(i) = get_index(x, y) {
        Some(cells[i])
    } else {
        None
    }
}

fn count_neighbors(cells: &Vec<bool>, i: usize) -> i32 {
    let mut counter = 0;
    let (origx, origy) = get_coords(i);
    for ty in -1isize..2isize {
        for tx in (-1isize..2isize).filter(|tx| *tx != 0 || ty != 0) {
            if let Some(i) = get_index((origx as isize + tx), origy as isize + ty) {
                if let Some(x) = get_cell(cells, i) {
                    if x {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}


fn toggle_field(x: isize, y: isize, cells: &mut Vec<bool>) {
    let cell_x = x / CELL_WIDTH;
    let cell_y = y / CELL_HEIGHT;
    cells[get_index(cell_x, cell_y).unwrap()] = !cells[get_index(cell_x, cell_y).unwrap()]
}



fn get_index(x: isize, y: isize) -> Option<usize> {
    if x < 0 || y < 0 || x > MAX_X as isize || y > MAX_Y as isize {
        None
    } else {
        Some((x + (y * (MAX_X + 1) as isize)) as usize)
    }
}

fn get_coords(i: usize) -> (isize, isize) {
    ((i % (MAX_X + 1)) as isize, (i / (MAX_X + 1)) as isize)
}


fn main() {
    let mut rng = rand::thread_rng();
    let (mut r, mut events) = init();
    let bg = Color::RGB(0xff, 0xff, 0xff);
    let fg = Color::RGB(0, 0, 0);
    let mut cells: Vec<bool> = Vec::with_capacity(NCELLS as usize);
    let mut paused: bool = false;

    for _ in 0..NCELLS {
        cells.push(rng.gen());
    }

    'running: loop {
        let timer = thread::spawn(|| thread::sleep(time::Duration::from_millis(FRAMETIME)));
        let frametimer = time::Instant::now();
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => paused = !paused,
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    for cell in cells.iter_mut() {
                        *cell = true;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    for cell in cells.iter_mut() {
                        *cell = false;
                    }
                }
                Event::MouseButtonDown { x: x, y: y, .. } => {
                    if paused {
                        toggle_field(x as isize, y as isize, &mut cells);
                    }
                }
                _ => {}
            }
        }

        r.set_draw_color(bg);
        r.clear();
        r.set_draw_color(fg);
        for (cell, i) in cells.iter().zip(0..NCELLS) {
            if *cell {
                let (x, y) = get_coords(i);
                r.fill_rect(Rect::new((x * CELL_WIDTH) as i32,
                                         (y * CELL_HEIGHT) as i32,
                                         CELL_WIDTH as u32,
                                         CELL_HEIGHT as u32))
                    .unwrap();
            }
        }

        r.present();

        if !paused {
            let old_cells = cells.clone();
            for (cell, i) in old_cells.iter().zip(0usize..(NCELLS as usize)) {
                let neighbors = count_neighbors(&old_cells, i);
                if *cell {
                    if neighbors < 2 || neighbors > 3 {
                        cells[i] = false;
                    } else {
                        cells[i] = true;
                    }
                } else {
                    if neighbors == 3 {
                        cells[i] = true;
                    } else {
                        cells[i] = false;
                    }
                }
            }
        }










        timer.join();

    }
}
