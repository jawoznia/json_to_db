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

fn load_json(path: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_db() {
        save_json_input_to_db("dummy.db", "data/prizes.json");
    }
}
