fn main() {
    let rows: usize = 10;
    let cols: usize = 20;
    let bg_char = '-';

    let map = make_map(rows, cols, bg_char);
    for row in map.iter() {
        for col in row.iter() {
            print!("{}", col.text);
        }
        print!("\n");
    }
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    White,
    Black
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
                text_color: Color::White,
                bg_color: Color::Black
            });
        }
    }

    map
}