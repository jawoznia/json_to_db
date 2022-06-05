mod database;

use database::DbManager;
use sqlite::Error;

pub fn create_database(db_path: &str, data_path: &str) -> Result<DbManager, Error> {
    let db_manager = DbManager::new(db_path)?;
    db_manager.insert_data_to_db(data_path)?;
    Ok(db_manager)
}
