#[cfg(test)]
mod test;

use ncurses::*;
use std::{io, io::Write};

/* Variables {{{ */
const BLOCK: &str = "█";
const BLOCKS: &str = "██";

const RED: &str = "\x1b[0;31;41m";
const BLACK: &str = "\x1b[0;37;47m";
const RED_PLAYER: &str = "\x1b[0;37;41m";
const BLACK_PLAYER: &str = "\x1b[0;31;47m";
const BARRIER: &str = "";

const RESET: &str = "\x1b[0;0m";

const PROMPT: &str = " >>> ";
const MAXPEICES: i16 = 12;

struct Pieces {
    red: [[bool; 8]; 8],
    redcount: i16,
    black: [[bool; 8]; 8],
    blackcount: i16,
}
/* }}} */


/* misc {{{ */
pub fn get_input(turn: bool) -> String {
    if turn == false {
        print!("\n\tRED");
    } else {
        print!("\n\tBLACK");
    }

    print!("\x1b[0K");
    print!("{}", PROMPT);
    io::stdout()
        .flush()
        .expect("get_input failed to get users input");

    let mut output: String = String::new();
    io::stdin()
        .read_line(&mut output)
        .expect("get_input failed to get users input");

    output.trim().to_string()
}
fn number_input(string: &String) -> Vec<usize> {
    let mut secondary: Vec<usize> = Vec::new();

    if string.len() >= 1 {
            for c in string.chars() {
                if c.is_numeric() {
                    let num: usize = c.to_string().parse().unwrap();
                    secondary.push(8-num); /* To flip the value arround to fit 
                        with the orintation of the board since the board numbers are reversed*/
                    //print!("{}", num);
                } else if 96 < (c as usize) && (c as usize) < 97+8{
                    let num: usize = (c as usize)-97;
                    secondary.push(num);
                    //print!("{}", num);
                }
            }
    }

    secondary
}
/* }}} */
/* play {{{ */
fn parse_input(input: &String, places: &mut [[bool; 8]; 8], count: &mut i16) -> bool {
    let command = number_input(input);

    if command.len() == 4 {
        let mut num: i16 = 0;
        // TODO: put v into a misc function
        for x in 0..places.len() {
            for y in 0..places[x].len() {
                if places[x][y] == true {
                    num += 1;
                }
            }
        }
        if count != &mut num {
            println!();
            return false;
        }

        places[command[0] as usize][command[1] as usize] = false;
        places[command[2] as usize][command[3] as usize] = true;

        return true;
    } else {
        println!("opps! Incorrect vale");
        return false;
    }
}
fn reset_peices(state: &mut Pieces) {
    for y in 0..8 {
        for x in 0..8 {
            if y < 3 && (0 == x % 2 && 0 == y % 2 || 1 == x % 2 && 1 == y % 2) {
                state.black[y][x] = true;
            } else {
                state.black[y][x] = false;
            }

            if y > 4 && (0 == x % 2 && 0 == y % 2 || 1 == x % 2 && 1 == y % 2) {
                state.red[y][x] = true;
            } else {
                state.red[y][x] = false;
            }
        }
    }
}
/* }}} */
/* fancy print board {{{ */
fn fancy_board_slice(input: &str, is_red: bool) {
    fancy_board_individual(input, is_red, RED, BLACK);
}
fn fancy_board_peice(input: &str, is_red: bool) {
    fancy_board_individual(input, is_red, RED_PLAYER, BLACK_PLAYER);
}
fn fancy_board_individual(input: &str, is_red: bool, red: &str, black: &str) {
    if is_red {
        print!("{}{}", red, input);
    } else {
        print!("{}{}", black, input);
    }
}
fn fancy_print_board(state: &Pieces) {
    let mut is_red: bool = true;

    print!("{:6}", " ");
    for x in 97..97+8 {
        print!(" {:2}{:2}", std::char::from_u32(x).unwrap(), " ");
    }
    println!("\n");

    for y in 0..8 * 3 {
        let ymod3 = y % 3;
        let ydiv3 = y / 3;

        if ymod3 == 1 {
            print!("{:3}{:2}", 8-(ydiv3), " ");
        } else {
            print!("{:5}", " ");
        }

        if y % 3 == 0 {
            is_red ^= true;
        }
        for x in 0..8 {
            for z in 0..3 {
                if z == 1 {
                    if 1 == y % 3 {
                        /*  */
                        if state.black[ydiv3][x] == true {
                            fancy_board_peice("", is_red)
                        } else if state.red[ydiv3][x] == true {
                            fancy_board_peice("", is_red)
                        } else {
                            fancy_board_slice(BLOCK, is_red)
                        }
                    } else {
                        fancy_board_slice(BLOCK, is_red);
                    }
                } else {
                    /* Surrounding red and white blocks */
                    fancy_board_slice(BLOCKS, is_red);
                }
            }
            is_red ^= true;
            print!("{1}{0}{1}", BARRIER, RESET);
        }
        print!("\n{}", RESET);
    }
}
/* }}} */
/* Loop {{{ */
fn the_loop(places: &mut Pieces, fancy: bool) {
    let mut black_turn: bool = false;

    'main: loop {
        let mut extend: i8 = 0;
        /* TODO: ability to turn off any print to feed into
         *  another gui frontend */
        if fancy {
            fancy_print_board(&places);
        } else {
            return;
        }

        let user_input = get_input(black_turn);

        /* TODO: put these into functions */
        if user_input == "quit".to_string() {
            break 'main; /* TODO: Add quit function */
        } else if user_input == "clear".to_string() {
            reset_peices(places);
        } else if user_input.contains("kill") {
            if black_turn == false {
                let command = number_input(&user_input);
                if command.len() == 2 {
                    if places.red[command[0]][command[1]] == true {
                        places.red[command[0]][command[1]] = false;
                        places.redcount -= 1;
                    }
                }
            } else if black_turn == true {
                let command = number_input(&user_input);
                if command.len() == 2 {
                    if places.black[command[0]][command[1]] == true {
                        places.black[command[0]][command[1]] = false;
                        places.blackcount -= 1;
                    }
                }
            }
        } else {
            /* TODO: this can/should be able to be optimized */
            match black_turn {
                false => {
                    /* Red */
                    match parse_input(&user_input, &mut places.red, &mut places.redcount) {
                        true => {
                            black_turn = true;
                        }
                        false => extend += 1,
                    }
                }
                true => {
                    /* Black */
                    match parse_input(&user_input, &mut places.black, &mut places.blackcount) {
                        true => {
                            black_turn = false;
                        }
                        false => extend += 1,
                    }
                }
            }
        }

        print!("\x1b[{}A", ((8 * 3) + 4) + extend);
        //print!("{:?}", places.red[2]);
    }
}
/* }}} */

/* vvv */

// TODO: split into functions
fn play(fancy: &mut bool) {
    let mut places: Pieces = Pieces {
        red: ([[false; 8]; 8]),
        redcount: MAXPEICES,
        black: ([[false; 8]; 8]),
        blackcount: MAXPEICES,
    };

    reset_peices(&mut places);

    the_loop(&mut places, *fancy);
}

fn main() {
    let mut fancy: bool = true;
    play(&mut fancy);
}

// vim: tw=64
