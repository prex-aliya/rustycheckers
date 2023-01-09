const BLOCK: &str    = "██";
const RED: &str     = "\x1b[0;31m";
const BLACK: &str   = "\x1b[0;0m";
const RED_PLAYER: &str     = "\x1b[0;30m";
const BLACK_PLAYER: &str   = "\x1b[0;33m";



/* play {{{ */
/*
 * TODO: make this a enum or like
fn _reset_peices() {
    for y in 0..8 {
        for x in 0..8 {
            if y < 3 && (0 == x%2 && 0 == y%2 || 1 == x%2 && 1 == y%2) {
                red_state[y][x] = true;
            }
            if y > 4 && (0 == x%2 && 0 == y%2 || 1 == x%2 && 1 == y%2) {
                black_state[y][x] = true;
            }
        }
    }
} */
fn print_board(red_state: &mut [[bool; 8]; 8], black_state: &mut [[bool; 8]; 8]) {
    /* For Testing */

    // TODO: Pieces only on diagnals need
    //      to fix that to big arrays.

    for y in 0..8*3 {
        for x in 0..8*3 {
            /* To Not Recompute these answers*/
            let ymod6 = y%6;
            let xmod6 = x%6;
            let ymod3 = y%3;
            let xmod3 = x%3;
            let ydiv3 = y/3;
            let xdiv3 = x/3;

            if 1==(xmod3) && 1==(ymod3) &&
                    black_state[ydiv3][xdiv3] == true {
                print!("{}{}", BLACK_PLAYER, BLOCK);
            } else if 1==(xmod3) && 1==(ymod3) &&
                    red_state[ydiv3][xdiv3] == true {
                print!("{}{}", RED_PLAYER, BLOCK);
            }

            else if 3>(xmod6) && 3>(ymod6) || 2<(xmod6) && 2<(ymod6) {
                print!("{}{}", BLACK, BLOCK);
            } else {
                print!("{}{}", RED, BLOCK);
            }
        }
        print!("\n");
    }
}
/* }}} */



/* vvv */



fn play() {
    let mut red_state: [[bool; 8]; 8] = [[false; 8]; 8];
    let mut black_state: [[bool; 8]; 8] = [[false; 8]; 8];
    'main: loop {
        print_board(&mut red_state, &mut black_state);
        break 'main;
    }
}

fn main() {
    play();
}
