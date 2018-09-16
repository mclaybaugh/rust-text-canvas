/* ANSI color escape codes 
 * 
 * Good resource at: http://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
 *
*/

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