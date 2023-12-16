use rand::Rng;
use std::io;
fn main() {
    let mut board = ['.'; 100];
    let mut block = [0; 4];
    let mut rng = rand::thread_rng();

    loop {
        let num = rng.gen_range(1..=7);
        let start = rng.gen_range(0..=6);
        match num {
            1 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 2+start; 
                block[3] = 3+start;
            }
            2 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 2+start;
                block[3] = 12+start;
            }
            3 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 2+start;
                block[3] = 10+start;
            }
            4 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 2+start;
                block[3] = 11+start;
            }
            5 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 10+start;
                block[3] = 11+start;
            }
            6 => {
                block[0] = 0+start;
                block[1] = 1+start;
                block[2] = 11+start;
                block[3] = 12+start;
            }
            7 => {
                block[0] = 1+start;
                block[1] = 2+start;
                block[2] = 10+start;
                block[3] = 11+start;
            }
            _ => {
                eprint!("something broke :(");
            }
        }
        board = tick(board, block);
    }
}

fn tick(board: [char; 100], block: [usize; 4]) -> [char; 100] {
    let mut board_mut = board.clone();
    let mut move_block: bool;
    let mut block = block;
    let mut input: String;

    loop {
        move_block = true;
        print_board(board_mut);

        input = get_input();
        
        
        match input.as_str() {
            "a" => {
                if move_left_check(board, block) {
                    for i in block {
                        board_mut[i] = '.';
                        board_mut[i-1] = '@';
                    }
                    for i in 0..4 {
                        block[i] -= 1;
                    }
                }
            }
            "d" => {
                if move_right_check(board, block) {
                    for i in block.into_iter().rev() {
                        board_mut[i] = '.';
                        board_mut[i+1] = '@';
                    }
                    for i in 0..4 {
                        block[i] += 1;
                    }
                }
            }
        
            "s" | "" => {
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
        _ => {}
        
        }

    }
}

fn print_board(board: [char; 100]) {
    let mut i = 0;
    print!("{}[2J", 27 as char);
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

fn move_left_check(board: [char; 100], block: [usize; 4]) -> bool {
    let block_mut: [usize; 4] = block.clone();
    let board_mut: [char; 100] = board.clone();
    for i in block_mut {
        if i % 10 == 0 {
            return false;
        }
    }
    for i in block_mut {
        if !block_mut.contains(&(i-1)) {
            if board_mut[i-1] == '@' {
                return false;
            }
        }
    }
    return true;
    
}

fn move_right_check(board: [char; 100], block: [usize; 4]) -> bool {
    let block_mut: [usize; 4] = block.clone();
    let board_mut: [char; 100] = board.clone();
    for i in block_mut {
        if i % 10 == 9 {
            return false;
        }
    }
    for i in block_mut {
        if !block_mut.contains(&(i+1)) {
            if board_mut[i+1] == '@' {
                return false;
            }
        }
    }
    return true;
}

// fn check_rotation(board: [char; 100], block: [usize; 4], block_type: usize) -> bool {
//     return false;
// }