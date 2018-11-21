pub mod run;

use std::io::{self, Write};
use self::run::Run;

#[derive(Default, Clone)]
pub struct Shell {
    pub name: String,
    pub input: String,
}

impl Shell {
    pub fn new() -> Shell {
        let mut out = Shell::default();
        out.name = String::from("FairShell");
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


impl Run for Shell {
    fn name_change(&mut self, name: &str) {
        let prompt = String::from(name);
//        prompt.push_str(" $: ");
        self.name = prompt;
    }
}

pub fn print_ps1(name: &str) {
    let mut prompt = name.to_owned();
    prompt.push_str(" $: ");
  print_prompt(&prompt).expect("Faied");
    io::stdout().flush().unwrap();
}

pub fn print_prompt(input: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(input.as_bytes())?;

    Ok(())
}
