#[cfg(test)]
mod test;

use std::process;

use ncurses::*;

const BLACK_BOARD_PAIR: i16 = 1;
const WHITE_BOARD_PAIR: i16 = 1;

fn error(msg: &str) {
    endwin();
    eprintln!("ERROR: {}", msg);
    process::exit(1);
}

/* turn {{{ */
#[derive(PartialEq)]
enum Turn {
    White,
    Black
}

impl Turn {
    fn toggle(&self) -> Self {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White
        }
    }
}
/* }}} */
/* ui {{{ */
#[derive(Default)]
struct Ui {
}

impl Ui {
    fn begin(&mut self, x:usize, y:usize) {

    }
    fn notifications(&mut self) {

    }
    fn print_board(&mut self) {

    }
    fn status(&mut self, turn: Turn) {

    }

    fn end(&mut self) {

    }

    fn save(&self) {

    }
    fn help(&self) {

    }
}
/* }}}*/

fn main() {
    if has_colors() == false {
        error("Terminal Doess not support color");
    }

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE)           ;

    start_color();
    /*          pair            forground   background */
    init_pair(BLACK_BOARD_PAIR, COLOR_WHITE, COLOR_RED);
    init_pair(WHITE_BOARD_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut turn: Turn = Turn::White;
    let mut ui = Ui::default();

    let mut quit = false;
    while !quit {
        ui.begin(0,0);
        {
            ui.notifications();

            /* chess board */
            ui.print_board();

            /* statistics */
            match turn {
                Turn::White => ui.status(Turn::White),
                Turn::Black => ui.status(Turn::Black),
            }


        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'h' => ui.help(),
            _ => {
                if turn == Turn::White {

                } else if turn == Turn::Black {

                } else {
                    error("It Must Be Black Or Whites Turn");
                }
            }
        }
    }


    ui.save();

    endwin();
}

// vim: tw=64
