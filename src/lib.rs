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
}

impl Cmd for Shell {
    fn run_cmd(&mut self, tokens: Vec<String>) {
        unimplemented!()
//        match tokens[0].as_ref() {
//            "nc" => self.name_change(tokens[1].as_ref()),
//            _ => {},
//        }
    }

    fn name_change(&mut self, name: &str) {
        let prompt = String::from(name);
        //        prompt.push_str(" $: ");
        self.name = prompt;
    }
}

impl BltIn for Shell {
    fn run_built(&mut self, tokens: Vec<String>) {
        match tokens[0].as_ref() {
            "exit" => Self::exit(tokens),
            "cd" => Self::cd(tokens),
            "env" => Self::env(),
            _ => ()
        }
    }

    fn exit(tokens: Vec<String>) {
        std::process::exit(0);
    }

    fn cd(tokens: Vec<String>) {
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

    pub fn print_prompt(input: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(input.as_bytes()).expect("whoops");
    }

    pub fn tokenize(input: &str) -> Vec<String> {
        let tok: Vec<String> = input.split_whitespace().map(|x| x.to_owned()).collect();
        tok
    }
