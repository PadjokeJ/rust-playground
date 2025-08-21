use clap::Parser;

use crate::to_bin::run;

mod fen;
mod to_bin;
mod commands;
mod ID;

#[derive(Parser)]
struct ChessProgram {
    file_name: String,
}

fn main() {
    let args = ChessProgram::parse();

    run(args.file_name);
    
    println!();
}
