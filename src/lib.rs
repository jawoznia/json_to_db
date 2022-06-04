mod database;

use database::create_db;

pub fn save_json_input_to_db(db_path: &str, json_path: &str) {
    create_db(db_path);
}
