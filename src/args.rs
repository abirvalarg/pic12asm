use crate::str_err;

pub struct Args {
    pub input: String,
    pub output: String
}

impl Args {
    pub fn from_cmd() -> Result<Args, Box<dyn std::error::Error>> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 2 {
            Ok(Args {
                input: args[1].clone(),
                output: args[2].clone()
            })
        } else {
            Err(Box::new(NotEnoughArgs))
        }
    }
}

str_err!(NotEnoughArgs, "Not enough arguments");
