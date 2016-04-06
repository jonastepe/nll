#![feature(question_mark)]

extern crate docopt;
extern crate lalrpop_intern;
extern crate graph_algorithms;
extern crate nll_repr;
extern crate rustc_serialize;

use docopt::Docopt;
use nll_repr::repr::*;
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;

mod env;
use self::env::Environment;
mod graph;
mod region;
mod regionck;
mod relate;
use self::graph::FuncGraph;

fn main() {
    let args: Args =
        Docopt::new(USAGE)
        .and_then(|d| d.argv(args()).decode())
        .unwrap_or_else(|e| e.exit());

    for input in &args.arg_inputs {
        match process_input(&args, input) {
            Ok(()) => { }
            Err(err) => {
                println!("{}: {}", input, err);
                process::exit(1);
            }
        }
    }
}

fn process_input(args: &Args, input: &str) -> Result<(), Box<Error>> {
    let ballast = Ballast::new();
    let mut arena = Arena::new(&ballast);
    let mut file_text = String::new();
    let mut file = try!(File::open(input));
    if file.read_to_string(&mut file_text).is_err() {
        return try!(Err(String::from("not UTF-8")));
    }
    let func = try!(Func::parse(&mut arena, &file_text));
    let graph = FuncGraph::new(func);
    let env = Environment::new(&graph);

    if args.flag_dominators {
        env.dump_dominators();
    }

    if args.flag_post_dominators {
        env.dump_postdominators();
    }

    try!(regionck::region_check(&env));

    Ok(())
}

const USAGE: &'static str = "
Usage: nll [options] <inputs>...

Options:
  --dominators
  --post-dominators
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_inputs: Vec<String>,
    flag_dominators: bool,
    flag_post_dominators: bool,
}
