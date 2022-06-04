mod db_manager;
mod json_loader;

use db_manager::create_db;
use json_loader::load_json;

pub fn save_json_input_to_db(db_path: &str, json_path: &str) {
    create_db(db_path);

    load_json(json_path);
}
