use sqlite::Error;

use self::json_loader::Laureat;
use db_manager::{create_db, insert_data_to_db};

mod db_manager;
mod json_loader;

pub fn init_db(db_path: &str, data_path: &str) -> Result<(), Error> {
    let connection = sqlite::open(db_path).unwrap();

    create_db(&connection)?;
    insert_data_to_db(data_path, &connection)?;
    Ok(())
}

pub fn get_laureats_by_year(year: u8) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}
pub fn get_laureats_by_category(category: String) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}
pub fn get_laureats_since(year: u8) -> Result<Vec<Laureat>, Error> {
    Ok(vec![])
}
