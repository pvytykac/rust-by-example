fn main() {
    let ints: [i32; 3] = [1, 2, 3];
    println!("{:?}", ints);

    // iteration
    for el in ints.iter() {
        print!("{}, ", el);
    }
    println!();

    for (ix, el) in ints.iter().enumerate() {
        print!("{} => {}, ", ix, el);
    }
    println!();

    for ix in 0..ints.len() {
        print!("{} => {}, ", ix, ints[ix]);
    }
    println!();

    // tic tac toe
    let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
    println!("{:?}", board);

    board[1][1] = 'x';
    board[1][0] = 'o';
    println!("{:?}", board);
    println!("Length: {}", board.len());

    // slicing
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", &arr[..]);
    println!("{:?}", &arr[..5]);
    println!("{:?}", &arr[5..]);
    println!("{:?}", &arr[3..8]);
}
