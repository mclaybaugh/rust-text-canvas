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
    terminal::draw(map);


    // while (gameState.continue) {
    //     gameState = update(gameState);
    //     draw(gameState);
    //     wait(time);
    // }
}