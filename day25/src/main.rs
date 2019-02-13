use std::collections::HashSet;
use std::env;

enum State {
    A, B, C, D, E, F
}

enum Write {
    On, Off
}

enum Move {
    Left = -1, Right = 1
}

struct TuringMachine {
    state: State,
    tape: HashSet<i32>,
    cursor: i32,
}

impl TuringMachine {
    fn new() -> TuringMachine {
        TuringMachine {
            state: State::A,
            tape: HashSet::new(),
            cursor: 0,
        }
    }

    fn run(&mut self, steps: u32) -> usize {
        for _ in 0..steps {
            let (write, dir, ns) = match (&self.state, self.tape.contains(&self.cursor)) {
                (&State::A, false) => (Write::On,  Move::Right, State::B),
                (&State::A,  true) => (Write::Off, Move::Right, State::C),
                (&State::B, false) => (Write::Off, Move::Left,  State::A),
                (&State::B,  true) => (Write::Off, Move::Right, State::D),
                (&State::C, false) => (Write::On,  Move::Right, State::D),
                (&State::C,  true) => (Write::On,  Move::Right, State::A),
                (&State::D, false) => (Write::On,  Move::Left,  State::E),
                (&State::D,  true) => (Write::Off, Move::Left,  State::D),
                (&State::E, false) => (Write::On,  Move::Right, State::F),
                (&State::E,  true) => (Write::On,  Move::Left,  State::B),
                (&State::F, false) => (Write::On,  Move::Right, State::A),
                (&State::F,  true) => (Write::On,  Move::Right, State::E),
            };
            self.command(write, dir);
            self.state = ns;
        }
        self.tape.len()
    }

    fn command(&mut self, write: Write, dir: Move) {
        match write {
            Write::Off => self.tape.remove(&self.cursor),
            Write::On => self.tape.insert(self.cursor),
        };
        self.cursor += dir as i32;
    }
}

fn parse_args(args: env::Args) -> Option<u32> {
    args.skip(1).next()?.parse::<u32>().ok()
}

fn main() {
    let mut tm = TuringMachine::new();
    match parse_args(env::args()) {
        Some(n) => println!("{}", tm.run(n)),
        None => println!("Invalid input!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let mut tm = TuringMachine::new();
        let chksum = tm.run(6);
        assert_eq!(2, chksum);
    }

    #[test]
    fn extended_run() {
        let mut tm = TuringMachine::new();
        let chksum = tm.run(12_368_930);
        assert_eq!(2725, chksum);
    }

    #[test]
    fn command() {
        let mut tm = TuringMachine::new();
        tm.command(Write::On, Move::Left);
        assert_eq!(-1, tm.cursor);
        assert!(tm.tape.contains(&0));
    }
}
