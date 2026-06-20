use std::io::{self, BufRead, Write};

use herb_game::{step, welcome, World};

fn main() -> io::Result<()> {
    let mut world = World::new();
    println!("{}", welcome());
    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut line)?;

        // To halt on Ctrl+D
        if line.len() == 0 {
            break;
        }

        line = line.trim().to_lowercase();
        if line == "quit" {
            break;
        }

        let result = step(&mut world, &line);
        println!("{result}");
    }

    Ok(())
}
