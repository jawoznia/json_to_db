use sqlite::{Connection, Error};

use super::json_loader::*;

pub struct DbManager {
    connection: Connection,
}

impl DbManager {
    pub fn new(db_path: &str) -> Self {
        let db_manager = DbManager {
            connection: sqlite::open(db_path).unwrap(),
        };
        match db_manager.create_tables() {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to create tables: {:?}", e),
        }
        db_manager
    }

    fn create_tables(&self) -> Result<(), Error> {
        self.connection
            .execute(
                "
            CREATE TABLE prizes (year INTEGER, category TEXT);
            CREATE TABLE laureates (id INTEGER, firstname TEXT, surname TEXT, motivation TEXT, share INTEGER, winning_year INTEGER);
            ",
            )?;
        Ok(())
    }

    pub fn insert_data_to_db(&self, data_path: &str) -> Result<(), Error> {
        let data = load_data_from_json(data_path);
        for prize in data.prizes().iter() {
            let statement = format!(
                "INSERT INTO prizes (year, category) values ({}, '{}')",
                &prize.year(),
                &prize.category()
            );
            self.connection.execute(statement)?;

            if prize.laureates().is_none() {
                continue;
            }
            for laureat in prize.laureates().as_ref().unwrap() {
                let statement = format!(
                    "INSERT INTO laureates (id, firstname, surname, motivation, share, winning_year) values ({}, '{}', '{}', '{}', {}, {})",
                    &laureat.id().parse::<i32>().unwrap(),
                    &laureat.firstname().replace('\'', "''"),
                    match &laureat.surname() {
                        Some(surname) => surname.replace('\'', "''"),
                        None => String::from(""),
                    },
                    &laureat.motivation().replace('\'', "''"),
                    &laureat.share().parse::<i32>().unwrap(),
                    &prize.year().parse::<i32>().unwrap(),
                );
                println!("{}", statement);
                self.connection.execute(statement.as_str())?;
            }
        }
        Ok(())
    }

    pub fn get_laureats_by_year(&self, year: u8) -> Result<Vec<Laureat>, Error> {
        Ok(vec![])
    }
    pub fn get_laureats_by_category(&self, category: String) -> Result<Vec<Laureat>, Error> {
        Ok(vec![])
    }
    pub fn get_laureats_since(&self, year: u8) -> Result<Vec<Laureat>, Error> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serial_test::serial;

    use super::*;

    fn remove_db_if_present() {
        match fs::remove_file("dummy.db") {
            Ok(_) => println!("Successfully removed dummy"),
            Err(_) => (),
        }
    }

    #[test]
    #[serial]
    fn should_create_db() -> Result<(), Error> {
        remove_db_if_present();
        DbManager::new("dummy.db");
        Ok(())
    }

    #[test]
    #[serial]
    fn should_load_all_prizes() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db");
        db_manager.insert_data_to_db("data/prize.json")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn should_handle_single_prize() -> Result<(), Error> {
        remove_db_if_present();

        let db_manager = DbManager::new("dummy.db");
        db_manager.insert_data_to_db("data/single_prize.json")?;
        let mut statement = db_manager
            .connection
            .prepare("SELECT * FROM prizes")
            .unwrap();

        statement.next().unwrap();
        assert_eq!(statement.read::<i64>(0).unwrap(), 2021);
        assert_eq!(statement.read::<String>(1).unwrap(), "chemistry");

        let mut statement = db_manager
            .connection
            .prepare("SELECT * FROM laureates")
            .unwrap();
        statement.next().unwrap();
        assert_eq!(statement.read::<i64>(0).unwrap(), 1002);
        assert_eq!(statement.read::<String>(1).unwrap(), "Benjamin");
        assert_eq!(statement.read::<String>(2).unwrap(), "List");
        assert_eq!(
            statement.read::<String>(3).unwrap(),
            "\"for the development of asymmetric organocatalysis\""
        );
        assert_eq!(statement.read::<i64>(4).unwrap(), 2);

        statement.next().unwrap();
        assert_eq!(statement.read::<i64>(0).unwrap(), 1003);
        assert_eq!(statement.read::<String>(1).unwrap(), "David");
        assert_eq!(statement.read::<String>(2).unwrap(), "MacMillan");
        assert_eq!(
            statement.read::<String>(3).unwrap(),
            "\"for the development of asymmetric organocatalysis\""
        );
        assert_eq!(statement.read::<i64>(4).unwrap(), 2);
        Ok(())
    }
}
