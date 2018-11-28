use crate::Shell;

pub(crate) trait Cmd {
    //    fn run_cmd(&mut self, tokens: &Vec<String>);
    fn name_change(&mut self, name: &str);
}

pub(crate) trait BltIn {
    //    fn run_built(&mut self, tokens: Vec<String>);
    fn exit(&mut self, tokens: Vec<String>);
    fn cd(tokens: &[String]);
    fn env();
    fn alias(tokens: &[String]);
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
    // Todo: cd implementation
    fn cd(_tokens: &[String]) {
        unimplemented!()
    }
    // Todo: print current env
    fn env() {
        unimplemented!()
    }
    // Todo Alias function
    fn alias(_tokens: &[String]) {
        unimplemented!()
    }
}
