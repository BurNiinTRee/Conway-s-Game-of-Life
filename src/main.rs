#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate sdl2;
extern crate rand;
extern crate rayon;


use std::{thread, time};

use rayon::prelude::*;

use rand::Rng;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;


const MAX_X: usize = 199;
const MAX_Y: usize = 199;
const CELL_WIDTH: isize = 5;
const CELL_HEIGHT: isize = 5;
const NCELLS: usize = (MAX_Y + 1) * (MAX_X + 1);
const FRAMETIME: u64 = 1000 / 30;


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

fn get_cell(cells: &[bool], i: usize) -> Option<bool> {
    let (x, y) = get_coords(i);
    if let Some(i) = get_index(x, y) {
        Some(cells[i])
    } else {
        None
    }
}

fn count_neighbors(cells: &[bool], i: usize) -> i32 {
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
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => paused = !paused,
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    cells.par_iter_mut().for_each(|cell| {
                        *cell = true;
                    });
                }
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    cells.par_iter_mut().for_each(|cell| {
                        *cell = false;
                    });
                }
                Event::MouseButtonDown { x, y, .. } => {
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
            cells.par_iter_mut().enumerate().for_each(|(i, cell)| {
                let neighbors = count_neighbors(&old_cells, i);
                if old_cells[i] {
                    if neighbors < 2 || neighbors > 3 {
                        *cell = false;
                    } else {
                        *cell = true;
                    }
                } else if neighbors == 3 {
                    *cell = true;
                } else {
                    *cell = false;

                }
            });
        }
        timer.join().unwrap();

    }
}
