use std::env;

use json_streams::load_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("called main");
    let mut args = env::args();
    let _program = args.next();
    let input = args.next().unwrap_or("-".to_string());
    eprintln!("input={}", input);
    for obj in load_from_file(&input)? {
        println!("{:?}", obj);
    }
    Ok(())
}
