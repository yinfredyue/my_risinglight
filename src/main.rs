use anyhow::Result; // Easily handling different `Result`
use my_risinglight::db::Database;
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let _ = rl.load_history("history.txt");

    let db = Database::new();

    // shell
    loop {
        let readline = rl.readline("sql> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.as_str() {
                    "" => (),
                    "quit" | "exit" => break,
                    sql => {
                        let mut res = db.run(sql)?;
                        println!("{}", res.remove(0));
                    }
                };
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => println!("Error reading input: {:?}", err),
        }
    }

    rl.save_history("history.txt")?;
    Ok(())
}
