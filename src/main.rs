///# rust-text-canvas
///## Modules:
///
///    canvas.rs
///        The core model
///
///    terminal.rs
///        Formatting output for terminal and setting
///        up event listeners
///
///    menu.rs
///        Menu-specific things on top of canvas
///
///    blockcopter.rs
///        First game implementation using canvas

mod canvas;
mod terminal;

fn main() {
    let rows: usize = 10;
    let cols: usize = 20;
    let bg_char = '-';

    let map = canvas::make_map(rows, cols, bg_char);
    for row in map.iter() {
        print!("{}", terminal::BG_RED);
        for col in row.iter() {
            print!("{}", col.text);
        }
        print!("\n");
    }
}