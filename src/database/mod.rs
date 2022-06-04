use sqlite::Connection;

mod json_loader;

pub fn init_db(db_path: &str, data_path: &str) {
    let connection = sqlite::open(db_path).unwrap();

    create_db(db_path, &connection);
}

fn create_db(db_path: &str, connection: &Connection) {
    connection
        .execute(
            "
        CREATE TABLE prizes (year INTEGER, category TEXT);
        CREATE TABLE laureat (id INTEGER, firstname TEXT, surname TEXT, motivation TEXT, share INTEGER, winning_year INTEGER);
        ",
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn remove_db_if_present() {
        match fs::remove_file("dummy.db") {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    #[test]
    fn should_create_db() -> std::io::Result<()> {
        remove_db_if_present();
        init_db("dummy.db", "");
        fs::remove_file("dummy.db")?;
        Ok(())
    }
}
