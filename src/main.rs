const BLOCK: &str    = "██";
const RED: &str     = "\x1b[0;31m";
const BLACK: &str   = "\x1b[0;0m";



/* play {{{ */
fn print_board(red_state: &mut [[bool; 8]; 8], black_state: &mut [[bool; 8]; 8]) {

    for y in 0..8 {
        for x in 0..8 {
            if y < 2 {
                red_state[y][x] = true;
            }
            if y > 5 {
                black_state[y][x] = true;
            }
        }
    }

    for y in 0..8*3 {
        for x in 0..8*3 {
            /* To Not Recompute these answers*/
            let ymod3 = y%3;
            let xmod3 = x%3;
            let ydiv3 = y/3;
            let xdiv3 = x/3;

            if 1==(xmod3) && 1==(ymod3) &&
                    red_state[ydiv3][xdiv3] == true {
                print!("{}{}", "\x1b[0;30m", BLOCK);
            } else if 1==(xmod3) && 1==(ymod3) &&
                    black_state[ydiv3][xdiv3] == true {
                print!("{}{}", "\x1b[0;30m", BLOCK);
            } else if 3>(x%6) && 3>(y%6) || 2<(x%6) && 2<(y%6) {
                print!("{}{}", RED, BLOCK);
            } else {
                print!("{}{}", BLACK, BLOCK);
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
    print_board(&mut red_state, &mut black_state);
}

fn main() {
    play();
}
