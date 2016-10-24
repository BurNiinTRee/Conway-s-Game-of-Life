#[macro_use]
extern crate clap;

fn main() {
    let mut app = clap_app!(gol =>
        (version: "0.1")
        (author: "Lars Mühmel <larsmuehmel@web.de")
        (about: "Conway's Game of Life in Rust")
        (@arg max_x: -x +takes_value "Sets the amount of horizontall cells")
        (@arg max_y: -y +takes_value "Sets the amount of verticall cells")
        (@arg cell_width: -w --width +takes_value "Sets the width of a single cell")
        (@arg cell_height: -h --height +takes_value "Sets the height of a single cell")
        (@arg framerate: -f --framerate +takes_value "Sets the number of generations per second")
        (@arg verbose: -v --verbose "Sets, wether or not the actual framerate should be printed to stdout")
        );
    app.gen_completions("gol", clap::Shell::Bash, env!("OUT_DIR"));
}