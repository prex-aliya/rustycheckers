#[cfg(test)]
mod test;

use std::process;
use ncurses::*;


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
        /* TODO: use macros to simplify
         *
         * third row checkes if there is peice in array then
         * prints the peice version if not prints normally
         *
         * maybe checkes entire row?
         */

        let mut black: [[bool; 8]; 8] = [[false; 8]; 8];
        black[1][1] = true;

        for x in 0..4 {
                mv(row, 3);
                for _ in 0..4 {
                    attron(COLOR_PAIR(1));
                    addstr("     ");

                    attron(COLOR_PAIR(2));
                    addstr("     ");
                }
                row += 1;
                mv(row, 3);
                let mut y: i8 = 0;
                while !(y > 8) {
                    if black[x as usize][y as usize] == true {
                        attron(COLOR_PAIR(1));
                        addstr("  A  ");
                    } else {
                        attron(COLOR_PAIR(1));
                        addstr("     ");
                    }
                    if y == 8 { break; }
                    y+=1;
                    if black[x as usize][y as usize] == true {
                        attron(COLOR_PAIR(2));
                        addstr("  A  ");
                    } else {
                        attron(COLOR_PAIR(2));
                        addstr("     ");
                    }
                }
                row += 1;
                mv(row, 3);
                for _ in 0..4 {
                    attron(COLOR_PAIR(1));
                    addstr("     ");

                    attron(COLOR_PAIR(2));
                    addstr("     ");
                }
                row += 1;
            for _ in 0..3 {
                mv(row, 3);
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
