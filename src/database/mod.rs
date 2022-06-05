use sqlite::{Connection, Error, State};

pub mod json_loader;
use json_loader::*;

pub struct DbManager {
    connection: Connection,
}

impl DbManager {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let db_manager = DbManager {
            connection: sqlite::open(db_path)?,
        };
        db_manager.create_tables()?;
        Ok(db_manager)
    }

    fn create_tables(&self) -> Result<(), Error> {
        self.connection
            .execute(
                "
            CREATE TABLE if not exists laureates (id INTEGER, firstname TEXT, surname TEXT, motivation TEXT, share INTEGER, year INTEGER, category TEXT);
            ",
            )?;
        Ok(())
    }

    pub fn insert_data_to_db(&self, data_path: &str) -> Result<(), Error> {
        let data = load_data_from_json(data_path);
        for prize in data.prizes().iter() {
            if prize.laureates().is_none() {
                continue;
            }
            for laureat in prize.laureates().as_ref().unwrap() {
                let statement = format!(
                    "INSERT INTO laureates (id, firstname, surname, motivation, share, year, category) values ({}, '{}', '{}', '{}', {}, {}, '{}')",
                    &laureat.id().parse::<i32>().unwrap(),
                    &laureat.firstname().replace('\'', "''"),
                    match &laureat.surname() {
                        Some(surname) => surname.replace('\'', "''"),
                        None => String::from(""),
                    },
                    &laureat.motivation().replace('\'', "''"),
                    &laureat.share().parse::<i32>().unwrap(),
                    &prize.year().parse::<i32>().unwrap(),
                    &prize.category().replace('\'', "''"),
                );
                println!("{}", statement);
                self.connection.execute(statement.as_str())?;
            }
        }
        Ok(())
    }

    pub fn get_laureats_by_year(&self, year: u32) -> Result<Vec<Laureat>, Error> {
        let mut statement = self
            .connection
            .prepare(format!("SELECT * FROM laureates WHERE year == {}", year))?;

        let mut laureates = vec![];
        while let State::Row = statement.next().unwrap() {
            laureates.push(Laureat::new(
                statement.read::<String>(0).unwrap(),
                statement.read::<String>(1).unwrap(),
                Some(statement.read::<String>(2).unwrap()),
                statement.read::<String>(3).unwrap(),
                statement.read::<String>(4).unwrap(),
            ));
        }
        Ok(laureates)
    }

    pub fn get_laureats_by_category(&self, category: String) -> Result<Vec<Laureat>, Error> {
        let mut statement = self.connection.prepare(format!(
            "SELECT * FROM laureates WHERE category == '{}'",
            category
        ))?;

        let mut laureates = vec![];
        while let State::Row = statement.next().unwrap() {
            laureates.push(Laureat::new(
                statement.read::<String>(0).unwrap(),
                statement.read::<String>(1).unwrap(),
                Some(statement.read::<String>(2).unwrap()),
                statement.read::<String>(3).unwrap(),
                statement.read::<String>(4).unwrap(),
            ));
        }
        Ok(laureates)
    }

    pub fn get_laureats_since(&self, year: u32) -> Result<Vec<Laureat>, Error> {
        let mut statement = self
            .connection
            .prepare(format!("SELECT * FROM laureates WHERE year >= {}", year))?;

        let mut laureates = vec![];
        while let State::Row = statement.next().unwrap() {
            laureates.push(Laureat::new(
                statement.read::<String>(0).unwrap(),
                statement.read::<String>(1).unwrap(),
                Some(statement.read::<String>(2).unwrap()),
                statement.read::<String>(3).unwrap(),
                statement.read::<String>(4).unwrap(),
            ));
        }
        Ok(laureates)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serial_test::serial;
    use sqlite::Statement;

    use super::*;

    #[test]
    #[serial]
    fn should_create_db() -> Result<(), Error> {
        remove_db_if_present();
        DbManager::new("dummy.db")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn should_load_all_prizes() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/prize.json")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn should_handle_single_prize() -> Result<(), Error> {
        remove_db_if_present();

        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/single_prize.json")?;

        let mut statement = db_manager
            .connection
            .prepare("SELECT * FROM laureates")
            .unwrap();

        match_laureates(
            &mut statement,
            1002,
            String::from("Benjamin"),
            String::from("List"),
            String::from("\"for the development of asymmetric organocatalysis\""),
            2,
            2021,
            String::from("chemistry"),
        );

        match_laureates(
            &mut statement,
            1003,
            String::from("David"),
            String::from("MacMillan"),
            String::from("\"for the development of asymmetric organocatalysis\""),
            2,
            2021,
            String::from("chemistry"),
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn should_find_all_laureates_by_year() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_by_year(2020)?;
        let expected_laureates = create_laureates();

        laureates
            .into_iter()
            .zip(expected_laureates.into_iter().skip(3))
            .for_each(|(a, b)| assert_eq!(a, b));
        Ok(())
    }

    #[test]
    #[serial]
    fn should_return_empty_vec() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_by_year(2022)?;
        assert!(laureates.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn should_find_all_laureates_since_year() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_since(2020)?;
        let expected_laureates = create_laureates();

        laureates
            .into_iter()
            .zip(expected_laureates.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
        Ok(())
    }

    #[test]
    #[serial]
    fn should_find_all_laureates_since_single_year() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_since(2021)?;
        let expected_laureates = create_laureates();

        laureates
            .into_iter()
            .zip(expected_laureates.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
        Ok(())
    }

    #[test]
    #[serial]
    fn since_year_should_return_empty_vec() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_since(2022)?;

        assert!(laureates.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn should_find_all_laureates_by_category() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_by_category(String::from("chemistry"))?;
        let mut expected_laureates = create_laureates();
        expected_laureates.remove(5);
        expected_laureates.remove(2);

        laureates
            .into_iter()
            .zip(expected_laureates.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
        Ok(())
    }

    #[test]
    #[serial]
    fn by_category_should_return_empty_vec() -> Result<(), Error> {
        remove_db_if_present();
        let db_manager = DbManager::new("dummy.db")?;
        db_manager.insert_data_to_db("data/ten_category_prizes.json")?;

        let laureates = db_manager.get_laureats_by_category(String::from("biology"))?;

        assert!(laureates.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn do_not_create_new_db_if_one_exists() -> Result<(), Error> {
        remove_db_if_present();
        {
            let db_manager = DbManager::new("dummy.db")?;
            db_manager.insert_data_to_db("data/ten_category_prizes.json")?;
        }

        let db_manager = DbManager::new("dummy.db")?;
        let laureates = db_manager.get_laureats_since(2020)?;
        let expected_laureates = create_laureates();
        laureates
            .into_iter()
            .zip(expected_laureates.into_iter())
            .for_each(|(a, b)| assert_eq!(a, b));
        Ok(())
    }

    #[test]
    #[serial]
    fn opening_db_with_wrong_path() -> Result<(), Error> {
        remove_db_if_present();
        if let Err(_) = DbManager::new("not_existing_dir/dummy.db") {
            return Ok(());
        }
        panic!("Opening db in non-existing directory should fail. Possible missing impl!");
    }

    fn remove_db_if_present() {
        match fs::remove_file("dummy.db") {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    fn match_laureates(
        statement: &mut Statement,
        id: i64,
        firstname: String,
        surname: String,
        motivation: String,
        share: i64,
        year: i64,
        category: String,
    ) {
        statement.next().unwrap();
        assert_eq!(statement.read::<i64>(0).unwrap(), id);
        assert_eq!(statement.read::<String>(1).unwrap(), firstname);
        assert_eq!(statement.read::<String>(2).unwrap(), surname);
        assert_eq!(statement.read::<String>(3).unwrap(), motivation);
        assert_eq!(statement.read::<i64>(4).unwrap(), share);
        assert_eq!(statement.read::<i64>(5).unwrap(), year);
        assert_eq!(statement.read::<String>(6).unwrap(), category);
    }

    fn create_laureates() -> Vec<Laureat> {
        let mut laureates = vec![];
        // 2021
        // chemistry
        laureates.push(Laureat::new(
            String::from("1002"),
            String::from("Benjamin"),
            Some(String::from("List")),
            String::from("\"for the development of asymmetric organocatalysis\""),
            String::from("2"),
        ));
        laureates.push(Laureat::new(
            String::from("1003"),
            String::from("David"),
            Some(String::from("MacMillan")),
            String::from("\"for the development of asymmetric organocatalysis\""),
            String::from("2"),
        ));
        // peace
        laureates.push(Laureat::new(
            String::from("1004"),
            String::from("World Food Programme"),
            Some(String::from("")),
            String::from(
                "\"for its efforts to combat hunger, for its contribution to bettering conditions for peace in conflict-affected areas and for acting as a driving force in efforts to prevent the use of hunger as a weapon of war and conflict\"",
            ),
            String::from("1"),
        ));

        // 2020
        // chemistry
        laureates.push(Laureat::new(
            String::from("991"),
            String::from("Emmanuelle"),
            Some(String::from("Charpentier")),
            String::from("\"for the development of a method for genome editing\""),
            String::from("2"),
        ));
        laureates.push(Laureat::new(
            String::from("992"),
            String::from("Jennifer A."),
            Some(String::from("Doudna")),
            String::from("\"for the development of a method for genome editing\""),
            String::from("2"),
        ));
        // literature
        laureates.push(Laureat::new(
            String::from("993"),
            String::from("Louise"),
            Some(String::from("Gl\u{00fc}ck")),
            String::from(
                "\"for her unmistakable poetic voice that with austere beauty makes individual existence universal\"",
            ),
            String::from("1"),
        ));

        laureates
    }
}
