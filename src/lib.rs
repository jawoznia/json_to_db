use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Prizes {
    prizes: Vec<Prize>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Prize {
    year: String,
    category: String,
    laureates: Vec<Laureat>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Laureat {
    id: String,
    firstname: String,
    surname: String,
    motivation: String,
    share: String,
}

pub fn save_json_input_to_db(db_path: &str, json_path: &str) {
    create_db(db_path);

    load_json(json_path);
}

fn create_db(path: &str) {
    let connection = sqlite::open(path).unwrap();

    connection
        .execute(
            "
        CREATE TABLE prizes (year INTEGER, category TEXT);
        CREATE TABLE laureat (id INTEGER, firstname TEXT, surname TEXT, motivation TEXT, share INTEGER, winning_year INTEGER);
        ",
        )
        .unwrap();
}

fn load_json(path: &str) -> Prizes {
    let json = fs::read_to_string(path).expect("Unable to read file.");
    println!("{}", json);
    serde_json::from_str::<Prizes>(json.as_str()).expect("JSON was not well-formatted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_db() -> std::io::Result<()> {
        save_json_input_to_db("dummy.db", "data/prize.json");
        fs::remove_file("dummy.db")?;
        Ok(())
        // todo!();
    }

    #[test]
    fn should_loud_dummy_json() {
        let prizes = load_json("data/single_prize.json");
        let expected_prize = Prizes {
            prizes: vec![Prize {
                year: String::from("2021"),
                category: String::from("chemistry"),
                laureates: vec![
                    Laureat {
                        id: String::from("1002"),
                        firstname: String::from("Benjamin"),
                        surname: String::from("List"),
                        motivation: String::from(
                            "\"for the development of asymmetric organocatalysis\"",
                        ),
                        share: String::from("2"),
                    },
                    Laureat {
                        id: String::from("1003"),
                        firstname: String::from("David"),
                        surname: String::from("MacMillan"),
                        motivation: String::from(
                            "\"for the development of asymmetric organocatalysis\"",
                        ),
                        share: String::from("2"),
                    },
                ],
            }],
        };
        assert_eq!(prizes, expected_prize);
    }
}
