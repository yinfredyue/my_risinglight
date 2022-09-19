use my_risinglight::{Database, DbError};
use std::path::Path;
use test_case::test_case;

#[test_case("01-01.slt")]
fn test_01_01(name: &str) {
    init_logger();
    let script = std::fs::read_to_string(Path::new("./sql").join(name)).unwrap();
    let mut tester = sqllogictest::Runner::new(Wrapper(Database::new()));
    tester.run_script(&script).unwrap();
}

#[test_case("01-03.slt")]
fn test_01_03(name: &str) {
    init_logger();
    let script = std::fs::read_to_string(Path::new("./sql").join(name)).unwrap();
    let mut tester = sqllogictest::Runner::new(Wrapper(Database::new()));
    tester.run_script(&script).unwrap();
}

struct Wrapper(Database);

impl sqllogictest::DB for Wrapper {
    type Error = DbError;
    fn run(&self, sql: &str) -> Result<String, Self::Error> {
        let output = self.0.run(sql)?;
        Ok(output.to_string())
    }
}

fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(env_logger::init);
}
