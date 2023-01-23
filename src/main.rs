#[cfg(test)]
mod test;

use std::process;

use ncurses::*;

const BLACK_BOARD_PAIR: i16 = 1;
const WHITE_BOARD_PAIR: i16 = 1;

fn main() {
    if has_colors() == false {
        endwin();
        eprintln!("Terminal Doess not support color");
        process::exit(1);
    }

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(BLACK_BOARD_PAIR, COLOR_WHITE, COLOR_RED);
    init_pair(WHITE_BOARD_PAIR, COLOR_BLACK, COLOR_WHITE);


}

// vim: tw=64
