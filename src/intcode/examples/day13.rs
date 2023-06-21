extern crate intcode;
use crossterm::event::read;
use crossterm::style::StyledContent;
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor::MoveTo, execute, style::Print, style::PrintStyledContent, Result};
use intcode::machine::*;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdout;

fn main() -> Result<()> {
    fn string_to_ascii(s: &str) -> Vec<Word> {
        s.chars().map(|c| c as i64).collect()
    }

    fn tile_to_string(c: Word) -> StyledContent<&'static str> {
        match c {
            0 => " ".black(),
            1 => "█".magenta(),
            2 => "#".white(),
            3 => "-".white(),
            4 => "o".white(),
            _ => "?".white(),
        }
    }

    let mut program = String::new();
    File::open("13_input.txt")
        .unwrap()
        .read_to_string(&mut program)
        .unwrap();

    let mut m = Machine::new(&program);
    execute!(stdout(), Clear(ClearType::All))?;

    m.mem.store(0, 2);
    let mut score = 0;
    let mut blocks = 0;

    while !m.halted {
        for (&x, &y, &b) in m.run(&vec![0]).iter().tuples() {
            if (x, y) == (-1, 0) {
                score = b;
            } else {
                if b == 2 {
                    blocks += 1;
                }
                execute!(
                    stdout(),
                    MoveTo(x as u16, y as u16),
                    PrintStyledContent(tile_to_string(b)),
                    MoveTo(10, 0),
                    Print(score)
                )
                .or_else(|e| execute!(stdout(), Clear(ClearType::All), Print(e)))?;
            }
        }
    }

    enable_raw_mode()?;
    read()?;
    disable_raw_mode()?;

    println!("\n{}", blocks);
    Ok(())
}
