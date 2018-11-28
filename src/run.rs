pub(crate) trait Cmd {
    //    fn run_cmd(&mut self, tokens: &Vec<String>);
    fn name_change(&mut self, name: &str);
}

pub(crate) trait BltIn {
    //    fn run_built(&mut self, tokens: Vec<String>);
    fn exit(&mut self, tokens: Vec<String>);
    fn cd(tokens: &[String]);
    fn env();
}
