use oci_runtime::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::build(&args);
    match conf {
        Err(s) => eprintln!("Problem parsing arguments: {}", s),
        Ok(conf) => oci_runtime::run(conf),
    }
}
