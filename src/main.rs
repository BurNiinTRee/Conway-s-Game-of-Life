//! TODO
//! replace all constants with fields of gol!!
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate sdl2;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate clap;



use std::{thread, time};

use rayon::prelude::*;

use rand::Rng;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;


#[derive(Debug, Copy, Clone)]
struct Gol {
    max_x: usize,
    max_y: usize,
    cell_width: isize,
    cell_height: isize,
    ncells: usize,
    frametime: u64,
    verbose: bool,
}



fn init<'a>(gol: Gol) -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life",
                ((gol.max_x + 1) * gol.cell_width as usize) as u32,
                ((gol.max_y + 1) * gol.cell_height as usize) as u32)
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

fn get_cell(cells: &[bool], i: usize, gol: Gol) -> Option<bool> {
    let (x, y) = get_coords(i, gol);
    if let Some(i) = get_index(x, y, gol) {
        Some(cells[i])
    } else {
        None
    }
}

fn count_neighbors(cells: &[bool], i: usize, gol: Gol) -> i32 {
    let mut counter = 0;
    let (origx, origy) = get_coords(i, gol);
    for ty in -1isize..2isize {
        for tx in (-1isize..2isize).filter(|tx| *tx != 0 || ty != 0) {
            if let Some(i) = get_index((origx as isize + tx), origy as isize + ty, gol) {
                if let Some(x) = get_cell(cells, i, gol) {
                    if x {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}


fn toggle_field(x: isize, y: isize, cells: &mut Vec<bool>, gol: Gol) {
    let cell_x = x / gol.cell_width;
    let cell_y = y / gol.cell_height;
    cells[get_index(cell_x, cell_y, gol).unwrap()] = !cells[get_index(cell_x, cell_y, gol).unwrap()]
}



fn get_index(x: isize, y: isize, gol: Gol) -> Option<usize> {
    if x < 0 || y < 0 || x > gol.max_x as isize || y > gol.max_y as isize {
        None
    } else {
        Some((x + (y * (gol.max_x + 1) as isize)) as usize)
    }
}

fn get_coords(i: usize, gol: Gol) -> (isize, isize) {
    ((i % (gol.max_x + 1)) as isize, (i / (gol.max_x + 1)) as isize)
}


fn main() {
    let matches = clap_app!(gol =>
        (version: "0.1")
        (author: "Lars Mühmel <larsmuehmel@web.de")
        (about: "Conway's Game of Life in Rust")
        (@arg max_x: -x +takes_value "Sets the amount of horizontall cells")
        (@arg max_y: -y +takes_value "Sets the amount of verticall cells")
        (@arg cell_width: -w --width +takes_value "Sets the width of a single cell")
        (@arg cell_height: -h --height +takes_value "Sets the height of a single cell")
        (@arg framerate: -f --framerate +takes_value "Sets the number of generations per second")
        (@arg verbose: -v --verbose "Sets, wether or not the actual framerate should be printed to stdout")
        ).get_matches();
    // create a settings struct
    let mut gol: Gol = Gol {
        max_x: 199,
        max_y: 199,
        frametime: 1000 / 30,
        ncells: 200 * 200,
        cell_width: 5,
        cell_height: 5,
        verbose: false,
    };

    if let Some(Ok(x)) = matches.value_of("max_x").map(str::parse::<usize>) {
        gol.max_x = x - 1;
    }
    if let Some(Ok(x)) = matches.value_of("max_y").map(str::parse::<usize>) {
        gol.max_y = x - 1;
    }
    gol.ncells = (gol.max_x + 1) * (gol.max_y + 1);
    if let Some(Ok(x)) = matches.value_of("framerate").map(str::parse::<u64>) {
        if x == 0 {
            gol.frametime = 0;
        } else {
            gol.frametime = 1000 / x;
        }
    }
    if let Some(Ok(x)) = matches.value_of("cell_width").map(str::parse) {
        gol.cell_width = x;
    }
    if let Some(Ok(x)) = matches.value_of("cell_height").map(str::parse) {
        gol.cell_height = x;
    }
    if matches.occurrences_of("verbose") != 0 {
        gol.verbose = true;
    }
    let mut rng = rand::thread_rng();
    let (mut r, mut events) = init(gol);
    let bg = Color::RGB(0xff, 0xff, 0xff);
    let fg = Color::RGB(0, 0, 0);
    let mut cells: Vec<bool> = Vec::with_capacity(gol.ncells as usize);
    let mut paused: bool = false;

    for _ in 0..gol.ncells {
        cells.push(rng.gen());
    }

    'running: loop {
        let frametimer = time::Instant::now();
        let timer =
            thread::spawn(move || thread::sleep(time::Duration::from_millis(gol.frametime)));
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
                        toggle_field(x as isize, y as isize, &mut cells, gol);
                    }
                }
                _ => {}
            }
        }

        r.set_draw_color(bg);
        r.clear();
        r.set_draw_color(fg);
        for (cell, i) in cells.iter().zip(0..gol.ncells) {
            if *cell {
                let (x, y) = get_coords(i, gol);
                r.fill_rect(Rect::new((x * gol.cell_width) as i32,
                                         (y * gol.cell_height) as i32,
                                         gol.cell_width as u32,
                                         gol.cell_height as u32))
                    .unwrap();
            }
        }

        r.present();

        if !paused {
            let old_cells = cells.clone();
            cells.par_iter_mut().enumerate().for_each(|(i, cell)| {
                let neighbors = count_neighbors(&old_cells, i, gol);
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
        if gol.verbose {
            println!("Framerate:\t{:5}",
                     1_000_000_000 / frametimer.elapsed().subsec_nanos());
        }

    }
}
