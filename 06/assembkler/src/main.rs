use std::{env, fs};
use std::fs::File;
use std::collections::HashMap;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::{Path, PathBuf};

static mut c_dict: HashMap<String, String> = HashMap::new();


static mut d_dict: HashMap<String, String> = HashMap::new();


static mut j_dict: HashMap<String, String> = HashMap::new();


fn init_dictionaries() {
    c_dict.insert("0", "101010".to_string());
    c_dict.insert("1", "111111");
    c_dict.insert("-1", "111010");
    c_dict.insert("D", "001100");
    c_dict.insert("A", "110000");
    c_dict.insert("!D", "001101");
    c_dict.insert("!A", "110001");
    c_dict.insert("-D", "001111");
    c_dict.insert("-A", "110011");
    c_dict.insert("D+1", "011111");
    c_dict.insert("A+1", "110111");
    c_dict.insert("D-1", "001110");
    c_dict.insert("A-1", "110010");
    c_dict.insert("D+A", "000010");
    c_dict.insert("D-A", "010011");
    c_dict.insert("A-D", "000111");
    c_dict.insert("D&A", "000000");
    c_dict.insert("D|A", "010101");
    j_dict.insert("JGT", "001");
    j_dict.insert("JEQ", "010");
    j_dict.insert("JGE", "011");
    j_dict.insert("JLT", "100");
    j_dict.insert("JNE", "101");
    j_dict.insert("JLE", "110");
    j_dict.insert("JMP", "111");
    j_dict.insert("null", "000");
    d_dict.insert("M", "001");
    d_dict.insert("D", "010");
    d_dict.insert("MD", "011");
    d_dict.insert("A", "100");
    d_dict.insert("AM", "101");
    d_dict.insert("AD", "110");
    d_dict.insert("AMD", "111");
    d_dict.insert("null", "000");
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let asm_path = Path::new(&args[1].clone());
    let mut hack_path = PathBuf::from(asm_path.clone());
    hack_path.set_extension("hack");
    let write_file = File::create(hack_path)?;

    init_dictionaries()

    let file = File::open(asm_path)?;
    let buffered = BufReader::new(file);

    write!(output, "Rust\n💖\nFun")?;
    write!(output, "Rust\n💖\nFun")?;
    // for line in buffered.lines() {
    //     // println!("{}", line?);

    // }

    // let current_dir = env::current_dir()?;
    Ok(())
}

fn num_parser() {

}