const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;
const ALL: u8 = UP + RIGHT + DOWN + LEFT;


fn rot(pipe: u8) -> u8{
    return (pipe >> 3 | pipe << 1) & 15; 
}

fn h_match(pipe1: u8, pipe2: u8) -> u8 {
    return !(pipe1 ^ pipe2 >> 2) & 2;
}

fn main() {
    println!("{}", rot((LEFT+RIGHT)));
    println!("{}", h_match(LEFT, UP+DOWN));
}
