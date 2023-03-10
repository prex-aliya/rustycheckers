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
/* print_board {{{ */
macro_rules! print_board {
    ($y:expr, $x:expr, $a:expr, $b:expr, $white:expr, $black:expr) => {
        let mut one = $a;
        let mut two = $b;
        let mut col = 1;

        mv($y, $x);
        addstr("  1  ");
        for x in 2..=8 {
            addstr(format!("  {}  ", (x+48 as u8) as char).as_str());
        }
        addch('\n' as u32);
        $y+=2;

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
        //addch('t' as u32);

        for y in 0..8 {
            if y % 2 == 0 {
                attron(COLOR_PAIR($a));
            } else {
                attron(COLOR_PAIR($b));
            }

            if $black[y][$x] {
                addstr("    ");
            } else if $white[y][$x] {
                addstr("    ");
            } else {
                addstr("     ");
            }
        }
    };
    ($a:expr, $b:expr) => {
        attroff(COLOR_PAIR($a));
        attroff(COLOR_PAIR($b));
        //addch('t' as u32);

        for _ in 0..4 {
            attron(COLOR_PAIR($a));
            addstr("     ");

            attron(COLOR_PAIR($b));
            addstr("     ");
        }
    }
}
/* }}} */
/* }}} */

/* turn {{{ */
#[derive(PartialEq)]
enum Turn {
    White,
    Black,
    Other
}

impl Turn {
    fn toggle(&self) -> Self {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
            _ => Turn::White,
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
    fn notifications(&mut self, turn: &mut Turn, notification: &str) {
        mv(self.row, self.col);

        match turn {
            Turn::White => addstr("White Turn"),
            Turn::Black => addstr("Black Turn"),
            Turn::Other => addstr("Setting")
        };

        mvprintw(self.row+1, self.col, notification);

        self.row += 3;
    }
    fn print_board(&mut self) {
        print_board!(self.row, self.col, 1, 2, self.white, self.black);
    }
    fn move_peice(&mut self, initkey: i32, turn: &mut Turn) {
        let x = self.col+10+((initkey-49)*5);
        let y = self.row-(3*9)+1;

        attron(COLOR_PAIR(3));
        mv(y, x-8);
        addch(initkey as u32);

        let key = getch();
        if (key as u8 as char).is_numeric() {
            let first: usize    = (initkey-49) as usize;
            let second: usize   = (key-49) as usize;

            if turn == &mut Turn::White {
                if self.white[first][second]  {
                    self.white[first][second] = false;
                } else {
                    self.white[first][second] = true;
                    turn.toggle();
                }
            } else {
                if self.black[first][second] {
                    self.black[first][second] = false;
                } else {
                    self.black[first][second] = true;
                    turn.toggle();
                }
            }
        }
                
        attroff(COLOR_PAIR(3));
        clear();
    }

    fn settings(&self) {}

    fn status(&mut self, _turn: Turn) {}
    fn end(&mut self) {}

    fn reset(&mut self) {
        for b in (0..8).step_by(2) {
            for i in 0..8 { self.white[b][i] = false; self.black[b][i] = false; }
        }

        for y in 0..8 {
            for x in 0..8 {
                if (((y % 2 == 0) & (x % 2 == 0)) || ((y % 2 == 1) & (x % 2 == 1))) & (x < 3) {
                    self.black[y][x] = true;
                } else if (((y % 2 == 0) & (x % 2 == 0)) || ((y % 2 == 1) & (x % 2 == 1))) & (x > 4) {
                    self.white[y][x] = true;
                }
            }
        }
    }
    fn save(&self) {}
    fn help(&self) {}
}
/* }}}*/


fn main() {




    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    if has_colors() == false {
        endwin();
        error("Terminal Doess not support color");
    }
    /*          pair            forground   background */
    init_pair(1, COLOR_BLACK, COLOR_WHITE);
    init_pair(2, COLOR_WHITE, COLOR_RED);
    init_pair(3, COLOR_BLACK, COLOR_WHITE);

    let mut x: i32 = 0;
    let mut y: i32 = 8;

    let mut turn: Turn = Turn::White;
    let mut ui = Ui::default();
    let mut notification: String = "".to_string();

    ui.reset(); /* resets the peices positions */

    let mut quit = false;
    while !quit {
        clear();
        ui.begin(y,x);
        {
            ui.notifications(&mut turn, &mut notification);
            notification = "".to_string();

            ui.print_board();
            /* statistics */
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'h' => ui.help(),
            'r' => {
                ui.reset();
                notification = "Peices Reset".to_string();
            }

            'J' => x-=1,
            'K' => x+=1,

            'H' => y-=1,
            'L' => y+=1,

            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => {
                ui.move_peice(key as u8 as i32, &mut turn);
            },
            '\t' => {
                ui.settings();
            }
            '\n' => {
                match turn {
                    Turn::White => turn = Turn::Black,
                    Turn::Black => turn = Turn::White,
                    _ => {},
                }
            },
            _ => {
                if turn != Turn::Other {
                    if turn == Turn::White {
                        match key as u8 as char {
                            _ => {},
                        }
                    } else if turn == Turn::Black {
                        match key as u8 as char {
                            _ => {},
                        }
                    }
                }
            }
        }
    }


    ui.save();

    endwin();
}

// vim: tw=64
