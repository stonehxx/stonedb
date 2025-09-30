use std::io::{Write, stdin, stdout};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        run(input);
    }
}

fn run(input: String) {

}
