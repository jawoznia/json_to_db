use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Prizes {
    prizes: Vec<Prize>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Prize {
    year: String,
    category: String,
    laureates: Option<Vec<Laureat>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Laureat {
    id: String,
    firstname: String,
    surname: Option<String>,
    motivation: String,
    share: String,
}

pub fn load_json(path: &str) -> Prizes {
    let json = fs::read_to_string(path).expect("Unable to read file.");
    serde_json::from_str::<Prizes>(json.as_str()).expect("JSON was not well-formatted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_loud_dummy_json() {
        let prizes = load_json("data/single_prize.json");
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
    }
}
