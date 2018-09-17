/* ANSI color escape codes 
 * 
 * Good resource at: http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
 *
*/

use canvas;

/* Reset code */
pub const RESET: &str = "\x1B[0m";
/* Basic colors */
pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";
pub const WHITE: &str = "\x1B[37m";
/* Bright colors */
pub const BRIGHT_BLACK: &str = "\x1B[30;1m";
pub const BRIGHT_RED: &str = "\x1B[31;1m";
pub const BRIGHT_GREEN: &str = "\x1B[32;1m";
pub const BRIGHT_YELLOW: &str = "\x1B[33;1m";
pub const BRIGHT_BLUE: &str = "\x1B[34;1m";
pub const BRIGHT_MAGENTA: &str = "\x1B[35;1m";
pub const BRIGHT_CYAN: &str = "\x1B[36;1m";
pub const BRIGHT_WHITE: &str = "\x1B[37;1m";
/* Background basic colors */
pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";
/* Background bright colors */
pub const BG_BRIGHT_BLACK: &str = "\x1B[40;1m";
pub const BG_BRIGHT_RED: &str = "\x1B[41;1m";
pub const BG_BRIGHT_GREEN: &str = "\x1B[42;1m";
pub const BG_BRIGHT_YELLOW: &str = "\x1B[43;1m";
pub const BG_BRIGHT_BLUE: &str = "\x1B[44;1m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1B[45;1m";
pub const BG_BRIGHT_CYAN: &str = "\x1B[46;1m";
pub const BG_BRIGHT_WHITE: &str = "\x1B[47;1m";

fn get_term_color(color: &canvas::Color) -> String {
    return match color {
        canvas::Color::Normal => String::from(RESET),
        canvas::Color::Black => String::from(BLACK),
        canvas::Color::Red => String::from(RED),
        canvas::Color::Green => String::from(GREEN),
        canvas::Color::Yellow => String::from(YELLOW),
        canvas::Color::Blue => String::from(BLUE),
        canvas::Color::Magenta => String::from(MAGENTA),
        canvas::Color::Cyan => String::from(CYAN),
        canvas::Color::White => String::from(WHITE),
        canvas::Color::BrightBlack => String::from(BRIGHT_BLACK),
        canvas::Color::BrightRed => String::from(BRIGHT_RED),
        canvas::Color::BrightGreen => String::from(BRIGHT_GREEN),
        canvas::Color::BrightYellow => String::from(BRIGHT_YELLOW),
        canvas::Color::BrightBlue => String::from(BRIGHT_BLUE),
        canvas::Color::BrightMagenta => String::from(BRIGHT_MAGENTA),
        canvas::Color::BrightCyan => String::from(BRIGHT_CYAN),
        canvas::Color::BrightWhite => String::from(BRIGHT_WHITE)
    };
}

fn get_bg_term_color(color: &canvas::Color) -> String {
    return match color {
        canvas::Color::Normal => String::from(RESET),
        canvas::Color::Black => String::from(BG_BLACK),
        canvas::Color::Red => String::from(BG_RED),
        canvas::Color::Green => String::from(BG_GREEN),
        canvas::Color::Yellow => String::from(BG_YELLOW),
        canvas::Color::Blue => String::from(BG_BLUE),
        canvas::Color::Magenta => String::from(BG_MAGENTA),
        canvas::Color::Cyan => String::from(BG_CYAN),
        canvas::Color::White => String::from(BG_WHITE),
        canvas::Color::BrightBlack => String::from(BG_BRIGHT_BLACK),
        canvas::Color::BrightRed => String::from(BG_BRIGHT_RED),
        canvas::Color::BrightGreen => String::from(BG_BRIGHT_GREEN),
        canvas::Color::BrightYellow => String::from(BG_BRIGHT_YELLOW),
        canvas::Color::BrightBlue => String::from(BG_BRIGHT_BLUE),
        canvas::Color::BrightMagenta => String::from(BG_BRIGHT_MAGENTA),
        canvas::Color::BrightCyan => String::from(BG_BRIGHT_CYAN),
        canvas::Color::BrightWhite => String::from(BG_BRIGHT_WHITE)
    };
}

pub fn draw(map: Vec<Vec<canvas::TextPixel>>) {
    for row in map.iter() {
        for col in row.iter() {
            match col.text_color {
                canvas::Color::Normal => print!("{}", col.text),
                _ => {
                    let color = get_term_color(&col.text_color);
                    print!("{}{}", color, col.text);
                }
            }
        }
        print!("\n");
    }
}