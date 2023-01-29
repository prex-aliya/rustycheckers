#[cfg(test)]
mod test;

use std::process;
use ncurses::*;


fn error(msg: &str) {
    endwin();
    eprintln!("ERROR: {}", msg);
    process::exit(1);
}

/* Macro {{{ */
macro_rules! print_board {
    ($y:expr, $x:expr, $a:expr, $b:expr, $white:expr, $black:expr) => {
        let mut one = $a;
        let mut two = $b;

        for x in 0..8 {
            mv($y, $x);
            print_board!(one, two);
            $y+=1;

            mv($y, $x);
            print_board!(one, two, x, $white, $black);
            $y+=1;

            mv($y, $x);
            print_board!(one, two);
            $y+=1;

            if one == $a {
                one = $b;
                two = $a;
            } else {
                one = $a;
                two = $b;
            }
        }

        attroff(COLOR_PAIR($a));
        attroff(COLOR_PAIR($b));
    };
    ($a:expr, $b:expr, $x:expr, $white:expr, $black:expr) => {
        attroff(COLOR_PAIR($a));
        attroff(COLOR_PAIR($b));
        addch('\t' as u32);

        for y in 0..8 {
            if y % 2 == 0 {
                attron(COLOR_PAIR($a));
            } else {
                attron(COLOR_PAIR($b));
            }

            if $black[y][$x] {
                addstr("  B  ");
            } else if $white[y][$x] {
                addstr("  W  ");
            } else {
                addstr("     ");
            }
        }
    };
    ($a:expr, $b:expr) => {
        attroff(COLOR_PAIR($a));
        attroff(COLOR_PAIR($b));
        addch('\t' as u32);

        for _ in 0..4 {
            attron(COLOR_PAIR($a));
            addstr("     ");

            attron(COLOR_PAIR($b));
            addstr("     ");
        }
    }
}
/* }}} */

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
    col: i32,
    white: [[bool; 8]; 8],
    black: [[bool; 8]; 8]
}

impl Ui {
    fn begin(&mut self, x:i32, y:i32) {
        self.row = y;
        self.col = x;

        mv(y, x);
    }
    fn notifications(&mut self) {
        self.row += 3;
    }
    fn print_board(&mut self) {
        print_board!(self.row, self.col, 1, 2, self.white, self.black);
    }
    fn status(&mut self, _turn: Turn) {
    }

    fn end(&mut self) {

    }

    fn reset(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                if (((y % 2 == 0) & (x % 2 == 0)) || ((y % 2 == 1) & (x % 2 == 1))) & (x < 3) {
                    self.black[y][x] = true;
                }
            }
        }
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

    ui.black[1][4] = true;
    ui.white[5][7] = true;

    ui.reset();

    let mut quit = false;
    while !quit {
        ui.begin(0,0);
        {
            ui.notifications();

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
            'r' => ui.reset(),
            _ => {
                if turn == Turn::White {
                    match key as u8 as char {
                        _ => {},
                    }
                } else if turn == Turn::Black {
                    match key as u8 as char {
                        _ => {},
                    }
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
