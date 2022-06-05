mod database;

use database::DbManager;

pub fn create_database(db_path: &str, data_path: &str) -> Result<DbManager, String> {
    let db_manager = match DbManager::new(db_path) {
        Ok(db_manager) => db_manager,
        Err(e) => return Err(e.to_string()),
    };
    match db_manager.insert_data_to_db(data_path) {
        Ok(_) => Ok(db_manager),
        Err(e) => Err(e),
    }
}
