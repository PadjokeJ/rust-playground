use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

use crate::commands::determine;
use crate::fen;

pub fn open_file(name: String) -> Lines<BufReader<File>> {
    println!("Opening : {}", name);
    let file = File::open(name).unwrap();

    BufReader::new(file).lines()
}

pub fn run(name: String) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let lines = open_file(name);

    for line in lines {
        let chess_pos = fen::translate_fen(line.unwrap());
        determine(chess_pos, &mut memory);
    }
}