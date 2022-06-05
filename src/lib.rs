mod database;

use database::DbManager;

pub fn create_database_manager(db_path: &str, data_path: &str) -> Result<DbManager, String> {
    let db_manager = match DbManager::new(db_path) {
        Ok(db_manager) => db_manager,
        Err(e) => return Err(e.to_string()),
    };
    match db_manager.insert_data_to_db(data_path) {
        Ok(_) => Ok(db_manager),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn do_not_panic_on_bad_db_path() {
        remove_db_if_present();
        match create_database_manager("not_existing_dir/dummy.db", "data/prize.json") {
            Ok(_) => panic!("Error about wrong db path was not propagated!"),
            Err(_) => (),
        }
    }

    #[test]
    #[serial]
    fn do_not_panic_on_bad_json_path() {
        remove_db_if_present();
        match create_database_manager("dummy.db", "data/i_do_not_exist.json") {
            Ok(_) => panic!("Error about wrong json path was not propagated!"),
            Err(_) => (),
        }
    }

    #[test]
    #[serial]
    fn create_db_manager_on_with_proper_paths() {
        remove_db_if_present();
        match create_database_manager("dummy.db", "data/prize.json") {
            Ok(_) => (),
            Err(e) => panic!("Failed to create database manager: {:?}", e),
        }
    }

    fn remove_db_if_present() {
        match fs::remove_file("dummy.db") {
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
