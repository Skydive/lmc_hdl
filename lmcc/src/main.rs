
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate bitvec;

use std::env;
use std::fs;
use std::io::Write;
use std::vec::Vec;
use std::collections::HashMap;

use regex::Regex;
use bitvec::prelude::*;

lazy_static! {
    static ref REGEX_COMMENT: Regex = Regex::new(r#"^\s*//"#).unwrap();
    static ref REGEX_LABEL: Regex = Regex::new(r"^(.+):(?:\s(.*))?$").unwrap();
    static ref REGEX_INST: Regex = Regex::new(r"^\s*(\w{3})(?:\s(\w+))?$").unwrap();
    static ref INST_MAP: HashMap<String, BitVec> = {
        let mut m = HashMap::new();
        m.insert("HLT".to_string(), bitvec![0,0,0]);
        m.insert("ADD".to_string(), bitvec![1,0,0]);
        m.insert("SUB".to_string(), bitvec![0,1,0]);
        m.insert("STA".to_string(), bitvec![1,1,0]);
        m.insert("BRP".to_string(), bitvec![0,0,1]);
        m.insert("LDA".to_string(), bitvec![1,0,1]);
        m.insert("BRA".to_string(), bitvec![0,1,1]);
        m.insert("BRZ".to_string(), bitvec![1,1,1]);
        m
    };
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let argsv = env::args().collect::<Vec<String>>();

    if argsv.len() < 3 {
        println!("ARGS INVALID");
        return Ok(());
    }

    let mut prog_bytes: Vec<u8> = Vec::new();
    let mut stack_labels: HashMap<String, u32> = HashMap::new();

    let data = fs::read_to_string(argsv[1].clone())?;
    let out_file = argsv[2].clone();
    let lines = data.split("\n")
        .filter(|x| x.trim() != "")
         // STRIP COMMENTS
        .map(|x| x.trim())
        .filter(|x| !REGEX_COMMENT.is_match(x))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("LINES:\n{}\n", lines.join("\n"));

    let mut lines_stripped: Vec<String> = Vec::new();

    let mut cur_line = 0;
    // Get stack labels + cleanse
    for line in lines.iter() {
        if let Some(c) = REGEX_LABEL.captures(line) {
            let stk = c.get(1).unwrap().as_str();
            stack_labels.insert(stk.to_string(), cur_line);
            if let Some(stack_val) = c.get(2) {
                lines_stripped.push(c.get(0).unwrap().as_str().to_string());
                cur_line += 1;
            }
            continue;
        } else if let Some(c) = REGEX_INST.captures(line) {
            lines_stripped.push(c.get(0).unwrap().as_str().to_string());
            cur_line += 1;
            continue;
        }
    }

    println!("STRP:\n{}\n", lines_stripped.join("\n"));

    let mut lines_bits: Vec<BitVec> = Vec::new();
    for line in lines_stripped.iter() {
        if let Some(c) = REGEX_LABEL.captures(line) {
            if let Some(stack_val) = c.get(2) {
                let store_val:u16 = stack_val.as_str().to_string().parse().unwrap();
                let mut bv = bitvec![0; 16];
                bv[0..16].store::<u16>(store_val);
                lines_bits.push(bv);
            } else {
                println!("COMPILER ERROR!");
                break;
            }
            continue;
        } else if let Some(c) = REGEX_INST.captures(line) {
            let ins_str = c.get(1).unwrap().as_str();
            let bv = INST_MAP.get(&ins_str.to_string()).unwrap();


            let mut sbv = bitvec![0; 16];
            println!("eh: {}", bv);
            sbv.set(0, bv[0]);
            println!("eh");
            sbv.set(1, bv[1]);
            sbv.set(2, bv[2]);
            sbv.set(3, false);

            if let Some(arg) = c.get(2) {
                let arg_str = arg.as_str().to_string();
                let stk_loc:u16 = *stack_labels.get(&arg_str).unwrap() as u16;
                println!("arg_str: {} - {} ", arg_str, stk_loc);
                sbv[4..16].store::<u16>(stk_loc);
            }
            println!("{}", sbv);

            lines_bits.push(sbv);
            continue;
        }
    }

    let mut out_hex: Vec<String> = Vec::new();

    let mut lc = 0;
    for l in lines_bits {
        println!("{:02} {}\t{}", lc, lines_stripped[lc], l);
        lc += 1;

        let word = l[0..16].load::<u16>();
        out_hex.push(format!("{:04x}\n", word))
    }

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_file.clone())?
        .write_all(&out_hex.join("").as_bytes())?;

    println!("{} created!", out_file.clone());
    Ok(())
}
