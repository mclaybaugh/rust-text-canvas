/* ANSI color escape codes 
 * 
 * Good resource at: http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
 *
*/

use canvas;

fn get_term_color(color: &canvas::Color) -> &str {
    return match color {
        canvas::Color::Normal => "\x1B[0m",
        canvas::Color::Black => "\x1B[30m",
        canvas::Color::Red => "\x1B[31m",
        canvas::Color::Green => "\x1B[32m",
        canvas::Color::Yellow => "\x1B[33m",
        canvas::Color::Blue => "\x1B[34m",
        canvas::Color::Magenta => "\x1B[35m",
        canvas::Color::Cyan => "\x1B[36m",
        canvas::Color::White => "\x1B[37m",
        canvas::Color::BrightBlack => "\x1B[30;1m",
        canvas::Color::BrightRed => "\x1B[31;1m",
        canvas::Color::BrightGreen => "\x1B[32;1m",
        canvas::Color::BrightYellow => "\x1B[33;1m",
        canvas::Color::BrightBlue => "\x1B[34;1m",
        canvas::Color::BrightMagenta => "\x1B[35;1m",
        canvas::Color::BrightCyan => "\x1B[36;1m",
        canvas::Color::BrightWhite => "\x1B[37;1m"
    };
}

fn get_bg_term_color(color: &canvas::Color) -> &str {
    return match color {
        canvas::Color::Normal => "\x1B[0m",
        canvas::Color::Black => "\x1B[40m",
        canvas::Color::Red => "\x1B[41m",
        canvas::Color::Green => "\x1B[42m",
        canvas::Color::Yellow => "\x1B[43m",
        canvas::Color::Blue => "\x1B[44m",
        canvas::Color::Magenta => "\x1B[45m",
        canvas::Color::Cyan => "\x1B[46m",
        canvas::Color::White => "\x1B[47m",
        canvas::Color::BrightBlack => "\x1B[40;1m",
        canvas::Color::BrightRed => "\x1B[41;1m",
        canvas::Color::BrightGreen => "\x1B[42;1m",
        canvas::Color::BrightYellow => "\x1B[43;1m",
        canvas::Color::BrightBlue => "\x1B[44;1m",
        canvas::Color::BrightMagenta => "\x1B[45;1m",
        canvas::Color::BrightCyan => "\x1B[46;1m",
        canvas::Color::BrightWhite => "\x1B[47;1m"
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