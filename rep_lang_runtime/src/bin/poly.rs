use combine::{stream::position, EasyParser, StreamOnce};
use std::env;
use std::fs::File;
use std::io::prelude::*;

use rep_lang_concrete_syntax::{parse::program, util::pretty::to_pretty};

use poly::{env::Env, eval::eval_program, infer::infer_program};

fn main() -> std::io::Result<()> {
    let width = 80;
    let fp = get_fp()?;
    let mut file = File::open(fp)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match program().easy_parse(position::Stream::new(&contents[..])) {
        Err(err) => panic!("parse error:\n\n{}\n", err),
        Ok((prog, extra_input)) => {
            if extra_input.is_partial() {
                panic!("error: unconsumed input: {:?}", extra_input);
            } else {
                // println!("{}", to_pretty(prog.ppr(), width));
                match infer_program(Env::new(), &prog) {
                    Ok((sc, env)) => {
                        println!("{:?}\n\n{:?}\n", sc, env);
                        let ty = to_pretty(sc.ppr(), width);
                        let (val, _env) = eval_program(&prog);
                        let val_str = to_pretty(val.ppr(), width);
                        println!("(: {}\n   {}\n)", val_str, ty);
                        Ok(())
                    }
                    Err(err) => panic!("type error: {:?}", err),
                }
            }
        }
    }
}

fn get_fp() -> std::io::Result<String> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, fp] => Ok(fp.clone()),
        _ => panic!("wanted one filepath, got {:?}", &args[1..]),
    }
}
