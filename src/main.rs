mod error;
mod args;
use std::collections::HashMap;
use regex::Regex;
use phf::phf_map;
use error::*;

type OpDecoder = dyn Fn(&HashMap<&str, usize>, Option<&str>, Option<&str>, usize) -> Result<[u8; 2], Box<dyn std::error::Error>>;
const OP_CODES: phf::Map<&'static str, &OpDecoder> = phf_map! {
    "ADDWF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8;
            let d = get_num(d.unwrap(), lbl)? as u8;
            Ok([0b1100_0000 | ((d & 1) << 5) | (f & 0b11111), 0b0001])
        }
    }),
    "ANDWF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8;
            let d = get_num(d.unwrap(), lbl)? as u8;
            Ok([0b0100_0000 | ((d & 1) << 5) | (f & 0b11111), 0b0001])
        }
    }),
    "CLRF" => &(|lbl, f, _, l| {
        if f == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8;
            Ok([0b0110_0000 | (f & 0b11111), 0])
        }
    }),
    "CLRW" => &(|_, _, _, _| {
        Ok([0b0100_0000, 0b0000])
    }),
    "COMF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b0100_0000 | f | (d << 5), 0b0010])
        }
    }),
    "DECF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1100_0000 | f | (d << 5), 0b0000])
        }
    }),
    "DECFSZ" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1100_0000 | f | (d << 5), 0b0010])
        }
    }),
    "INCF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1000_0000 | f | (d << 5), 0b0010])
        }
    }),
    "INCFSZ" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1100_0000 | f | (d << 5), 0b0011])
        }
    }),
    "IORWF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b0000_0000 | f | (d << 5), 0b0001])
        }
    }),
    "MOVF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b0000_0000 | f | (d << 5), 0b0010])
        }
    }),
    "MOVWF" => &(|lbl, f, _, l| {
        if f == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            Ok([0b0010_0000 | f, 0b0000])
        }
    }),
    "NOP" => &(|_, _, _, _| { Ok([0, 0]) }),
    "RLF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b0100_0000 | f | (d << 5), 0b0011])
        }
    }),
    "RRF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b0000_0000 | f | (d << 5), 0b0011])
        }
    }),
    "SUBWF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1000_0000 | f | (d << 5), 0b0000])
        }
    }),
    "SWAPF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1000_0000 | f | (d << 5), 0b0011])
        }
    }),
    "XORWF" => &(|lbl, f, d, l| {
        if f == None || d == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let d = get_num(d.unwrap(), lbl)? as u8 & 0b1;
            Ok([0b1000_0000 | f | (d << 5), 0b0001])
        }
    }),
    "BCF" => &(|lbl, f, b, l| {
        if f == None || b == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let b = get_num(b.unwrap(), lbl)? as u8 & 0b111;
            Ok([f | (b << 5), 0b0100])
        }
    }),
    "BSF" => &(|lbl, f, b, l| {
        if f == None || b == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let b = get_num(b.unwrap(), lbl)? as u8 & 0b111;
            Ok([f | (b << 5), 0b0101])
        }
    }),
    "BTFSC" => &(|lbl, f, b, l| {
        if f == None || b == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let b = get_num(b.unwrap(), lbl)? as u8 & 0b111;
            Ok([f | (b << 5), 0b0110])
        }
    }),
    "BTFSS" => &(|lbl, f, b, l| {
        if f == None || b == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b11111;
            let b = get_num(b.unwrap(), lbl)? as u8 & 0b111;
            Ok([f | (b << 5), 0b0111])
        }
    }),
    "ANDLW" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1110])
        }
    }),
    "CALL" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1001])
        }
    }),
    "CLRWDT" => &(|_, _, _, _| { Ok([0b0100, 0]) }),
    "GOTO" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u16 & 0x1ff;
            Ok([(k & 0xff) as u8, (0b1010 | (k >> 8)) as u8])
        }
    }),
    "IORLW" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1101])
        }
    }),
    "MOVLW" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1100])
        }
    }),
    "OPTION" => &(|_, _, _, _| { Ok([0b0010, 0]) }),
    "RETLW" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1000])
        }
    }),
    "SLEEP" => &(|_, _, _, _| { Ok([0b0011, 0]) }),
    "TRIS" => &(|lbl, f, _, l| {
        if f == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let f = get_num(f.unwrap(), lbl)? as u8 & 0b0111;
            Ok([f, 0b0000])
        }
    }),
    "XORLW" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u8 & 0xff;
            Ok([k, 0b1111])
        }
    }),
    "DATA" => &(|lbl, k, _, l| {
        if k == None {
            Err(Box::new(BadOpArgs(l + 1)))
        } else {
            let k = get_num(k.unwrap(), lbl)? as u16 & 0x0fff;
            Ok([(k & 0xff) as u8, (k >> 8) as u8])
        }
    })
};

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
    for (line_num, line) in input.lines().enumerate() {
        let line = line.split(';').next().unwrap();
        let content = line.split(':').collect::<Vec<&str>>();
        match content.len() {
            1 => {
                let instr = content[0].trim();
                if instr != "" {
                    instrs.push((line_num, instr));
                }
            }
            2 => {
                let label = content[0].trim();
                let instr = content[1].trim();
                symbols.insert(label, instrs.len());
                if instr != "" {
                    instrs.push((line_num, instr));
                }
            }
            _ => return Err(Box::new(error::SyntaxError {line: line_num + 1}))
        }
    }

    let mut output = Vec::new();
    let instr_re= Regex::new(r"(?x)(?P<op>[[:alpha:]]+) (?:\s+(?P<arg1>[[:alpha:][0-9]]+)(?:\s*,\s*(?P<arg2>[0-9]+))?)?").unwrap();
    for (line_num, instr) in instrs.iter() {
        //println!("{:?}", instr_re.captures(instr));
        match instr_re.captures(instr) {
            Some(cap) => {
                let instr = cap.name("op").unwrap().as_str().to_uppercase();
                let decoder = match OP_CODES.get(&instr) {
                    Some(d) => d,
                    None => return Err(Box::new(InvalidInstruction(instr.into(), *line_num)))
                };
                let arg1 = match cap.name("arg1") {
                    Some(m) => Some(m.as_str()),
                    None => None
                };
                let arg2 = match cap.name("arg2") {
                    Some(m) => Some(m.as_str()),
                    None => None
                };
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

fn get_num(lit: &str, labels: &std::collections::HashMap<&str, usize>)
    -> Result<usize, Box<dyn std::error::Error>> {
    let ch = lit.chars().collect::<Vec<char>>();
    if ch[0].is_digit(10) {
        if lit.len() > 2 && ch[0] == '0' && ch[1] != '0' {
            let base = match ch[1] {
                'x' => 16,
                'o' => 8,
                'b' => 2,
                _ => return Err(Box::new(SyntaxError{line: 0}))
            };
            Ok(usize::from_str_radix(&lit[2..], base)?)
        } else {
            Ok(lit.parse()?)
        }
    } else {
        match labels.get(lit) {
            Some(val) => Ok(*val),
            None => Err(Box::new(UndefinedLabel(lit.into())))
        }
    }
}
