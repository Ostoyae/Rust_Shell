use shell::{*, run::*};

fn main() {
    let mut shell = Shell::new();

    'shell: loop {
        print_ps1(&shell.name);
        match shell.get_line() {
            Ok(n) if n == 0 => break,
            Ok(n) if n > 0 => {
                shell.output_line(false);
            }
            Err(msg) => {
                eprintln!("{}", msg);
                continue 'shell;
            }
            _ => {}
        }

        //        let input = shell.input.to_owned();
        if !shell.input.is_empty() {
            let tokens = tokenize(&shell.input);
            if tokens.len() > 1 {
                shell.run_cmd(tokens);
            } else {
                shell.run_built(tokens);
            }
            print_prompt(&shell.input);
            println!();
        }
    }
}
