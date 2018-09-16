pub enum Color {
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

pub struct TextPixel {
    pub text: char,
    pub text_color: Color,
    pub bg_color: Color
}

pub struct Coord {
    pub row: usize,
    pub col: usize
}

pub struct TextSprite {
    pub pixel_array: Vec<TextPixel>,
    pub coord: Coord
}

pub fn make_map (rows: usize, cols: usize, bg_text: char)
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