use sqlite::{Connection, Error};

mod json_loader;

pub fn init_db(db_path: &str, data_path: &str) -> Result<(), Error> {
    let connection = sqlite::open(db_path).unwrap();

    create_db(&connection)?;
    insert_data_to_db(data_path, &connection)?;
    Ok(())
}

fn create_db(connection: &Connection) -> Result<(), Error> {
    connection
        .execute(
            "
        CREATE TABLE prizes (year INTEGER, category TEXT);
        CREATE TABLE laureates (id INTEGER, firstname TEXT, surname TEXT, motivation TEXT, share INTEGER, winning_year INTEGER);
        ",
        )?;
    Ok(())
}

fn insert_data_to_db(data_path: &str, connection: &Connection) -> Result<(), Error> {
    let data = json_loader::load_data_from_json(data_path);
    for prize in data.prizes().iter() {
        let statement = format!(
            "INSERT INTO prizes (year, category) values ({}, '{}')",
            &prize.year(),
            &prize.category()
        );
        connection.execute(statement)?;

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
            connection.execute(statement.as_str())?;
        }
    }
    Ok(())
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
        init_db("dummy.db", "data/prize.json")?;
        Ok(())
    }

    #[test]
    #[serial]
    fn should_handle_single_prize() -> Result<(), Error> {
        remove_db_if_present();
        init_db("dummy.db", "data/single_prize.json")?;

        let connection = sqlite::open("dummy.db").unwrap();
        let mut statement = connection.prepare("SELECT * FROM prizes").unwrap();

        statement.next().unwrap();
        assert_eq!(statement.read::<i64>(0).unwrap(), 2021);
        assert_eq!(statement.read::<String>(1).unwrap(), "chemistry");

        let mut statement = connection.prepare("SELECT * FROM laureates").unwrap();
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