use std::{io, io::Write};



const BLOCK: &str           = "██";
const RED: &str             = "\x1b[0;31m";
const BLACK: &str           = "\x1b[0;0m";
const RED_PLAYER: &str      = "\x1b[0;33m";
const BLACK_PLAYER: &str    = "\x1b[0;30m";
const RESET: &str           = "\x1b[0;0m";
const PROMPT:   &str        = " >>> ";


struct Pieces {
    red:        [[bool; 8]; 8],
    redcount:   i16,
    black:      [[bool; 8]; 8],
    blackcount: i16,
}



/* Misc {{{ */
pub fn get_input(turn: bool) -> String {
    if turn == false {
        print!("\n\tRED");
    } else {
        print!("\n\tBLACK");
    }

    print!("\x1b[0K");
    print!("{}", PROMPT);
    io::stdout().flush().expect("get_input failed to get users input");

    let mut output: String = String::new();
    io::stdin().read_line(&mut output)
        .expect("get_input failed to get users input");

    output.trim().to_string()
}
fn number_input(string: &String) -> Vec<usize> {
    let mut secondary: Vec<usize> = Vec::new();
    let command = (string.trim().split_whitespace()).collect::<Vec<&str>>();

    if command.len() >= 1 {
        for x in command.iter() {
            for c in x.chars() {
                if c.is_numeric() {
                    let num: usize = c.to_string().parse().unwrap();
                    secondary.push(num-1); /* To Line up With Board */
                    break;
                }
            }
        }
    }

    secondary
}
/* }}} */
/* play {{{ */
fn parse_input(input: &String, places: &mut [[bool;8];8], count: &mut i16) {
    let command = number_input(input);

    if command.len() == 4 {
        let mut num: i16 = 0;
        for x in 0..places.len() {
            for y in 0..places[x].len() {
                if places[x][y] == true {
                    num += 1;
                }
            }
        }
        if count != &mut num { return; }


        places[command[0] as usize][command[1] as usize] = false;
        places[command[2] as usize][command[3] as usize] = true;

        return;
    }
}
fn reset_peices(state: &mut Pieces) {
    for y in 0..8 {
        for x in 0..8 {
            if y < 3 && (0 == x%2 && 0 == y%2 || 1 == x%2 && 1 == y%2) {
                state.red[y][x] = true;
            }
            if y > 4 && (0 == x%2 && 0 == y%2 || 1 == x%2 && 1 == y%2) {
                state.black[y][x] = true;
            }
        }
    }
}
fn board_print_alphabet(color: &str) {
    let mut alphabet: u8 = 97;
    print!("\n\t{}", color);
    for _ in 0..8 {
        print!("{1:3}{0:3}", alphabet as char, " ");
        alphabet += 1;
    }
    print!("{}\n", RESET);
}
fn board_print_numbers(y :usize, num: i32, color: &str) {
    if 1 == y%3 {
        print!("{}{:4}{}\t", color, num+1, RESET);
    } else {
        print!("\t");
    }
}
fn print_board(state: &Pieces) {

    board_print_alphabet(RED);

    for y in 0..8*3 {
        /* To Not Recompute Y More Than Needed */
        let ymod6 = y%6;
        let ydiv3 = y/3;
        let ymod3 = y%3;

        print!("\n");
        board_print_numbers(y, ydiv3 as i32, BLACK);

        for x in 0..8*3 {
            /* To Not Recompute these answers*/
            let xmod6 = x%6;
            let xmod3 = x%3;
            let xdiv3 = x/3;

            /* TODO: this can be optimized v */
            if 1==(xmod3) && 1==(ymod3) &&
                    state.black[ydiv3][xdiv3] == true {
                print!("{}{}", BLACK_PLAYER, BLOCK);
            } else if 1==(xmod3) && 1==(ymod3) &&
                    state.red[ydiv3][xdiv3] == true {
                print!("{}{}", RED_PLAYER, BLOCK);
            }

            /* The Board Itself */
            else if 3>(xmod6) && 3>(ymod6) || 2<(xmod6) && 2<(ymod6) {
                print!("{}{}", BLACK, BLOCK); } else {
                print!("{}{}", RED, BLOCK);
            }
        }

        board_print_numbers(y, (((y as f32)*0.33 - 8.0) as i32)*-1, RED);
    }

    println!();
    board_print_alphabet(BLACK);
}
/* }}} */



/* vvv */



fn play() {
    let mut places: Pieces = Pieces { red: ([[false;8];8]), redcount: 12, black: ([[false;8];8]), blackcount: 12 };
    let mut black_turn: bool = false;

    reset_peices(&mut places) ;


    let mut input: Vec<String> = Default::default();
    let history: i8 = 4; /* Changable in real time */

    for _ in 0..history {
        input.push(" ".to_string());
    }

    'main: loop {
        /* TODO: ability to turn off any print to feed into
         *  another gui frontend */
        print_board(&places);

        let user_input = get_input(black_turn);

        if user_input == "quit".to_string() {
            break 'main /* TODO: Add quit function */
        } else if user_input == "kill".to_string() {

            if black_turn == false {
                let command = number_input(&user_input);
                if command.len() == 2 {
                    
                }
            }

        } else {
            if black_turn == false {
                parse_input(&user_input, &mut places.red, &mut places.redcount);
                black_turn = true;
            } else if black_turn {
                parse_input(&user_input, &mut places.black, &mut places.blackcount);
                black_turn = false;
            }
        }

        print!("\x1b[{}A", ((8*3)+7));//+history);

        // TODO: get this to work properly
        // TODO: move this to function
        //      Should This Just Not Exist
        /* command history */
        //input.push(user_input);
        //for y in (0..=history as usize).rev() {
        //    if y < input.len() { println!("{} \x1b[0K", input[y]);
        //    } else { println!(); }
        //}
        //if input.len() > history as usize { input.remove(0); }
    }
}

fn main() {
    play();
}

#[cfg(test)]
mod test;
// vim: tw=64
