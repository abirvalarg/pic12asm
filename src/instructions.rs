use phf::phf_map;
use std::collections::HashMap;
use crate::util::get_num;
use crate::error::*;

type OpDecoder = dyn Fn(&HashMap<&str, usize>, Option<&str>, Option<&str>, usize) -> Result<[u8; 2], Box<dyn std::error::Error>>;
pub const OP_CODES: phf::Map<&'static str, &OpDecoder> = phf_map! {
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
