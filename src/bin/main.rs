use shell::*;

fn main() {
    let mut shell = Shell::new();
    let error = " is not recognized as an internal or external command,
operable program or batch file.";

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

        //Todo: run_cmd
        if !shell.input.is_empty() {
            let tokens = tokenize(&shell.input);
            if shell.run_built(tokenize(&shell.input))
                .is_err() && shell.run_cmd(&tokens)
                    .is_err() {
                    // Todo create a function that handles Errors.
                        print_prompt(&tokens[0]);
                        print_prompt(&error);
                    }
            println!();
        }

    } //end of loop 'shell
}
