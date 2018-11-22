
pub trait Cmd {
    fn run_cmd(&mut self, tokens : Vec<String>);
    fn name_change(&mut self, name: &str);
}

pub trait BltIn {
    fn run_built(&mut self, tokens : Vec<String>);
    fn exit(tokens : Vec<String>);
    fn cd(tokens : Vec<String>);
    fn env();
}
