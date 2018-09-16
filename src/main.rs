///# rust-text-canvas
///## Modules:
///
///    main.rs
///        The thing that runs everything
///    canvas.rs
///        The core model
///
///    terminal_view.rs
///        Outputs model to terminal
///
///    menu_model.rs
///        Menu-specific things on top of canvas
///    menu_controller.rs
///        Handles input and updates menu_model
///
///    block_copter_model.rs
///    block_copter_controller.rs
///        First game implementation using canvas

mod canvas;
mod terminal_view;

fn main() {
    let rows: usize = 10;
    let cols: usize = 20;
    let bg_char = '-';

    let map = canvas::make_map(rows, cols, bg_char);
    for row in map.iter() {
        print!("{}", terminal_view::BG_RED);
        for col in row.iter() {
            print!("{}", col.text);
        }
        print!("\n");
    }
}