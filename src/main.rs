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

    let mut tab = Turn::White;

    let mut quit = false;
    while !quit {
        ui.begin(0,0);
        {
            ui.notifications();

            /* chess board */
            ui.print_board();

            /* statistics */
            match turn {
                Turn::White => ui.print_stats("White"),
                Turn::Black => ui.print_stats("Black"),
            }


        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'h' => ui.help(),
            _ => {
                if turn == Turn::While {

                } else if turn == Turn::Black {

                } else {
                    error("It Must Be Black Or Whites Turn");
                }
            }
        }
    }


    save_state();

    endwin();
}

// vim: tw=64
