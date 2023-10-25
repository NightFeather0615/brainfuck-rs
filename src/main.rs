use std::{io::Read, env, fs::File};


#[derive(Clone)]
enum Token {
  IncrementPtr,
  DecrementPtr,
  Increment,
  Decrement,
  Output,
  Input,
  LoopBegin,
  LoopEnd
}

impl Token {
  fn tokenization(source: &str) -> Vec<Self> {
    let mut tokens: Vec<Self> = Vec::new();

    for char in source.chars() {
      let token: Option<Self> = match char {
        '>' => Some(Self::IncrementPtr),
        '<' => Some(Self::DecrementPtr),
        '+' => Some(Self::Increment),
        '-' => Some(Self::Decrement),
        '.' => Some(Self::Output),
        ',' => Some(Self::Input),
        '[' => Some(Self::LoopBegin),
        ']' => Some(Self::LoopEnd),
        _ => None
      };

      if let Some(token) = token {
        tokens.push(token);
      }
    }

    tokens
  }
}

enum Command {
  IncrementPtr,
  DecrementPtr,
  Increment,
  Decrement,
  Output,
  Input,
  Loop(Vec<Command>)
}

impl Command {
  fn parse_command(tokens: Vec<Token>) -> Vec<Self> {
    let mut commands: Vec<Self> = Vec::new();
    let mut loop_depth: i32 = 0;
    let mut loop_start: usize = 0;

    tokens.iter().enumerate().for_each(
      |(index, token): (usize, &Token)| {
        if loop_depth == 0 {
          let command: Option<Command> = match token {
            Token::IncrementPtr => Some(Self::IncrementPtr),
            Token::DecrementPtr => Some(Self::DecrementPtr),
            Token::Increment => Some(Self::Increment),
            Token::Decrement => Some(Self::Decrement),
            Token::Output => Some(Self::Output),
            Token::Input => Some(Self::Input),
            Token::LoopBegin => {
              loop_depth += 1;
              loop_start = index;
              None
            },
            Token::LoopEnd => panic!("Unclosed delimiter at {}", index)
          };
          
          if let Some(command) = command {
            commands.push(command);
          }
        } else {
          match token {
            Token::LoopBegin => {
              loop_depth += 1;
            }
            Token::LoopEnd => {
              loop_depth -= 1;

              if loop_depth == 0 {
                commands.push(
                  Self::Loop(
                    Self::parse_command(
                      tokens[(loop_start + 1)..index].to_vec()
                    )
                  )
                )
              }
            }
            _ => ()
          };
        }
      }
    );

    if loop_depth != 0 {
      panic!("Unclosed delimiter at {}", loop_start);
    }

    commands
  }
}


struct Machine {
  tape: Vec<u8>,
  ptr: usize
}

impl Machine {
  fn execute(&mut self, commands: &Vec<Command>) {
    commands.into_iter().for_each(
      |command: &Command| {
        match command {
          Command::IncrementPtr => {
            self.ptr += 1;
          },
          Command::DecrementPtr => {
            self.ptr -= 1;
          },
          Command::Increment => {
            self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
          },
          Command::Decrement => {
            self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
          },
          Command::Output => {
            print!("{}", self.tape[self.ptr] as char);
          },
          Command::Input => {
            let mut buf: [u8; 1] = [0; 1];
            std::io::stdin().read_exact(&mut buf).expect(
              "Failed to read input"
            );
            self.tape[self.ptr] = buf[0];
          },
          Command::Loop(loop_commands) => {
            while self.tape[self.ptr] != 0 {
              self.execute(loop_commands);
            }
          },
        }
      }
    );
  }

  fn run(source: &str) {
    let tokens: Vec<Token> = Token::tokenization(source);

    let commands: Vec<Command> = Command::parse_command(tokens);

    let mut machine: Machine = Machine {
      tape: vec![0u8; 30000],
      ptr: 0
    };

    machine.execute(&commands);
  }
}


fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("Simple Brainfuck interpreter.");
    println!("");
    println!("Usage: brainfuck-rs <file.bf>");
    std::process::exit(1);
  }

  let filename: &String = &args[1];

  let mut bf_file: File = File::open(&args[1]).expect(
    &format!("Source file `{}` not found", filename)
  );
  let mut bf_source: String = String::new();

  bf_file.read_to_string(&mut bf_source).expect(
    &format!("Failed to read `{}`", filename)
  );

  Machine::run(&bf_source);
}
