pub mod run;

use self::run::{BltIn, Cmd};
use std::io::{self, Write};

#[derive(Default)]
pub struct Shell {
    pub name: String,
    pub input: String,
}

impl Shell {
    pub fn new() -> Shell {
        let mut out = Shell::default();
        out.name = String::from("FerrisShell");
        out
    }

    pub fn get_line(&mut self) -> io::Result<usize> {
        self.input.clear();
        io::stdin().read_line(&mut self.input)
    }

    pub fn output_line(&mut self, is_print: bool) {
        let sl = self.input.trim_right().len();
        self.input.truncate(sl);
        if is_print {
            print_prompt(&self.input);
            println!();
        }
    }

    // Todo: have run cmd look through Paths.
    pub fn run_cmd(&mut self, tokens: &[String]) -> Result<(), ()> {
        //     unimplemented!()
        match tokens[0].as_ref() {
            "nc" => {
                self.name_change(tokens[1].as_ref());
            }
            "place" => {
                unimplemented!();
            }
            _ => return Err(())
        }
        Ok(())
    }

    //Todo: add set env
    pub fn run_built(&mut self, tokens: Vec<String>) ->  Result<(), ()> {
        match tokens[0].as_ref() {
            "exit" => self.exit(tokens),
            "cd" => Self::cd(&tokens),
            "env" => Self::env(),
            _ => return Err(())
        }
        Ok(())
    }
}

impl Cmd for Shell {
    // TODO: change env var ps1
    fn name_change(&mut self, name: &str) {
        let prompt = String::from(name);
        self.name = prompt;
    }
}

impl BltIn for Shell {
    fn exit(&mut self, tokens: Vec<String>) {
        unsafe {
            self.input = std::mem::uninitialized();
            self.name = std::mem::uninitialized();
        }
        std::mem::drop(tokens);
        std::process::exit(0);
    }

    fn cd(_tokens: &[String]) {
        unimplemented!()
    }

    fn env() {
        unimplemented!()
    }
}

pub fn print_ps1(name: &str) {
    let mut prompt = name.to_owned();
    prompt.push_str(" $: ");
    print_prompt(&prompt);
    io::stdout().flush().unwrap();
}

// TODO: use env PS1
pub fn print_prompt(input: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(input.as_bytes()).expect("whoops");
}

pub fn tokenize(input: &str) -> Vec<String> {
    let tok: Vec<String> = input.split_whitespace().map(|x| x.to_owned()).collect();
    tok
}
