use seventh_core::historical::run;
use std::process;

fn main() {
    match run() {
        Ok(data) => println!("{:?}", data),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
