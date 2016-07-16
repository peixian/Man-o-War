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

fn v_match(pipe1: u8, pipe2:u8) -> u8 {
    return !(pipe1 >> 2 ^ pipe2) & 1;
}

fn mismatch() {
    //iterate through
}

struct Grid {
    cols: i32,
    rows: i32,
    data: Vec<u8>
}

impl Grid {
    fn new(cols: i32, rows: i32) -> Grid {
        Grid {
            cols: cols,
            rows: rows,
            data: vec![0.0; cols * rows]
        }
    }
}

fn main() {
    let mut grid_test: [[i32; 4]; 3] = [[1,2,3,4];[1,2,3,4];[1,2,3,4]];
    println!("grid_test: {}", grid_test);
    println!("grid at 0,0: {}", grid_test[0][0];
    (grid_test.0).0 = 4;
    println!("grid at 0,0: {}", grid_test[0][0];
    println!("{}", rot((LEFT+RIGHT)));
    println!("{}", h_match(LEFT, UP+DOWN));
}
