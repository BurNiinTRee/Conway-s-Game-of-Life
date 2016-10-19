# Conway's Game of Life
An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life "Wikipedia") in Rust.

## Instalation
Clone the repository and cd into it:
```
$ git clone https://github.com/burniintree/conway-s-game-of-life
$ cd conway-s-game-of-life
```
### Build it with Cargo:
```
$ cargo build --release
```
You can find it under `target/release/gol`
### or run it:
```
$ cargo run --release
```
### or install it:
```
$ cargo install
```

## Usage
```
$ gol
```
You can close it with Escape,
pause with Space,
kill everithing with c,
and let everithing live with f.
When the game is paused, you can click on cells to toggle their current status.
