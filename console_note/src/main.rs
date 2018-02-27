extern crate console;

use std::io;
use std::thread;
use std::time::Duration;

use console::Term;

fn print_sample() -> io::Result<()> {
    let term = Term::stdout();
    term.write_line("Hello World!")?;
    thread::sleep(Duration::from_millis(2000));
    term.clear_line()?;
    Ok(())
}

fn main() {
    print_sample().unwrap();
}
