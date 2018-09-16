///# rust-text-canvas
///## Modules:
///    main.rs
///
///    terminal_view.rs
///        Outputs model to terminal
///
///    canvas.rs
///
///    menu_model.rs
///
///    menu_controller.rs
///        Handles input and updates menu_model
///

mod terminal_view;

fn main() {
    let rows: usize = 10;
    let cols: usize = 20;
    let bg_char = '-';

    let map = make_map(rows, cols, bg_char);
    for row in map.iter() {
        print!("{}", terminal_view::BG_RED);
        for col in row.iter() {
            print!("{}", col.text);
        }
        print!("\n");
    }
}

enum Color {
    Normal,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

struct TextPixel {
    text: char,
    text_color: Color,
    bg_color: Color
}

struct Coord {
    row: usize,
    col: usize
}

struct TextSprite {
    pixel_array: Vec<TextPixel>,
    coord: Coord
}

fn make_map (rows: usize, cols: usize, bg_text: char)
-> Vec<Vec<TextPixel>> {
    let mut map: Vec<Vec<TextPixel>> = Vec::new();

    for row in 0..rows {
        map.push(Vec::new());
        for _col in 0..cols {
            map[row].push(TextPixel {
                text: bg_text,
                text_color: Color::Red,
                bg_color: Color::Normal
            });
        }
    }

    map
}