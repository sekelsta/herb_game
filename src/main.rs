use std::io::{self, BufRead, Write};

use herb_game::{step, welcome, welcome_on_load, save_to_json, load_from_json, restart_save};

fn main() -> io::Result<()> {
    let path = "save.json";
    match std::fs::read_to_string(path) {
        Ok(contents) => {
            load_from_json(contents.as_str());
            println!("{}", welcome_on_load());
        },
        Err(_) => println!("{}", welcome()),
    }

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
        if line == "close shop" {
            println!("This will delete your progress and start over. Are you sure? yes/no");
            let mut confirmation = String::new();
            stdin.lock().read_line(&mut confirmation)?;
            confirmation = confirmation.trim().to_lowercase();
            if confirmation.starts_with("y") {
                restart_save();
                println!("{}", welcome());
            } else {
                println!("Nevermind, then.");
            }
            continue;
        }

        let result = step(&line);
        println!("{result}");


        let mut output = std::fs::File::create(path)?;
        write!(output, "{}", save_to_json())?;
    }

    Ok(())
}
