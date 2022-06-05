use sqlite::Error;

use self::json_loader::Laureat;
use db_manager::DbManager;

mod db_manager;
mod json_loader;

pub fn init_db(db_path: &str, data_path: &str) -> Result<(), Error> {
    let db_manager = DbManager::new(db_path)?;
    db_manager.insert_data_to_db(data_path)?;
    Ok(())
}

#[allow(dead_code, unused)]
pub fn get_laureats_by_year(year: u8) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}

#[allow(dead_code, unused)]
pub fn get_laureats_by_category(category: String) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}

#[allow(dead_code, unused)]
pub fn get_laureats_since(year: u8) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}
