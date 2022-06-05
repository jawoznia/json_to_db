use derive_getters::Getters;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, PartialEq, Getters)]
pub struct Prizes {
    prizes: Vec<Prize>,
}

#[derive(Debug, Deserialize, PartialEq, Getters)]
pub struct Prize {
    year: String,
    category: String,
    laureates: Option<Vec<Laureat>>,
}

#[derive(Debug, Deserialize, PartialEq, Getters)]
pub struct Laureat {
    id: String,
    firstname: String,
    surname: Option<String>,
    motivation: String,
    share: String,
}

impl Laureat {
    pub fn new(
        id: String,
        firstname: String,
        surname: Option<String>,
        motivation: String,
        share: String,
    ) -> Self {
        Self {
            id,
            firstname,
            surname,
            motivation,
            share,
        }
    }
}

pub fn load_data_from_json(path: &str) -> Result<Prizes, String> {
    match read_to_string(path) {
        Ok(data) => match serde_json::from_str::<Prizes>(data.as_str()) {
            Ok(prizes) => Ok(prizes),
            Err(e) => Err(format!("Error while parsing JSON: {:?}", e)),
        },
        Err(e) => Err(format!(
            "Error while reading file {} to string: {:?}",
            path, e
        )),
    }
}

fn read_to_string(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_loud_dummy_json() -> Result<(), String> {
        let prizes = load_data_from_json("data/single_prize.json")?;
        let expected_prize = Prizes {
            prizes: vec![Prize {
                year: String::from("2021"),
                category: String::from("chemistry"),
                laureates: Some(vec![
                    Laureat {
                        id: String::from("1002"),
                        firstname: String::from("Benjamin"),
                        surname: Some(String::from("List")),
                        motivation: String::from(
                            "\"for the development of asymmetric organocatalysis\"",
                        ),
                        share: String::from("2"),
                    },
                    Laureat {
                        id: String::from("1003"),
                        firstname: String::from("David"),
                        surname: Some(String::from("MacMillan")),
                        motivation: String::from(
                            "\"for the development of asymmetric organocatalysis\"",
                        ),
                        share: String::from("2"),
                    },
                ]),
            }],
        };
        assert_eq!(prizes, expected_prize);
        Ok(())
    }

    // #[test]
    // fn try_load_non_exisiting_file() {
    //     let prizes = load_data_from_json("data/i_dont_exist.json");
    //     assert_eq!(prizes, expected_prize);
    // }
}
