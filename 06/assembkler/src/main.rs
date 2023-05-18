use std::{env, fs};
use std::fs::File;
use std::collections::HashMap;
use std::io::{Write, BufReader, BufRead, Error};
use std::path::{Path, PathBuf};
use std::cell::Cell;
use regex::Regex;

#[derive(Debug)]
enum INST_TYPE {
    A,
    C,
    L
}

#[derive(Debug)]
enum C_INST_TYPE {
    COMP,
    DEST,
    JMP
}

fn main() -> Result<(), Error> {
    // init_dictionaries;

    let mut c_dict = HashMap::new();
    let mut d_dict = HashMap::new();
    let mut j_dict = HashMap::new();

    c_dict.insert("0", "101010".to_string());
    c_dict.insert("1", "111111".to_string());
    c_dict.insert("-1", "111010".to_string());
    c_dict.insert("D", "001100".to_string());
    c_dict.insert("A", "110000".to_string());
    c_dict.insert("!D", "001101".to_string());
    c_dict.insert("!A", "110001".to_string());
    c_dict.insert("-D", "001111".to_string());
    c_dict.insert("-A", "110011".to_string());
    c_dict.insert("D+1", "011111".to_string());
    c_dict.insert("A+1", "110111".to_string());
    c_dict.insert("D-1", "001110".to_string());
    c_dict.insert("A-1", "110010".to_string());
    c_dict.insert("D+A", "000010".to_string());
    c_dict.insert("D-A", "010011".to_string());
    c_dict.insert("A-D", "000111".to_string());
    c_dict.insert("D&A", "000000".to_string());
    c_dict.insert("D|A", "010101".to_string());
    j_dict.insert("JGT", "001".to_string());
    j_dict.insert("JEQ", "010".to_string());
    j_dict.insert("JGE", "011".to_string());
    j_dict.insert("JLT", "100".to_string());
    j_dict.insert("JNE", "101".to_string());
    j_dict.insert("JLE", "110".to_string());
    j_dict.insert("JMP", "111".to_string());
    // j_dict.insert("null", "000".to_string());
    d_dict.insert("M", "001".to_string());
    d_dict.insert("D", "010".to_string());
    d_dict.insert("MD", "011".to_string());
    d_dict.insert("A", "100".to_string());
    d_dict.insert("AM", "101".to_string());
    d_dict.insert("AD", "110".to_string());
    d_dict.insert("AMD", "111".to_string());
    // d_dict.insert("null", "000".to_string());

    let args: Vec<String> = env::args().collect();
    let binding = args[1].clone();
    let asm_path = Path::new(&binding);

    let num_re = Regex::new(r"\d+").unwrap();

    let file = File::open(asm_path)?;
    let buffered = BufReader::new(file);

    let mut hack_path = PathBuf::from(asm_path.clone());
    hack_path.set_extension("hack");
    let mut write_file = File::create(hack_path)?;

    let block_flag: Cell<bool> = Cell::new(false);
    let temp_num: Cell<u8> = Cell::new(0);

    for line in buffered.lines() {
        if line.as_ref().unwrap().starts_with("//"){
            continue;
        }

        let processed_line: String = line?.split("//").nth(0).unwrap().trim().to_string();
        println!("{}", processed_line.clone());
        if processed_line.clone().starts_with("@"){
            block_flag.set(true);
            let captures = num_re.captures(&processed_line).unwrap();
            temp_num.set(captures.get(0).unwrap().as_str().parse::<u8>().unwrap());
            continue;
            
        }

        if block_flag.get(){
            println!("{}", format!("{:013b}", temp_num.get()));
            block_flag.set(false);
            if !jmp_type_check(&processed_line) {
                let mut dest_comp = processed_line.split("=");

                let d_string = dest_comp.clone().nth(0).unwrap();
                let c_string = dest_comp.nth(1).unwrap();

                println!("{:?} {:?}", d_string.clone(), c_string.clone());
                let d_bin = &d_dict[d_string];
                let c_bin = &c_dict[c_string];
                // println!("{:?} {:?}", d_bin, c_bin);
                let bin_code = "111".to_string() + d_bin + c_bin + "000";
                writeln!(write_file,  "{}",bin_code)?;
            }

        }
        // println!("{}", processed_line);


    }

    // let current_dir = env::current_dir()?;
    Ok(())
}

fn jmp_type_check(inst: &str) -> bool {
    let inst_split = inst.split("=");
    if inst_split.count()==1{
        return true;
    }

    return false;
}