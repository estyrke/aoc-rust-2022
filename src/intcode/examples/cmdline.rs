use clap::Parser;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::Read;

extern crate intcode;
use intcode::machine::*;

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
  T: std::str::FromStr,
  T::Err: Error + Send + Sync + 'static,
  U: std::str::FromStr,
  U::Err: Error + Send + Sync + 'static,
{
  let pos = s
    .find('=')
    .ok_or_else(|| format!("invalid addr=value: no `=` found in `{}`", s))?;
  Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  program: String,

  #[clap(long, parse(try_from_str = parse_key_val), multiple_occurrences(true))]
  poke: Vec<(usize, Word)>,

  #[clap(long, multiple_occurrences(true))]
  peek: Vec<usize>,
}

fn main() {
  let mut program = String::new();
  let args = Args::parse();

  File::open(args.program)
    .unwrap()
    .read_to_string(&mut program)
    .unwrap();

  let mut m = Machine::new(&program);

  for (addr, value) in args.poke {
    println!("Poking {} at address {}", value, addr);
    m.mem.store(addr, value)
  }
  while !m.halted {
    println!("Enter input numbers, separated by space: ");

    let mut inp_line = String::new();
    stdin().read_line(&mut inp_line).unwrap();
    let inp = inp_line
      .split_whitespace()
      .map(|n| n.trim().parse::<Word>().unwrap())
      .collect();

    let outp = m.run(&inp);
    println!("{}", outp.iter().join(", "));
  }

  for addr in args.peek {
    println!("Peeking at {}: {}", addr, m.mem.load(addr));
  }
}
