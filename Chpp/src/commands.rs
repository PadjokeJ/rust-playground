use std::collections::HashMap;

use crate::ID;

pub fn row_to_u64(row: Vec<i8>) -> u64 {
    let mut val: u64 = 0;
    let mut i = 56;
    for col in row {
        let add = ((col as u64) << i) as u64;
        val += add;
        i -= 8;
    }

    val
}

pub fn determine(pos: Vec<i8>, memory: &mut HashMap<u64, u64>) {
    let func_id = row_to_u64(pos[0..8].to_vec());
    //println!("func id : {}", func_id);
    match func_id {
        ID::WRITE_VAR => memorize(pos[8..].to_vec(), memory),
        ID::PRINT => print!("{}", remember(pos[8..].to_vec(), memory) as u8 as char),
        _ => ()
    }
}

pub fn memorize(board: Vec<i8>, memory: &mut HashMap<u64, u64>) {
    let var_name = row_to_u64(board[..8].to_vec());
    let var_value = num_value_from_row(board[8..29].to_vec());

    memory.insert(var_name, var_value) ;
}

pub fn remember(board: Vec<i8>, memory: &mut HashMap<u64, u64>) -> u64 {
    let var_name = row_to_u64(board[..8].to_vec());
    let var_value = memory[&var_name];

    var_value
}

pub fn num_value_from_row(row: Vec<i8>) -> u64 {
    let mut val = 0;
    let mut i = 0;
    for piece in row {
        let add = ((if piece > 8 { piece - 4 } else { piece }) & 7) as u64;
        val += add << i;
        i += 3;
    }

    //println!("{:?}", val);
    val
}