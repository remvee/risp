use std::io;
use std::io::Read;

mod eval;
mod parse;

fn main() -> io::Result<()> {
    let mut code = String::new();
    io::stdin().read_to_string(&mut code)?;
    match eval::eval(&code) {
        Ok(value) => println!("{:?}", value),
        Err(why) => panic!("{:?}", why),
    }
    Ok(())
}
