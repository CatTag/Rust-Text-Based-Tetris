use std::str::ParseBoolError;
use std::time::Duration;
use std::thread;
use rand::Rng;
use std::io;
fn main() {
    let mut board = ['.'; 100];
    let mut block = [0; 4];
    let mut rng = rand::thread_rng();
    
    loop {
        let num = rng.gen_range(1..=7);
        match num {
            1 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 2+3; 
                block[3] = 3+3;
            }
            2 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 2+3;
                block[3] = 12+3;
            }
            3 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 2+3;
                block[3] = 10+3;
            }
            4 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 2+3;
                block[3] = 11+3;
            }
            5 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 10+3;
                block[3] = 11+3;
            }
            6 => {
                block[0] = 0+3;
                block[1] = 1+3;
                block[2] = 11+3;
                block[3] = 12+3;
            }
            7 => {
                block[0] = 1+3;
                block[1] = 2+3;
                block[2] = 10+3;
                block[3] = 11+3;
            }
            _ => {
                eprint!("something broke :(");
            }
        }
        board = tick(board, block, num);
    }
}

fn tick(board: [char; 100], block: [usize; 4], block_type: usize) -> [char; 100] {
    let mut board_mut = board.clone();
    let mut move_block = true;
    let mut block = block;
    let mut input = String::new();
    let mut move_block_side = true;
    let mut direction = 0;

    loop {
        move_block = true;
        move_block_side = true;
        print_board(board_mut);

        if input != "s" {
            input = get_input();
        }
        if input == "a" {
            if move_left_check(board, block, block_type) {
                for i in block {
                    board_mut[i] = '.';
                    board_mut[i-1] = '@';
                }
                for i in 0..4 {
                    block[i] -= 1;
                }
            }
        }
        if input == "d" {
            if move_right_check(board, block, block_type) {
                for i in block.into_iter().rev() {
                    board_mut[i] = '.';
                    board_mut[i+1] = '@';
                }
                for i in 0..4 {
                    block[i] += 1;
                }
            }
        }

        // if input == "w" {
        //     if check_rotation(board, block, block_type) {
        //         if direction != 3 {
        //             direction += 1;
        //         } else {
        //             direction = 0;
        //         }
        //         match block_type {
        //             1 => {
        //                 if direction == 0 {
        //                     block[0] -= 9;
        //                     block[2] += 11;
        //                     block[3] -= 22;
        //                 }
        //                 if direction == 1 {
        //                     block[0] -= 9;
        //                     block[2] += 9;
        //                     block[3] += 19;
        //                 }
        //                 if direction == 2 {
        //                     block[0] += 11;
        //                     block[2] -= 9;
        //                     block[3] -= 19;
        //                 }
        //                 if direction == 3 {
        //                     block[0] += 9;
        //                     block[2] -= 9;
        //                     block[3] -= 18;
        //                 }
        //             }
        //             2 => {
        //                 if direction == 0 {
        //                     block[0] -= 11;
        //                     block[2] += 11;
        //                     block[3] += 21;
        //                 }
        //                 if direction == 1 {
        //                     block[0] -= 9;
        //                     block[2] += 9;
        //                     block[3] -= 2;
        //                 }
        //                 if direction == 2 {
        //                     block[0] += 11;
        //                     block[2] -= 11;
        //                     block[3] -= 20;
        //                 }
        //                 if direction == 3 {
        //                     block[0] += 9;
        //                     block[2] -= 9;
        //                     block[3] += 2;
        //                 }
        //             }
        //             3 => {
        //                 block[0] = 0;
        //                 block[1] = 1;
        //                 block[2] = 2;
        //                 block[3] = 10;
        //             }
        //             4 => {
        //                 block[0] = 0;
        //                 block[1] = 1;
        //                 block[2] = 2;
        //                 block[3] = 11;
        //             }
        //             5 => {
        //                 block[0] = 0;
        //                 block[1] = 1;
        //                 block[2] = 10;
        //                 block[3] = 11;
        //             }
        //             6 => {
        //                 block[0] = 0;
        //                 block[1] = 1;
        //                 block[2] = 11;
        //                 block[3] = 12;
        //             }
        //             7 => {
        //                 block[0] = 1;
        //                 block[1] = 2;
        //                 block[2] = 10;
        //                 block[3] = 11;
        //             }
        //             _ => {eprint!("something broke :(");}
        //         }
        //     }
        // }
        

        for i in block.into_iter().rev() {
            if i+10 <= 99 {
                
                if board_mut[i+10] == '@' {
                    if !block.contains(&(i+10)) {
                        move_block = false;
                    }
                }

            } else {
                return board_mut;
            }
        }

        if move_block == false {
            return board_mut;
        } else {
            for i in block.into_iter().rev() {
                if i+10 <= 99 {
                    board_mut[i] = '.';
                    board_mut[i+10] = '@';
                } else {
                    return board_mut;
                }
            }
            for i in 0..4 {
                block[i] += 10;
            }
        }
        
        

    }
}

fn print_board(board: [char; 100]) {
    let mut i = 0;
    println!("\n\n\n\n");
    for c in board {
        if (i+1) % 10 == 0 {
            println!("{c}");
        } else {
            print!("{c}");
        }
        i += 1;
    }
}

fn get_input() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    return input.trim().to_string();
}

fn move_left_check(board: [char; 100], block: [usize; 4], block_type: usize) -> bool {
    for i in board {
        if i % 10 == 0 {
            return false;
        }
    }
    
}

fn move_right_check(board: [char; 100], block: [usize; 4], block_type: usize) -> bool {
    return false
}

// fn check_rotation(board: [char; 100], block: [usize; 4], block_type: usize) -> bool {
//     return false
// }