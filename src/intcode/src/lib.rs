pub mod machine;

pub use machine::*;

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn day_7_2() {
        let mut program = String::new();
        File::open("7_input.txt")
            .unwrap()
            .read_to_string(&mut program)
            .unwrap();

        let mut max_signal = 0;
        let phase_settings: Vec<Word> = vec![5, 6, 7, 8, 9];
        let mut ms: Vec<Machine> = (0..5).map(|_| Machine::new(&program)).collect();

        for phases in phase_settings.iter().permutations(phase_settings.len()) {
            for i in 0..5 {
                ms[i].reset();
                ms[i].run(&[*phases[i]].to_vec());
            }

            let mut signal: Word = 0;
            while !ms[0].halted {
                for i in 0..5 {
                    signal = ms[i].run(&[signal].to_vec())[0];
                }
            }
            if signal > max_signal {
                max_signal = signal
            }
        }
        println!("Max signal is {}", max_signal);
        assert_eq!(max_signal, 22476942);
    }
    #[test]
    fn day_17_2() {
        fn string_to_ascii(s: &str) -> Vec<Word> {
            s.chars().map(|c| c as i64).collect()
        }

        fn ascii_to_string(chars: &Vec<Word>) -> String {
            return chars
                .into_iter()
                .map(|&c| (c as u8 as char))
                .collect::<String>();
        }

        let mut program = String::new();
        File::open("17_input.txt")
            .unwrap()
            .read_to_string(&mut program)
            .unwrap();

        let mut m = Machine::new(&program);
        // day 17:2
        let a = "L,12,L,12,R,12\n";
        let b = "L,8,L,8,R,12,L,8,L,8\n";
        let c = "L,10,R,8,R,12\n";
        let main = "A,A,B,C,C,A,B,C,A,B\n";
        m.mem.store(0, 2);
        assert_eq!(ascii_to_string(&m.run(&vec![])).ends_with("Main:\n"), true);
        assert_eq!(
            ascii_to_string(&m.run(&string_to_ascii(main))),
            "Function A:\n"
        );
        assert_eq!(
            ascii_to_string(&m.run(&string_to_ascii(a))),
            "Function B:\n"
        );
        assert_eq!(
            ascii_to_string(&m.run(&string_to_ascii(b))),
            "Function C:\n"
        );
        assert_eq!(
            ascii_to_string(&m.run(&string_to_ascii(c))),
            "Continuous video feed?\n"
        );
        let mut fin = m.run(&string_to_ascii("y\n"));
        let answer = fin.pop();

        /*for m in ascii_to_string(&fin).split("\n\n") {
            sleep(Duration::from_millis(10));
            let o = stdout();
            let mut l = o.lock();
            l.write_all(b"\n\n").unwrap();
            l.write_all(m.as_bytes()).unwrap();
            l.flush().unwrap();
        }*/

        assert_eq!(answer, Some(1499679));
    }

    #[test]
    fn day_9() {
        let mut program = String::new();
        File::open("9_input.txt")
            .unwrap()
            .read_to_string(&mut program)
            .unwrap();

        let mut m = Machine::new(&program);

        assert_eq!(m.run(&vec![1])[0], 2494485073);
        m.reset();
        assert_eq!(m.run(&vec![2])[0], 44997);
    }
}
