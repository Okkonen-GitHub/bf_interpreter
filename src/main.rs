use std::num::Wrapping;

#[cfg(test)]
mod test;

const TAPE_LEN: usize = 30_000;

#[derive(PartialEq, Debug)]
enum Command {
    MvRight,   // >
    MvLeft,    // <
    Increment, // +
    Decrement, // -
    Output,    // .
    Input,     // ,
    JumpPast,  // [
    JumpBack,  // ]
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Command as C;
        write!(
            f,
            "{}",
            match self {
                C::MvRight => '>',
                C::MvLeft => '<',
                C::Increment => '+',
                C::Decrement => '-',
                C::Output => '.',
                C::Input => ',',
                C::JumpPast => '[',
                C::JumpBack => ']',
            }
        )
    }
}

fn lex(source: &str) -> Vec<Command> {
    use Command as C;

    let mut commands = vec![];

    for ch in source.chars() {
        let command = match ch {
            '>' => Some(C::MvRight),
            '<' => Some(C::MvLeft),
            '+' => Some(C::Increment),
            '-' => Some(C::Decrement),
            '.' => Some(C::Output),
            ',' => Some(C::Input),
            '[' => Some(C::JumpPast),
            ']' => Some(C::JumpBack),
            _ => None,
        };
        if let Some(command) = command {
            commands.push(command);
        }
    }

    return commands;
}

fn run(commands: Vec<Command>, ascii_mode: bool) {
    use Command as C;
    // dbg!(&commands);
    let mut state: [Wrapping<u8>; TAPE_LEN] = [Wrapping(0u8); TAPE_LEN];
    let mut idx: usize = 0; // tape position
    let mut cdx: usize = 0; // command index

    loop {
        if cdx >= commands.len() {
            break; // stop the execution at the end of the sourcecode, might be buggy
        }
        let command = &commands[cdx];

        match command {
            C::MvRight => {
                if idx == TAPE_LEN - 1 {
                    idx = 0
                } else {
                    idx += 1
                }
            }
            C::MvLeft => {
                if idx == 0 {
                    idx = TAPE_LEN - 1
                } else {
                    idx -= 1
                }
            }
            C::Increment => state[idx] += 1,
            C::Decrement => state[idx] -= 1,
            C::Output => {
                let it = state[idx].0;
                if ascii_mode {
                    print!("{}", it as char);
                } else {
                    print!("{}", it);
                }
            }
            C::Input => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read stdin");
                let val = match input.trim().parse::<u8>() {
                    Ok(n) => Wrapping(n),
                    Err(e) => {
                        eprintln!("Failed to parse input: {e}. Defaulting to 0");
                        Wrapping(0)
                    }
                };
                state[idx] = val;
            }
            C::JumpPast => {
                if state[idx] == std::num::Wrapping(0) {
                    let mut jc: usize = 1;
                    while jc > 0 {
                        cdx += 1;
                        if commands[cdx] == C::JumpPast {
                            jc += 1
                        } else if commands[cdx] == C::JumpBack {
                            jc -= 1
                        }
                    }
                }
            }
            C::JumpBack => {
                if state[idx] != std::num::Wrapping(0) {
                    let mut jc: usize = 1;
                    while jc > 0 {
                        cdx -= 1;
                        if commands[cdx] == C::JumpBack {
                            jc += 1
                        } else if commands[cdx] == C::JumpPast {
                            jc -= 1
                        }
                    }
                    cdx -= 1
                }
            }
        }
        cdx += 1;
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        println!("Usage:\nbf_interpreter <program.b[f]>");
        return;
    }
    let ascii_mode = match args.iter().nth(1) {
        Some(arg) => arg == "-a" || arg == "--ascii",
        None => false,
    };
    let path = match args.last() {
        Some(path) => {
            if path.ends_with(".bf") || path.ends_with(".b") {
                path
            } else {
                eprintln!("Warning: Possibly not a brainf*ck sourcefile");
                path
            }
        }
        None => panic!("No second argument passed"),
    };

    let program_file = std::fs::read_to_string(path);
    let source = match program_file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error reading source file: {}", e);
            return;
        }
    };
    let program = lex(&source);
    run(program, ascii_mode);
}
