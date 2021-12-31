use phf::phf_map;
use crate::error::*;

pub fn get_num(lit: &str, labels: &std::collections::HashMap<&str, usize>)
    -> Result<usize, Box<dyn std::error::Error>> {
    let ch = lit.chars().collect::<Vec<char>>();
    if ch.len() == 3 && ch[0] == '\'' && ch[2] == '\'' {
        Ok(ch[1] as usize)
    } else if ch[0].is_digit(10) {
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
    } else if let Some(val) = REGS.get(lit) {
        Ok(*val)
    } else {
        match labels.get(lit) {
            Some(val) => Ok(*val),
            None => Err(Box::new(UndefinedLabel(lit.into())))
        }
    }
}

pub static REGS: phf::Map<&'static str, usize> = phf_map! {
    "INDF" => 0,
    "TMR0" => 1,
    "PCL" => 2,
    "STATUS" => 3,
    "FSR" => 4,
    "OSCCAL" => 5,
    "GPIO" => 6,
    "GPIOB" => 6,
    "GOIPC" => 7
};
