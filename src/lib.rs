mod database;

use database::init_db;

pub fn save_json_input_to_db(db_path: &str, json_path: &str) {
    init_db(db_path, json_path);
}
