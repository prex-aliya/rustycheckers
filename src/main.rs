#[cfg(test)]
mod test;

use std::process;

use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

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
    row: i32,
    col: i32
}

impl Ui {
    fn begin(&mut self, x:i32, y:i32) {
        self.row = y;
        self.col = x;

        mv(y, x);
    }
    fn notifications(&mut self) {
    }
    fn print_board(&mut self) {
        self.row = 3;
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
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE)           ;


    start_color();
    if has_colors() == false {
        endwin();
        error("Terminal Doess not support color");
    }

    /*          pair            forground   background */
    init_pair(1, COLOR_WHITE, COLOR_RED);
    init_pair(2, COLOR_BLACK, COLOR_WHITE);



    let turn: Turn = Turn::White;
    let mut ui = Ui::default();

    ui.print_board();

    let mut quit = false;
    while !quit {

        let mut row: i32 = 1;

        for _ in 0..4 {
            for _ in 0..3 {
                mv(row, 0);
                for _ in 0..4 {
                    attron(COLOR_PAIR(1));
                    addstr("     ");

                    attron(COLOR_PAIR(2));
                    addstr("     ");
                }
                row += 1;
            }
            for _ in 0..3 {
                mv(row, 0);
                for _ in 0..4 {
                    attron(COLOR_PAIR(2));
                    addstr("     ");

                    attron(COLOR_PAIR(1));
                    addstr("     ");
                }
                row += 1;
            }
        }


        ui.begin(0,0);
        {
            ui.notifications();

            /* chess board */

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
