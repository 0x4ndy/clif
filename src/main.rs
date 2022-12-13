use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {

    if let Err(e) = clif::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
