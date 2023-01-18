use super::*;


/*https://stackoverflow.com/questions/38995892/how-to-move-tests-into-a-separate-file-for-binaries-in-rusts-cargo*/

#[test]
fn test() {
    let mut places: Pieces = Pieces { 
        red: ([[false;8];8]),
        redcount: 12,
        black: ([[false;8];8]),
        blackcount: 12};
    reset_peices(&mut places);

    fancy_print_board(&places);
}
