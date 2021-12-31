mod error;
mod args;
mod instructions;
mod util;
use std::collections::HashMap;
use regex::Regex;
use error::*;
use instructions::OP_CODES;

fn main() {
    match args::Args::from_cmd() {
        Ok(args) => {
            if let Err(err) = run(args) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn run(args: args::Args) -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(args.input)?;
    let mut instrs = Vec::new();
    let mut symbols = HashMap::new();
    let mut num_banks = 0;
    for (line_num, line) in input.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        let content = line.split(':').collect::<Vec<&str>>();
        if content[0].trim().to_uppercase() == "BANK" {
            num_banks += 1;
            instrs.resize(num_banks * 512, (0, "NOP"));
        } else {
            match content.len() {
                1 => {
                    let instr = content[0].trim();
                    if !instr.is_empty() {
                        instrs.push((line_num, instr));
                    }
                }
                2 => {
                    let label = content[0].trim();
                    let instr = content[1].trim();
                    symbols.insert(label, instrs.len());
                    if !instr.is_empty() {
                        instrs.push((line_num, instr));
                    }
                }
                _ => return Err(Box::new(error::SyntaxError {line: line_num + 1}))
            }
            if instrs.len() / 512 != num_banks {
                return Err(Box::new(DoesnotFit(line_num + 1)));
            }
        }
    }

    let mut output = Vec::new();
    let instr_re= Regex::new(r"(?x)(?P<op>[[:alpha:]]+) (?:\s+(?P<arg1>([[:alpha:][0-9]_]+|'.'))(?:\s*,\s*(?P<arg2>[0-9]+))?)?").unwrap();
    for (line_num, instr) in instrs.iter() {
        match instr_re.captures(instr) {
            Some(cap) => {
                let instr = cap.name("op").unwrap().as_str().to_uppercase();
                let decoder = match OP_CODES.get(&instr) {
                    Some(d) => d,
                    None => return Err(Box::new(InvalidInstruction(instr, *line_num + 1)))
                };
                let arg1 = cap.name("arg1").map(|m| m.as_str());
                let arg2 = cap.name("arg2").map(|m| m.as_str());
                let op_code: [u8; 2] = decoder(&symbols, arg1, arg2, *line_num)?;
                output.push(op_code[0]);
                output.push(op_code[1]);
            }
            None => return Err(Box::new(SyntaxError{line: line_num + 1}))
        }
    }

    std::fs::write(args.output, output)?;
    Ok(())
}
