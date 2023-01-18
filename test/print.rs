const BLOCK: &str   = "█";
const BLOCKS: &str   = "██";
const RED: &str             = "\x1b[0;31m";
const BLACK: &str           = "\x1b[0;37m";
const BARRIER: &str         = "\x1b[0;30m";


fn print_board_slice(input: &str, is_red: bool) {
    if is_red {
        print!("{}{}", RED, input);
    } else {
        print!("{}{}", BLACK, input);
    }
}
fn print_board_peice(input: &str, is_red: bool) {
    if is_red {
        print!("{}{}", "\x1b[0;37;41m", input);
    } else {
        print!("{}{}", "\x1b[0;30;47m", input);
    }
}
fn main() {
    let mut is_red: bool = true;
    for y in 0..8*3 {

        if y%3 == 0 { is_red ^= true; }
        for x in 0..8 {
            for z in 0..3 {

                if z == 1 {
                    if 1 == y%3 {
                        print_board_peice("", is_red)
                    } else {
                        print_board_slice(BLOCK, is_red);
                    }
                } else {
                    print_board_slice(BLOCKS, is_red);
                }

            }
            print!("{}{}", BARRIER, BLOCK);

            is_red ^= true;

        }
        print!("\n");
    }
}
/*
   A A A A
   A A A A
   A A A A
   A A A A

   AAAAA 
   AABAA
   AAAAA



 * */
