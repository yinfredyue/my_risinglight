use my_risinglight::db::{Database, DbError};
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<(), DbError> {
    let mut rl = Editor::<()>::new().expect("Open editor error");
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

    rl.save_history("history.txt")
        .expect("Save editor history error");
    Ok(())
}
