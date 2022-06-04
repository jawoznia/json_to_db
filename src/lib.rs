mod json_loader;

use json_loader::load_json;

pub fn save_json_input_to_db(db_path: &str, json_path: &str) {
    create_db(db_path);

    load_json(json_path);
}

fn create_db(path: &str) {
    let connection = sqlite::open(path).unwrap();

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
        save_json_input_to_db("dummy.db", "data/prize.json");
        fs::remove_file("dummy.db")?;
        Ok(())
    }
}
